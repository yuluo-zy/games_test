// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/systems/toolbar/src/toolbar_button.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: b8824750208f9a777246cc08bf22b726471a2686cde90e7390b52d64efb4d4a3
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: toolbar_button.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 4201
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use bevy_ecs; // namespace candidate
//   package identities: [{"name":"bevy_ecs","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.3"}]
// use bevy_ptr; // namespace candidate
//   package identities: [{"name":"bevy_ptr","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.3"}]
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
// 首条位置: {"module":4201,"record_offset":15972,"rva":43407648,"va":5412116768}
// 编译实例 RVAs: 0x2965920
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t4502069","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412116768,"raw_hex":"e835b24400","size":5,"soft_fail":false,"target_va":5416621408,"va":5412119334,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-4566","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412116768,"raw_hex":"e82aeeffff","size":5,"soft_fail":false,"target_va":5412112272,"va":5412116833,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t4501950","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412116768,"raw_hex":"e8beb14400","size":5,"soft_fail":false,"target_va":5416621376,"va":5412119421,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-28153380","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412116768,"raw_hex":"e8dc6952fe","size":5,"soft_fail":false,"target_va":5383963792,"va":5412117167,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::__RUST_STD_INTERNAL_VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-28144876","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412116768,"raw_hex":"e8148b52fe","size":5,"soft_fail":false,"target_va":5383972224,"va":5412117095,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::end_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-28154146","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412116768,"raw_hex":"e8de6652fe","size":5,"soft_fail":false,"target_va":5383964912,"va":5412119053,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-28153996","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412116768,"raw_hex":"e8746752fe","size":5,"soft_fail":false,"target_va":5383962816,"va":5412116807,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::thread_local::native::lazy::Storage$LT$T$C$D$GT$::get_or_init_slow
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t4537211","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412116768,"raw_hex":"e87b3b4500","size":5,"soft_fail":false,"target_va":5416656560,"va":5412119344,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t4499969","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412116768,"raw_hex":"e801aa4400","size":5,"soft_fail":false,"target_va":5416619360,"va":5412119386,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$58::deref_mut
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":4201,"record_offset":18776,"parent_record_offset":15972,"ancestor_record_offset":15972,"inlinee":9404,"source_line":434,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::message::iterators::impl$0::last
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/iterators.rs
//     evidence={"module":4201,"record_offset":22624,"parent_record_offset":15972,"ancestor_record_offset":15972,"inlinee":13153,"source_line":26,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::message::message_reader::MessageReader::read
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/message_reader.rs
//     evidence={"module":4201,"record_offset":22060,"parent_record_offset":15972,"ancestor_record_offset":15972,"inlinee":9339,"source_line":27,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::query::iter::impl$1::next
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs
//     evidence={"module":4201,"record_offset":18840,"parent_record_offset":15972,"ancestor_record_offset":15972,"inlinee":10317,"source_line":909,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::query::Query::iter_mut
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":4201,"record_offset":18344,"parent_record_offset":15972,"ancestor_record_offset":15972,"inlinee":18903,"source_line":704,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":4201,"record_offset":22028,"parent_record_offset":15972,"ancestor_record_offset":15972,"inlinee":5477,"source_line":991,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":4201,"record_offset":32544,"parent_record_offset":15972,"ancestor_record_offset":15972,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":4201,"record_offset":17536,"parent_record_offset":15972,"ancestor_record_offset":15972,"inlinee":8276,"source_line":589,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":4201,"record_offset":16144,"parent_record_offset":15972,"ancestor_record_offset":15972,"inlinee":9267,"source_line":626,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":4201,"record_offset":16524,"parent_record_offset":15972,"ancestor_record_offset":15972,"inlinee":8242,"source_line":659,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4201,"record_offset":18608,"parent_record_offset":18584,"ancestor_record_offset":15972,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4201,"record_offset":18656,"parent_record_offset":18632,"ancestor_record_offset":15972,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4201,"record_offset":18632,"parent_record_offset":18608,"ancestor_record_offset":15972,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":4201,"record_offset":18584,"parent_record_offset":18560,"ancestor_record_offset":15972,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":4201,"record_offset":18560,"parent_record_offset":18528,"ancestor_record_offset":15972,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":4201,"record_offset":19144,"parent_record_offset":19120,"ancestor_record_offset":15972,"inlinee":5735,"source_line":3021,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":4201,"record_offset":19168,"parent_record_offset":19144,"ancestor_record_offset":15972,"inlinee":4800,"source_line":2996,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":4201,"record_offset":18528,"parent_record_offset":18504,"ancestor_record_offset":15972,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::entities
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs
//     evidence={"module":4201,"record_offset":21432,"parent_record_offset":18872,"ancestor_record_offset":15972,"inlinee":10334,"source_line":484,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::is_empty
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs
//     evidence={"module":4201,"record_offset":20360,"parent_record_offset":18872,"ancestor_record_offset":15972,"inlinee":10331,"source_line":655,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::table_id
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs
//     evidence={"module":4201,"record_offset":20444,"parent_record_offset":18872,"ancestor_record_offset":15972,"inlinee":8568,"source_line":478,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::ArchetypeEntity::table_row
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs
//     evidence={"module":4201,"record_offset":21684,"parent_record_offset":18872,"ancestor_record_offset":15972,"inlinee":10337,"source_line":346,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetypes::get
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs
//     evidence={"module":4201,"record_offset":20276,"parent_record_offset":18872,"ancestor_record_offset":15972,"inlinee":10330,"source_line":864,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$57::set_changed
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":4201,"record_offset":18804,"parent_record_offset":18776,"ancestor_record_offset":15972,"inlinee":9399,"source_line":397,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::iterators::MessageIteratorWithId::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/iterators.rs
//     evidence={"module":4201,"record_offset":22172,"parent_record_offset":22140,"ancestor_record_offset":15972,"inlinee":9344,"source_line":54,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::iterators::impl$3::last
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/iterators.rs
//     evidence={"module":4201,"record_offset":22660,"parent_record_offset":22624,"ancestor_record_offset":15972,"inlinee":10306,"source_line":112,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::message_cursor::MessageCursor::read_with_id
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/message_cursor.rs
//     evidence={"module":4201,"record_offset":22140,"parent_record_offset":22092,"ancestor_record_offset":15972,"inlinee":9342,"source_line":89,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::StorageSwitch::extract
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":4201,"record_offset":20092,"parent_record_offset":20068,"ancestor_record_offset":15972,"inlinee":9590,"source_line":2964,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$48::set_archetype
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":4201,"record_offset":20560,"parent_record_offset":20536,"ancestor_record_offset":15972,"inlinee":11800,"source_line":1912,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$48::set_table
//     owner=registry_dependency_definition; count=14
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":4201,"record_offset":19228,"parent_record_offset":19204,"ancestor_record_offset":15972,"inlinee":11801,"source_line":1927,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$49::fetch
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":4201,"record_offset":20068,"parent_record_offset":20044,"ancestor_record_offset":15972,"inlinee":11822,"source_line":1987,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$49::fetch::closure$0
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":4201,"record_offset":20116,"parent_record_offset":20092,"ancestor_record_offset":15972,"inlinee":9577,"source_line":1994,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$89::fetch
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":4201,"record_offset":20044,"parent_record_offset":18872,"ancestor_record_offset":15972,"inlinee":10339,"source_line":2520,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIter::new
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs
//     evidence={"module":4201,"record_offset":18480,"parent_record_offset":18368,"ancestor_record_offset":15972,"inlinee":10316,"source_line":40,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::init
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs
//     evidence={"module":4201,"record_offset":18504,"parent_record_offset":18480,"ancestor_record_offset":15972,"inlinee":9124,"source_line":2408,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::next
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs
//     evidence={"module":4201,"record_offset":18872,"parent_record_offset":18840,"ancestor_record_offset":15972,"inlinee":10318,"source_line":2511,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$2::set_archetype
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/world_query.rs
//     evidence={"module":4201,"record_offset":20536,"parent_record_offset":18872,"ancestor_record_offset":15972,"inlinee":10332,"source_line":182,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$2::set_table
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/world_query.rs
//     evidence={"module":4201,"record_offset":19204,"parent_record_offset":18872,"ancestor_record_offset":15972,"inlinee":10321,"source_line":195,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_array::BlobArray::get_sub_slice
//     owner=registry_dependency_definition; count=14
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_array.rs
//     evidence={"module":4201,"record_offset":19572,"parent_record_offset":19548,"ancestor_record_offset":15972,"inlinee":10327,"source_line":143,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseArray::get
//     owner=registry_dependency_definition; count=14
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":4201,"record_offset":19308,"parent_record_offset":19284,"ancestor_record_offset":15972,"inlinee":8615,"source_line":56,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
//     owner=registry_dependency_definition; count=14
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":4201,"record_offset":19284,"parent_record_offset":19260,"ancestor_record_offset":15972,"inlinee":8626,"source_line":405,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$16::get::closure$0
//     owner=registry_dependency_definition; count=14
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":4201,"record_offset":19444,"parent_record_offset":19420,"ancestor_record_offset":15972,"inlinee":8628,"source_line":406,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_column
//     owner=registry_dependency_definition; count=14
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":4201,"record_offset":19260,"parent_record_offset":19228,"ancestor_record_offset":15972,"inlinee":8625,"source_line":489,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::is_empty
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":4201,"record_offset":19120,"parent_record_offset":18872,"ancestor_record_offset":15972,"inlinee":10320,"source_line":646,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::TableRow::index
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":4201,"record_offset":21808,"parent_record_offset":21784,"ancestor_record_offset":15972,"inlinee":8632,"source_line":114,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":4201,"record_offset":19036,"parent_record_offset":18872,"ancestor_record_offset":15972,"inlinee":10319,"source_line":752,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::column::ThinColumn::get_changed_ticks_slice
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/column.rs
//     evidence={"module":4201,"record_offset":19604,"parent_record_offset":19228,"ancestor_record_offset":15972,"inlinee":11819,"source_line":313,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::column::ThinColumn::get_data_slice
//     owner=registry_dependency_definition; count=14
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/column.rs
//     evidence={"module":4201,"record_offset":19548,"parent_record_offset":19228,"ancestor_record_offset":15972,"inlinee":10326,"source_line":297,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::thin_array_ptr::ThinArrayPtr::as_slice
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/thin_array_ptr.rs
//     evidence={"module":4201,"record_offset":19628,"parent_record_offset":19604,"ancestor_record_offset":15972,"inlinee":11820,"source_line":300,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::copy_unsafe
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":4201,"record_offset":18420,"parent_record_offset":18396,"ancestor_record_offset":15972,"inlinee":11851,"source_line":639,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":4201,"record_offset":18444,"parent_record_offset":18420,"ancestor_record_offset":15972,"inlinee":11852,"source_line":522,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::reborrow
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":4201,"record_offset":18368,"parent_record_offset":18344,"ancestor_record_offset":15972,"inlinee":18904,"source_line":604,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::reborrow_unsafe
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":4201,"record_offset":18396,"parent_record_offset":18368,"ancestor_record_offset":15972,"inlinee":11850,"source_line":621,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::impl$4::into_iter
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":4201,"record_offset":22848,"parent_record_offset":22824,"ancestor_record_offset":15972,"inlinee":10314,"source_line":2505,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::system_param::impl$14::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/system_param.rs
//     evidence={"module":4201,"record_offset":22092,"parent_record_offset":22060,"ancestor_record_offset":15972,"inlinee":9614,"source_line":1064,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ptr::ThinSlicePtr::get
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"bevy_ptr","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ptr-0.17.3/src/lib.rs
//     evidence={"module":4201,"record_offset":20140,"parent_record_offset":20116,"ancestor_record_offset":15972,"inlinee":9579,"source_line":1073,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":4201,"record_offset":17864,"parent_record_offset":17836,"ancestor_record_offset":15972,"inlinee":6507,"source_line":2061,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":4201,"record_offset":17892,"parent_record_offset":17864,"ancestor_record_offset":15972,"inlinee":4678,"source_line":510,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":4201,"record_offset":17812,"parent_record_offset":17784,"ancestor_record_offset":15972,"inlinee":6546,"source_line":1221,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":4201,"record_offset":17836,"parent_record_offset":17812,"ancestor_record_offset":15972,"inlinee":6505,"source_line":1255,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":4201,"record_offset":17952,"parent_record_offset":17836,"ancestor_record_offset":15972,"inlinee":5177,"source_line":2443,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$52::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":4201,"record_offset":18064,"parent_record_offset":18040,"ancestor_record_offset":15972,"inlinee":4995,"source_line":2052,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::last
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\chain.rs
//     evidence={"module":4201,"record_offset":22700,"parent_record_offset":22660,"ancestor_record_offset":15972,"inlinee":10307,"source_line":174,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":4201,"record_offset":17916,"parent_record_offset":17892,"ancestor_record_offset":15972,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":4201,"record_offset":22400,"parent_record_offset":22376,"ancestor_record_offset":15972,"inlinee":7937,"source_line":917,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":4201,"record_offset":22216,"parent_record_offset":22172,"ancestor_record_offset":15972,"inlinee":9345,"source_line":2344,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=16
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":4201,"record_offset":17684,"parent_record_offset":17644,"ancestor_record_offset":15972,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=16
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":4201,"record_offset":19332,"parent_record_offset":19308,"ancestor_record_offset":15972,"inlinee":5449,"source_line":1541,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":4201,"record_offset":19380,"parent_record_offset":19356,"ancestor_record_offset":15972,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":4201,"record_offset":19420,"parent_record_offset":19284,"ancestor_record_offset":15972,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$47::branch
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":4201,"record_offset":22788,"parent_record_offset":22660,"ancestor_record_offset":15972,"inlinee":5260,"source_line":2765,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":4201,"record_offset":16472,"parent_record_offset":16448,"ancestor_record_offset":15972,"inlinee":4656,"source_line":829,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":4201,"record_offset":18300,"parent_record_offset":17644,"ancestor_record_offset":15972,"inlinee":6443,"source_line":266,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":4201,"record_offset":18016,"parent_record_offset":17784,"ancestor_record_offset":15972,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":4201,"record_offset":18724,"parent_record_offset":18700,"ancestor_record_offset":15972,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":4201,"record_offset":16944,"parent_record_offset":16920,"ancestor_record_offset":15972,"inlinee":8268,"source_line":575,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":4201,"record_offset":16920,"parent_record_offset":16896,"ancestor_record_offset":15972,"inlinee":8267,"source_line":728,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":4201,"record_offset":17128,"parent_record_offset":17100,"ancestor_record_offset":15972,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$49::slice_contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":4201,"record_offset":17228,"parent_record_offset":17200,"ancestor_record_offset":15972,"inlinee":8260,"source_line":405,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":4201,"record_offset":17200,"parent_record_offset":17172,"ancestor_record_offset":15972,"inlinee":8258,"source_line":2589,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":4201,"record_offset":19060,"parent_record_offset":19036,"ancestor_record_offset":15972,"inlinee":4165,"source_line":572,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get_unchecked
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":4201,"record_offset":21628,"parent_record_offset":18872,"ancestor_record_offset":15972,"inlinee":4887,"source_line":639,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":4201,"record_offset":19084,"parent_record_offset":19060,"ancestor_record_offset":15972,"inlinee":4167,"source_line":218,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=18
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":4201,"record_offset":19500,"parent_record_offset":19468,"ancestor_record_offset":15972,"inlinee":4183,"source_line":239,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":4201,"record_offset":22376,"parent_record_offset":22352,"ancestor_record_offset":15972,"inlinee":8594,"source_line":366,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$7::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":4201,"record_offset":22352,"parent_record_offset":22328,"ancestor_record_offset":15972,"inlinee":8593,"source_line":545,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":4201,"record_offset":18700,"parent_record_offset":18528,"ancestor_record_offset":15972,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":4201,"record_offset":16872,"parent_record_offset":16848,"ancestor_record_offset":15972,"inlinee":8265,"source_line":81,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":4201,"record_offset":16896,"parent_record_offset":16872,"ancestor_record_offset":15972,"inlinee":8266,"source_line":119,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::any
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":4201,"record_offset":17256,"parent_record_offset":17228,"ancestor_record_offset":15972,"inlinee":8261,"source_line":326,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":4201,"record_offset":18972,"parent_record_offset":18872,"ancestor_record_offset":15972,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$172::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":4201,"record_offset":16848,"parent_record_offset":16732,"ancestor_record_offset":15972,"inlinee":8264,"source_line":438,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":4201,"record_offset":16272,"parent_record_offset":16248,"ancestor_record_offset":15972,"inlinee":9253,"source_line":377,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":4201,"record_offset":16560,"parent_record_offset":16524,"ancestor_record_offset":15972,"inlinee":8243,"source_line":1527,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":4201,"record_offset":16700,"parent_record_offset":16668,"ancestor_record_offset":15972,"inlinee":8253,"source_line":125,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":4201,"record_offset":16668,"parent_record_offset":16632,"ancestor_record_offset":15972,"inlinee":8251,"source_line":215,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":4201,"record_offset":16608,"parent_record_offset":16584,"ancestor_record_offset":15972,"inlinee":8247,"source_line":315,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":4201,"record_offset":16632,"parent_record_offset":16608,"ancestor_record_offset":15972,"inlinee":8249,"source_line":709,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":4201,"record_offset":16584,"parent_record_offset":16560,"ancestor_record_offset":15972,"inlinee":8245,"source_line":798,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":4201,"record_offset":17172,"parent_record_offset":16632,"ancestor_record_offset":15972,"inlinee":8257,"source_line":659,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":4201,"record_offset":16248,"parent_record_offset":16216,"ancestor_record_offset":15972,"inlinee":9270,"source_line":438,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":4201,"record_offset":16216,"parent_record_offset":16188,"ancestor_record_offset":15972,"inlinee":9269,"source_line":468,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":4201,"record_offset":16372,"parent_record_offset":16340,"ancestor_record_offset":15972,"inlinee":9252,"source_line":507,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":4201,"record_offset":16424,"parent_record_offset":16372,"ancestor_record_offset":15972,"inlinee":9254,"source_line":527,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":4201,"record_offset":16340,"parent_record_offset":16312,"ancestor_record_offset":15972,"inlinee":8274,"source_line":539,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":4201,"record_offset":16188,"parent_record_offset":16144,"ancestor_record_offset":15972,"inlinee":8272,"source_line":62,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":4201,"record_offset":16448,"parent_record_offset":16424,"ancestor_record_offset":15972,"inlinee":8315,"source_line":189,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":4201,"record_offset":16732,"parent_record_offset":16700,"ancestor_record_offset":15972,"inlinee":8255,"source_line":78,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":4201,"record_offset":17016,"parent_record_offset":16732,"ancestor_record_offset":15972,"inlinee":8270,"source_line":16,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":4201,"record_offset":16988,"parent_record_offset":16732,"ancestor_record_offset":15972,"inlinee":8269,"source_line":10,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":4201,"record_offset":16792,"parent_record_offset":16732,"ancestor_record_offset":15972,"inlinee":8262,"source_line":23,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxU32::get
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"nonmax","version":"0.5.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":4201,"record_offset":21832,"parent_record_offset":21808,"ancestor_record_offset":15972,"inlinee":8634,"source_line":153,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=14
//     packages=[{"name":"nonmax","version":"0.5.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":4201,"record_offset":19468,"parent_record_offset":19444,"ancestor_record_offset":15972,"inlinee":4193,"source_line":153,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":4201,"record_offset":17572,"parent_record_offset":17536,"ancestor_record_offset":15972,"inlinee":8278,"source_line":425,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":4201,"record_offset":32608,"parent_record_offset":32576,"ancestor_record_offset":15972,"inlinee":8283,"source_line":601,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":4201,"record_offset":33060,"parent_record_offset":32856,"ancestor_record_offset":15972,"inlinee":6607,"source_line":602,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":4201,"record_offset":17712,"parent_record_offset":17684,"ancestor_record_offset":15972,"inlinee":8280,"source_line":92,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":4201,"record_offset":17784,"parent_record_offset":17644,"ancestor_record_offset":15972,"inlinee":6545,"source_line":429,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":4201,"record_offset":17988,"parent_record_offset":17784,"ancestor_record_offset":15972,"inlinee":6548,"source_line":591,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":4201,"record_offset":17740,"parent_record_offset":17712,"ancestor_record_offset":15972,"inlinee":5707,"source_line":58,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":4201,"record_offset":17644,"parent_record_offset":17608,"ancestor_record_offset":15972,"inlinee":5703,"source_line":508,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":4201,"record_offset":17608,"parent_record_offset":17572,"ancestor_record_offset":15972,"inlinee":5702,"source_line":473,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::toolbar_button::toggle_building_material_button
// 记录形式: compiled_procedure; instances=1
// 源码行: 75
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":4201,"record_offset":41164,"rva":43412336,"va":5412121456}
// 编译实例 RVAs: 0x2966b70
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t-36443644","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412121456,"raw_hex":"e804ead3fd","size":5,"soft_fail":false,"target_va":5375678688,"va":5412122327,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t-36443693","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412121456,"raw_hex":"e8d3e9d3fd","size":5,"soft_fail":false,"target_va":5375678704,"va":5412122392,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=3
//     target_aliases=1699 candidates; complete list: toolbar_button.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-43400127","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412121456,"raw_hex":"e841c469fd","size":5,"soft_fail":false,"target_va":5368722192,"va":5412122314,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6fd10
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: toolbar_button.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4231042","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412121456,"raw_hex":"e8828f4000","size":5,"soft_fail":false,"target_va":5416353040,"va":5412121993,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb10
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: toolbar_button.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4934739","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412121456,"raw_hex":"e8534c4b00","size":5,"soft_fail":false,"target_va":5417057040,"va":5412122296,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: toolbar_button.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4934696","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412121456,"raw_hex":"e8284c4b00","size":5,"soft_fail":false,"target_va":5417057056,"va":5412122355,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t4497016","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412121456,"raw_hex":"e8789e4400","size":5,"soft_fail":false,"target_va":5416620794,"va":5412123773,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::string::impl$93::ne
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":4201,"record_offset":45712,"parent_record_offset":41164,"ancestor_record_offset":41164,"inlinee":18157,"source_line":2670,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":4201,"record_offset":42808,"parent_record_offset":41164,"ancestor_record_offset":41164,"inlinee":7222,"source_line":434,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$58::deref_mut
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":4201,"record_offset":46596,"parent_record_offset":41164,"ancestor_record_offset":41164,"inlinee":9404,"source_line":434,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::message::iterators::impl$0::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/iterators.rs
//     evidence={"module":4201,"record_offset":42040,"parent_record_offset":41164,"ancestor_record_offset":41164,"inlinee":11533,"source_line":14,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::message::message_reader::MessageReader::read
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/message_reader.rs
//     evidence={"module":4201,"record_offset":41344,"parent_record_offset":41164,"ancestor_record_offset":41164,"inlinee":9339,"source_line":27,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::query::iter::impl$1::next
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs
//     evidence={"module":4201,"record_offset":43588,"parent_record_offset":41164,"ancestor_record_offset":41164,"inlinee":10317,"source_line":909,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::commands::EntityCommands::insert
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/commands/mod.rs
//     evidence={"module":4201,"record_offset":42864,"parent_record_offset":41164,"ancestor_record_offset":41164,"inlinee":11388,"source_line":1390,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::query::Query::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":4201,"record_offset":42324,"parent_record_offset":41164,"ancestor_record_offset":41164,"inlinee":11704,"source_line":1388,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::query::Query::iter_mut
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":4201,"record_offset":43156,"parent_record_offset":41164,"ancestor_record_offset":41164,"inlinee":18903,"source_line":704,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":4201,"record_offset":45948,"parent_record_offset":41164,"ancestor_record_offset":41164,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":4201,"record_offset":46660,"parent_record_offset":41164,"ancestor_record_offset":41164,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4201,"record_offset":46388,"parent_record_offset":46360,"ancestor_record_offset":41164,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4201,"record_offset":46416,"parent_record_offset":46388,"ancestor_record_offset":41164,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4201,"record_offset":46868,"parent_record_offset":46840,"ancestor_record_offset":41164,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4201,"record_offset":46896,"parent_record_offset":46868,"ancestor_record_offset":41164,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4201,"record_offset":46444,"parent_record_offset":46416,"ancestor_record_offset":41164,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4201,"record_offset":46924,"parent_record_offset":46896,"ancestor_record_offset":41164,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4201,"record_offset":46360,"parent_record_offset":46320,"ancestor_record_offset":41164,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4201,"record_offset":46840,"parent_record_offset":46772,"ancestor_record_offset":41164,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4201,"record_offset":43420,"parent_record_offset":43396,"ancestor_record_offset":41164,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4201,"record_offset":43004,"parent_record_offset":42976,"ancestor_record_offset":41164,"inlinee":5066,"source_line":338,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4201,"record_offset":46248,"parent_record_offset":46212,"ancestor_record_offset":41164,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4201,"record_offset":43088,"parent_record_offset":43064,"ancestor_record_offset":41164,"inlinee":4788,"source_line":616,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4201,"record_offset":46808,"parent_record_offset":46772,"ancestor_record_offset":41164,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4201,"record_offset":46772,"parent_record_offset":46744,"ancestor_record_offset":41164,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4201,"record_offset":43064,"parent_record_offset":43032,"ancestor_record_offset":41164,"inlinee":5068,"source_line":764,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4201,"record_offset":43468,"parent_record_offset":43444,"ancestor_record_offset":41164,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4201,"record_offset":43444,"parent_record_offset":43420,"ancestor_record_offset":41164,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4201,"record_offset":43032,"parent_record_offset":43004,"ancestor_record_offset":41164,"inlinee":5067,"source_line":653,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4201,"record_offset":46320,"parent_record_offset":46284,"ancestor_record_offset":41164,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4201,"record_offset":46284,"parent_record_offset":46248,"ancestor_record_offset":41164,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4201,"record_offset":46744,"parent_record_offset":46716,"ancestor_record_offset":41164,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":4201,"record_offset":46092,"parent_record_offset":46056,"ancestor_record_offset":41164,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":4201,"record_offset":46128,"parent_record_offset":46092,"ancestor_record_offset":41164,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":4201,"record_offset":46164,"parent_record_offset":46128,"ancestor_record_offset":41164,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":4201,"record_offset":46056,"parent_record_offset":46020,"ancestor_record_offset":41164,"inlinee":4447,"source_line":841,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":4201,"record_offset":46020,"parent_record_offset":45984,"ancestor_record_offset":41164,"inlinee":4445,"source_line":210,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$48::from
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":4201,"record_offset":45984,"parent_record_offset":45948,"ancestor_record_offset":41164,"inlinee":4520,"source_line":3113,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":4201,"record_offset":43396,"parent_record_offset":43372,"ancestor_record_offset":41164,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":4201,"record_offset":41684,"parent_record_offset":41660,"ancestor_record_offset":41164,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":4201,"record_offset":43888,"parent_record_offset":43864,"ancestor_record_offset":41164,"inlinee":5735,"source_line":3021,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":4201,"record_offset":43912,"parent_record_offset":43888,"ancestor_record_offset":41164,"inlinee":4800,"source_line":2996,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":4201,"record_offset":42976,"parent_record_offset":42948,"ancestor_record_offset":41164,"inlinee":5090,"source_line":1445,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":4201,"record_offset":46212,"parent_record_offset":46164,"ancestor_record_offset":41164,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":4201,"record_offset":41660,"parent_record_offset":41432,"ancestor_record_offset":41164,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::entities
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs
//     evidence={"module":4201,"record_offset":45508,"parent_record_offset":43624,"ancestor_record_offset":41164,"inlinee":10334,"source_line":484,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::is_empty
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs
//     evidence={"module":4201,"record_offset":44760,"parent_record_offset":43624,"ancestor_record_offset":41164,"inlinee":10331,"source_line":655,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::table_id
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs
//     evidence={"module":4201,"record_offset":44908,"parent_record_offset":43624,"ancestor_record_offset":41164,"inlinee":8568,"source_line":478,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::ArchetypeEntity::table_row
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs
//     evidence={"module":4201,"record_offset":50472,"parent_record_offset":47448,"ancestor_record_offset":41164,"inlinee":10337,"source_line":346,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetypes::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs
//     evidence={"module":4201,"record_offset":44676,"parent_record_offset":43624,"ancestor_record_offset":41164,"inlinee":10330,"source_line":864,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":4201,"record_offset":42832,"parent_record_offset":42808,"ancestor_record_offset":41164,"inlinee":7224,"source_line":397,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$57::set_changed
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":4201,"record_offset":46624,"parent_record_offset":46596,"ancestor_record_offset":41164,"inlinee":9399,"source_line":397,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::Entities::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/entity/mod.rs
//     evidence={"module":4201,"record_offset":42384,"parent_record_offset":42348,"ancestor_record_offset":41164,"inlinee":8602,"source_line":948,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::Entities::get_id_location
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/entity/mod.rs
//     evidence={"module":4201,"record_offset":42408,"parent_record_offset":42384,"ancestor_record_offset":41164,"inlinee":8603,"source_line":955,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::impl$18::get_id_location::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/entity/mod.rs
//     evidence={"module":4201,"record_offset":42524,"parent_record_offset":42492,"ancestor_record_offset":41164,"inlinee":8620,"source_line":958,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::impl$37::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/entity/mod.rs
//     evidence={"module":4201,"record_offset":42548,"parent_record_offset":42524,"ancestor_record_offset":41164,"inlinee":8622,"source_line":214,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::iterators::MessageIteratorWithId::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/iterators.rs
//     evidence={"module":4201,"record_offset":41432,"parent_record_offset":41408,"ancestor_record_offset":41164,"inlinee":9344,"source_line":54,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::iterators::impl$3::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/iterators.rs
//     evidence={"module":4201,"record_offset":42064,"parent_record_offset":42040,"ancestor_record_offset":41164,"inlinee":11534,"source_line":86,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::message_cursor::MessageCursor::read_with_id
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/message_cursor.rs
//     evidence={"module":4201,"record_offset":41408,"parent_record_offset":41368,"ancestor_record_offset":41164,"inlinee":9342,"source_line":89,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::StorageSwitch::extract
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":4201,"record_offset":50548,"parent_record_offset":50524,"ancestor_record_offset":41164,"inlinee":9590,"source_line":2964,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::set_archetype
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":4201,"record_offset":49160,"parent_record_offset":49128,"ancestor_record_offset":41164,"inlinee":10333,"source_line":1530,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::set_table
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":4201,"record_offset":48044,"parent_record_offset":48012,"ancestor_record_offset":41164,"inlinee":10322,"source_line":1545,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$37::fetch
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":4201,"record_offset":50524,"parent_record_offset":50500,"ancestor_record_offset":41164,"inlinee":10341,"source_line":1598,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$37::fetch::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":4201,"record_offset":50572,"parent_record_offset":50548,"ancestor_record_offset":41164,"inlinee":9575,"source_line":1605,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$48::set_archetype
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":4201,"record_offset":45024,"parent_record_offset":45000,"ancestor_record_offset":41164,"inlinee":11800,"source_line":1912,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$48::set_table
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":4201,"record_offset":43976,"parent_record_offset":43948,"ancestor_record_offset":41164,"inlinee":11801,"source_line":1927,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$49::fetch::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":4201,"record_offset":50856,"parent_record_offset":50832,"ancestor_record_offset":41164,"inlinee":9577,"source_line":1994,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$89::fetch
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":4201,"record_offset":50500,"parent_record_offset":47448,"ancestor_record_offset":41164,"inlinee":10339,"source_line":2520,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIter::new
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs
//     evidence={"module":4201,"record_offset":43292,"parent_record_offset":43180,"ancestor_record_offset":41164,"inlinee":10316,"source_line":40,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::init
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs
//     evidence={"module":4201,"record_offset":43316,"parent_record_offset":43292,"ancestor_record_offset":41164,"inlinee":9124,"source_line":2408,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::next
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs
//     evidence={"module":4201,"record_offset":43624,"parent_record_offset":43588,"ancestor_record_offset":41164,"inlinee":10318,"source_line":2511,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$2::set_archetype
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/world_query.rs
//     evidence={"module":4201,"record_offset":49128,"parent_record_offset":47448,"ancestor_record_offset":41164,"inlinee":10332,"source_line":182,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$2::set_table
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/world_query.rs
//     evidence={"module":4201,"record_offset":48012,"parent_record_offset":47448,"ancestor_record_offset":41164,"inlinee":10321,"source_line":195,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$3::set_archetype
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/world_query.rs
//     evidence={"module":4201,"record_offset":45000,"parent_record_offset":43624,"ancestor_record_offset":41164,"inlinee":10710,"source_line":182,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$3::set_table
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/world_query.rs
//     evidence={"module":4201,"record_offset":43948,"parent_record_offset":43624,"ancestor_record_offset":41164,"inlinee":10708,"source_line":195,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_array::BlobArray::get_sub_slice
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_array.rs
//     evidence={"module":4201,"record_offset":44320,"parent_record_offset":44296,"ancestor_record_offset":41164,"inlinee":10327,"source_line":143,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseArray::get
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":4201,"record_offset":44056,"parent_record_offset":44032,"ancestor_record_offset":41164,"inlinee":8615,"source_line":56,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":4201,"record_offset":44032,"parent_record_offset":44008,"ancestor_record_offset":41164,"inlinee":8626,"source_line":405,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$16::get::closure$0
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":4201,"record_offset":44192,"parent_record_offset":44168,"ancestor_record_offset":41164,"inlinee":8628,"source_line":406,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::entities
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":4201,"record_offset":44416,"parent_record_offset":43624,"ancestor_record_offset":41164,"inlinee":10328,"source_line":208,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_column
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":4201,"record_offset":44008,"parent_record_offset":43976,"ancestor_record_offset":41164,"inlinee":8625,"source_line":489,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_data_slice_for
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":4201,"record_offset":48072,"parent_record_offset":48044,"ancestor_record_offset":41164,"inlinee":10323,"source_line":377,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::is_empty
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":4201,"record_offset":43864,"parent_record_offset":43624,"ancestor_record_offset":41164,"inlinee":10320,"source_line":646,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::TableRow::index
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":4201,"record_offset":50596,"parent_record_offset":50572,"ancestor_record_offset":41164,"inlinee":8632,"source_line":114,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":4201,"record_offset":43780,"parent_record_offset":43624,"ancestor_record_offset":41164,"inlinee":10319,"source_line":752,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::column::ThinColumn::get_changed_ticks_slice
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/column.rs
//     evidence={"module":4201,"record_offset":44352,"parent_record_offset":43976,"ancestor_record_offset":41164,"inlinee":11819,"source_line":313,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::column::ThinColumn::get_data_slice
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/column.rs
//     evidence={"module":4201,"record_offset":44296,"parent_record_offset":43976,"ancestor_record_offset":41164,"inlinee":10326,"source_line":297,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::impl$4::get_data_slice_for::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":4201,"record_offset":48476,"parent_record_offset":48452,"ancestor_record_offset":41164,"inlinee":10325,"source_line":382,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::thin_array_ptr::ThinArrayPtr::as_slice
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/thin_array_ptr.rs
//     evidence={"module":4201,"record_offset":44376,"parent_record_offset":44352,"ancestor_record_offset":41164,"inlinee":11820,"source_line":300,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::queue
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/commands/mod.rs
//     evidence={"module":4201,"record_offset":42920,"parent_record_offset":42892,"ancestor_record_offset":41164,"inlinee":8582,"source_line":588,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::queue_internal
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/commands/mod.rs
//     evidence={"module":4201,"record_offset":42948,"parent_record_offset":42920,"ancestor_record_offset":41164,"inlinee":8583,"source_line":652,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::EntityCommands::queue
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/commands/mod.rs
//     evidence={"module":4201,"record_offset":42892,"parent_record_offset":42864,"ancestor_record_offset":41164,"inlinee":11368,"source_line":1889,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::copy_unsafe
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":4201,"record_offset":43232,"parent_record_offset":43208,"ancestor_record_offset":41164,"inlinee":11851,"source_line":639,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::get_inner
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":4201,"record_offset":42348,"parent_record_offset":42324,"ancestor_record_offset":41164,"inlinee":11705,"source_line":1539,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::new
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":4201,"record_offset":43256,"parent_record_offset":43232,"ancestor_record_offset":41164,"inlinee":11852,"source_line":522,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::reborrow
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":4201,"record_offset":43180,"parent_record_offset":43156,"ancestor_record_offset":41164,"inlinee":18904,"source_line":604,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::reborrow_unsafe
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":4201,"record_offset":43208,"parent_record_offset":43180,"ancestor_record_offset":41164,"inlinee":11850,"source_line":621,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::system_param::impl$14::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/system_param.rs
//     evidence={"module":4201,"record_offset":41368,"parent_record_offset":41344,"ancestor_record_offset":41164,"inlinee":9614,"source_line":1064,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ptr::ThinSlicePtr::get
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ptr","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ptr-0.17.3/src/lib.rs
//     evidence={"module":4201,"record_offset":50768,"parent_record_offset":50744,"ancestor_record_offset":41164,"inlinee":9579,"source_line":1073,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] core::bool::impl$0::then
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\bool.rs
//     evidence={"module":4201,"record_offset":42648,"parent_record_offset":42620,"ancestor_record_offset":41164,"inlinee":4276,"source_line":65,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":4201,"record_offset":45748,"parent_record_offset":45712,"ancestor_record_offset":41164,"inlinee":4487,"source_line":264,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":4201,"record_offset":45820,"parent_record_offset":45784,"ancestor_record_offset":41164,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::and_then_or_clear
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\chain.rs
//     evidence={"module":4201,"record_offset":42116,"parent_record_offset":42092,"ancestor_record_offset":41164,"inlinee":7044,"source_line":331,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\chain.rs
//     evidence={"module":4201,"record_offset":42092,"parent_record_offset":42064,"ancestor_record_offset":41164,"inlinee":7042,"source_line":81,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\chain.rs
//     evidence={"module":4201,"record_offset":42196,"parent_record_offset":42172,"ancestor_record_offset":41164,"inlinee":7430,"source_line":82,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":4201,"record_offset":41616,"parent_record_offset":41588,"ancestor_record_offset":41164,"inlinee":7937,"source_line":917,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":4201,"record_offset":41476,"parent_record_offset":41432,"ancestor_record_offset":41164,"inlinee":9345,"source_line":2344,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":4201,"record_offset":44104,"parent_record_offset":44080,"ancestor_record_offset":41164,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":4201,"record_offset":44080,"parent_record_offset":44056,"ancestor_record_offset":41164,"inlinee":5449,"source_line":1541,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":4201,"record_offset":42140,"parent_record_offset":42116,"ancestor_record_offset":41164,"inlinee":4102,"source_line":766,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":4201,"record_offset":44128,"parent_record_offset":44104,"ancestor_record_offset":41164,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::filter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":4201,"record_offset":42492,"parent_record_offset":42408,"ancestor_record_offset":41164,"inlinee":8618,"source_line":1578,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":4201,"record_offset":44168,"parent_record_offset":44032,"ancestor_record_offset":41164,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::ok_or
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":4201,"record_offset":42592,"parent_record_offset":42348,"ancestor_record_offset":41164,"inlinee":5838,"source_line":1337,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":4201,"record_offset":42172,"parent_record_offset":42092,"ancestor_record_offset":41164,"inlinee":7428,"source_line":1647,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":4201,"record_offset":46504,"parent_record_offset":46164,"ancestor_record_offset":41164,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":4201,"record_offset":46532,"parent_record_offset":46504,"ancestor_record_offset":41164,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":4201,"record_offset":46688,"parent_record_offset":46660,"ancestor_record_offset":41164,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":4201,"record_offset":41932,"parent_record_offset":41908,"ancestor_record_offset":41164,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":4201,"record_offset":42272,"parent_record_offset":42248,"ancestor_record_offset":41164,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":4201,"record_offset":43744,"parent_record_offset":43716,"ancestor_record_offset":41164,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":4201,"record_offset":45856,"parent_record_offset":45820,"ancestor_record_offset":41164,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":4201,"record_offset":45896,"parent_record_offset":45856,"ancestor_record_offset":41164,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":4201,"record_offset":41532,"parent_record_offset":41432,"ancestor_record_offset":41164,"inlinee":4165,"source_line":572,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":4201,"record_offset":44844,"parent_record_offset":43624,"ancestor_record_offset":41164,"inlinee":4887,"source_line":639,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":4201,"record_offset":41884,"parent_record_offset":41432,"ancestor_record_offset":41164,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":4201,"record_offset":42460,"parent_record_offset":42436,"ancestor_record_offset":41164,"inlinee":4167,"source_line":218,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":4201,"record_offset":44248,"parent_record_offset":44216,"ancestor_record_offset":41164,"inlinee":4183,"source_line":239,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":4201,"record_offset":41588,"parent_record_offset":41560,"ancestor_record_offset":41164,"inlinee":8594,"source_line":366,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$7::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":4201,"record_offset":41560,"parent_record_offset":41532,"ancestor_record_offset":41164,"inlinee":8593,"source_line":545,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":4201,"record_offset":41908,"parent_record_offset":41884,"ancestor_record_offset":41164,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":4201,"record_offset":42248,"parent_record_offset":42196,"ancestor_record_offset":41164,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":4201,"record_offset":45784,"parent_record_offset":45748,"ancestor_record_offset":41164,"inlinee":5443,"source_line":29,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::contains
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":4201,"record_offset":42620,"parent_record_offset":42348,"ancestor_record_offset":41164,"inlinee":4274,"source_line":392,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::contains_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":4201,"record_offset":42696,"parent_record_offset":42672,"ancestor_record_offset":41164,"inlinee":4279,"source_line":408,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::get_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":4201,"record_offset":42720,"parent_record_offset":42696,"ancestor_record_offset":41164,"inlinee":4280,"source_line":171,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::impl$2::contains::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":4201,"record_offset":42672,"parent_record_offset":42648,"ancestor_record_offset":41164,"inlinee":4278,"source_line":395,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxU32::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nonmax","version":"0.5.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":4201,"record_offset":50620,"parent_record_offset":50596,"ancestor_record_offset":41164,"inlinee":8634,"source_line":153,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"nonmax","version":"0.5.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":4201,"record_offset":44216,"parent_record_offset":44192,"ancestor_record_offset":41164,"inlinee":4193,"source_line":153,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::toolbar_button::switch_app_mode_button
// 记录形式: compiled_procedure; instances=1
// 源码行: 129
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":4201,"record_offset":7988,"rva":43405744,"va":5412114864}
// 编译实例 RVAs: 0x29651b0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f2b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: toolbar_button.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4234945","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412114864,"raw_hex":"e8c19e4000","size":5,"soft_fail":false,"target_va":5416350384,"va":5412115434,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6fd10
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: toolbar_button.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4236863","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412114864,"raw_hex":"e83fa64000","size":5,"soft_fail":false,"target_va":5416353040,"va":5412116172,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::mode_manager::impl$58::eq
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t7779","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412114864,"raw_hex":"e8631e0000","size":5,"soft_fail":false,"target_va":5412123792,"va":5412116008,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$58::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":4201,"record_offset":15524,"parent_record_offset":7988,"ancestor_record_offset":7988,"inlinee":9404,"source_line":434,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::message::iterators::impl$0::last
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/iterators.rs
//     evidence={"module":4201,"record_offset":11768,"parent_record_offset":7988,"ancestor_record_offset":7988,"inlinee":13153,"source_line":26,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::message::iterators::impl$0::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/iterators.rs
//     evidence={"module":4201,"record_offset":9248,"parent_record_offset":7988,"ancestor_record_offset":7988,"inlinee":11533,"source_line":14,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::message::message_reader::MessageReader::read
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/message_reader.rs
//     evidence={"module":4201,"record_offset":8152,"parent_record_offset":7988,"ancestor_record_offset":7988,"inlinee":9339,"source_line":27,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::message::message_writer::MessageWriter::write
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/message_writer.rs
//     evidence={"module":4201,"record_offset":8860,"parent_record_offset":7988,"ancestor_record_offset":7988,"inlinee":11206,"source_line":69,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::query::iter::impl$1::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs
//     evidence={"module":4201,"record_offset":12276,"parent_record_offset":7988,"ancestor_record_offset":7988,"inlinee":10317,"source_line":909,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::commands::EntityCommands::insert
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/commands/mod.rs
//     evidence={"module":4201,"record_offset":15588,"parent_record_offset":7988,"ancestor_record_offset":7988,"inlinee":11388,"source_line":1390,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::query::Query::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":4201,"record_offset":9532,"parent_record_offset":7988,"ancestor_record_offset":7988,"inlinee":11704,"source_line":1388,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::query::Query::iter_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":4201,"record_offset":11996,"parent_record_offset":7988,"ancestor_record_offset":7988,"inlinee":18903,"source_line":704,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4201,"record_offset":9084,"parent_record_offset":9052,"ancestor_record_offset":7988,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4201,"record_offset":15772,"parent_record_offset":15736,"ancestor_record_offset":7988,"inlinee":5066,"source_line":338,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4201,"record_offset":15880,"parent_record_offset":15852,"ancestor_record_offset":7988,"inlinee":4788,"source_line":616,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4201,"record_offset":15852,"parent_record_offset":15808,"ancestor_record_offset":7988,"inlinee":5068,"source_line":764,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4201,"record_offset":9132,"parent_record_offset":9108,"ancestor_record_offset":7988,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4201,"record_offset":9108,"parent_record_offset":9084,"ancestor_record_offset":7988,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4201,"record_offset":15808,"parent_record_offset":15772,"ancestor_record_offset":7988,"inlinee":5067,"source_line":653,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":4201,"record_offset":9052,"parent_record_offset":9008,"ancestor_record_offset":7988,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":4201,"record_offset":10108,"parent_record_offset":10084,"ancestor_record_offset":7988,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":4201,"record_offset":8488,"parent_record_offset":8464,"ancestor_record_offset":7988,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":4201,"record_offset":12568,"parent_record_offset":12544,"ancestor_record_offset":7988,"inlinee":5735,"source_line":3021,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":4201,"record_offset":12592,"parent_record_offset":12568,"ancestor_record_offset":7988,"inlinee":4800,"source_line":2996,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":4201,"record_offset":8976,"parent_record_offset":8932,"ancestor_record_offset":7988,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":4201,"record_offset":9008,"parent_record_offset":8976,"ancestor_record_offset":7988,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":4201,"record_offset":15736,"parent_record_offset":15696,"ancestor_record_offset":7988,"inlinee":5090,"source_line":1445,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":4201,"record_offset":8464,"parent_record_offset":8240,"ancestor_record_offset":7988,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::entities
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs
//     evidence={"module":4201,"record_offset":15320,"parent_record_offset":12312,"ancestor_record_offset":7988,"inlinee":10334,"source_line":484,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::is_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs
//     evidence={"module":4201,"record_offset":13992,"parent_record_offset":12312,"ancestor_record_offset":7988,"inlinee":10331,"source_line":655,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::table_id
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs
//     evidence={"module":4201,"record_offset":14140,"parent_record_offset":12312,"ancestor_record_offset":7988,"inlinee":8568,"source_line":478,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::ArchetypeId::index
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs
//     evidence={"module":4201,"record_offset":9840,"parent_record_offset":9556,"ancestor_record_offset":7988,"inlinee":8565,"source_line":122,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetypes::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs
//     evidence={"module":4201,"record_offset":13908,"parent_record_offset":12312,"ancestor_record_offset":7988,"inlinee":10330,"source_line":864,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":4201,"record_offset":9212,"parent_record_offset":8888,"ancestor_record_offset":7988,"inlinee":7224,"source_line":397,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$57::set_changed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":4201,"record_offset":15552,"parent_record_offset":15524,"ancestor_record_offset":7988,"inlinee":9399,"source_line":397,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::Entities::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/entity/mod.rs
//     evidence={"module":4201,"record_offset":9604,"parent_record_offset":9556,"ancestor_record_offset":7988,"inlinee":8602,"source_line":948,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::Entities::get_id_location
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/entity/mod.rs
//     evidence={"module":4201,"record_offset":9628,"parent_record_offset":9604,"ancestor_record_offset":7988,"inlinee":8603,"source_line":955,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::impl$18::get_id_location::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/entity/mod.rs
//     evidence={"module":4201,"record_offset":9744,"parent_record_offset":9712,"ancestor_record_offset":7988,"inlinee":8620,"source_line":958,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::impl$37::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/entity/mod.rs
//     evidence={"module":4201,"record_offset":9768,"parent_record_offset":9744,"ancestor_record_offset":7988,"inlinee":8622,"source_line":214,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::iterators::MessageIteratorWithId::new
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/iterators.rs
//     evidence={"module":4201,"record_offset":8240,"parent_record_offset":8216,"ancestor_record_offset":7988,"inlinee":9344,"source_line":54,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::iterators::impl$3::last
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/iterators.rs
//     evidence={"module":4201,"record_offset":11800,"parent_record_offset":11768,"ancestor_record_offset":7988,"inlinee":10306,"source_line":112,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::iterators::impl$3::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/iterators.rs
//     evidence={"module":4201,"record_offset":9272,"parent_record_offset":9248,"ancestor_record_offset":7988,"inlinee":11534,"source_line":86,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::message_cursor::MessageCursor::read_with_id
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/message_cursor.rs
//     evidence={"module":4201,"record_offset":8216,"parent_record_offset":8176,"ancestor_record_offset":7988,"inlinee":9342,"source_line":89,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::messages::Messages::write
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/messages.rs
//     evidence={"module":4201,"record_offset":8888,"parent_record_offset":8860,"ancestor_record_offset":7988,"inlinee":8680,"source_line":125,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::messages::Messages::write_with_caller
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/messages.rs
//     evidence={"module":4201,"record_offset":8932,"parent_record_offset":8888,"ancestor_record_offset":7988,"inlinee":8681,"source_line":129,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::StorageSwitch::extract
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":4201,"record_offset":10824,"parent_record_offset":10800,"ancestor_record_offset":7988,"inlinee":9590,"source_line":2964,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::set_archetype
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":4201,"record_offset":10284,"parent_record_offset":10260,"ancestor_record_offset":7988,"inlinee":10333,"source_line":1530,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::set_table
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":4201,"record_offset":10308,"parent_record_offset":10284,"ancestor_record_offset":7988,"inlinee":10322,"source_line":1545,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$37::fetch
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":4201,"record_offset":10800,"parent_record_offset":10776,"ancestor_record_offset":7988,"inlinee":10341,"source_line":1598,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$37::fetch::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":4201,"record_offset":10848,"parent_record_offset":10824,"ancestor_record_offset":7988,"inlinee":9575,"source_line":1605,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$48::set_archetype
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":4201,"record_offset":14820,"parent_record_offset":14232,"ancestor_record_offset":7988,"inlinee":11800,"source_line":1912,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$48::set_table
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":4201,"record_offset":13184,"parent_record_offset":12628,"ancestor_record_offset":7988,"inlinee":11801,"source_line":1927,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$92::fetch
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":4201,"record_offset":10776,"parent_record_offset":9556,"ancestor_record_offset":7988,"inlinee":11825,"source_line":2520,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIter::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs
//     evidence={"module":4201,"record_offset":12044,"parent_record_offset":12020,"ancestor_record_offset":7988,"inlinee":10316,"source_line":40,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::init
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs
//     evidence={"module":4201,"record_offset":12068,"parent_record_offset":12044,"ancestor_record_offset":7988,"inlinee":9124,"source_line":2408,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs
//     evidence={"module":4201,"record_offset":12312,"parent_record_offset":12276,"ancestor_record_offset":7988,"inlinee":10318,"source_line":2511,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$3::set_archetype
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/world_query.rs
//     evidence={"module":4201,"record_offset":10260,"parent_record_offset":9556,"ancestor_record_offset":7988,"inlinee":10710,"source_line":182,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$3::set_table
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/world_query.rs
//     evidence={"module":4201,"record_offset":12628,"parent_record_offset":12312,"ancestor_record_offset":7988,"inlinee":10708,"source_line":195,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_array::BlobArray::get_sub_slice
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_array.rs
//     evidence={"module":4201,"record_offset":10720,"parent_record_offset":10696,"ancestor_record_offset":7988,"inlinee":10327,"source_line":143,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseArray::get
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":4201,"record_offset":10408,"parent_record_offset":10384,"ancestor_record_offset":7988,"inlinee":8615,"source_line":56,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":4201,"record_offset":10384,"parent_record_offset":10360,"ancestor_record_offset":7988,"inlinee":8626,"source_line":405,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$16::get::closure$0
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":4201,"record_offset":10544,"parent_record_offset":10520,"ancestor_record_offset":7988,"inlinee":8628,"source_line":406,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::entities
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":4201,"record_offset":13648,"parent_record_offset":12312,"ancestor_record_offset":7988,"inlinee":10328,"source_line":208,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_column
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":4201,"record_offset":10360,"parent_record_offset":10332,"ancestor_record_offset":7988,"inlinee":8625,"source_line":489,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_data_slice_for
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":4201,"record_offset":10332,"parent_record_offset":10308,"ancestor_record_offset":7988,"inlinee":10323,"source_line":377,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::is_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":4201,"record_offset":12544,"parent_record_offset":12312,"ancestor_record_offset":7988,"inlinee":10320,"source_line":646,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::TableRow::index
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":4201,"record_offset":10872,"parent_record_offset":10848,"ancestor_record_offset":7988,"inlinee":8632,"source_line":114,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":4201,"record_offset":10028,"parent_record_offset":9556,"ancestor_record_offset":7988,"inlinee":10319,"source_line":752,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::column::ThinColumn::get_changed_ticks_slice
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/column.rs
//     evidence={"module":4201,"record_offset":13584,"parent_record_offset":13184,"ancestor_record_offset":7988,"inlinee":11819,"source_line":313,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::column::ThinColumn::get_data_slice
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/column.rs
//     evidence={"module":4201,"record_offset":10696,"parent_record_offset":10672,"ancestor_record_offset":7988,"inlinee":10326,"source_line":297,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::impl$4::get_data_slice_for::closure$0
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":4201,"record_offset":10672,"parent_record_offset":10648,"ancestor_record_offset":7988,"inlinee":10325,"source_line":382,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::thin_array_ptr::ThinArrayPtr::as_slice
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/thin_array_ptr.rs
//     evidence={"module":4201,"record_offset":13608,"parent_record_offset":13584,"ancestor_record_offset":7988,"inlinee":11820,"source_line":300,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::queue
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/commands/mod.rs
//     evidence={"module":4201,"record_offset":15660,"parent_record_offset":15624,"ancestor_record_offset":7988,"inlinee":8582,"source_line":588,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::queue_internal
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/commands/mod.rs
//     evidence={"module":4201,"record_offset":15696,"parent_record_offset":15660,"ancestor_record_offset":7988,"inlinee":8583,"source_line":652,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::EntityCommands::queue
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/commands/mod.rs
//     evidence={"module":4201,"record_offset":15624,"parent_record_offset":15588,"ancestor_record_offset":7988,"inlinee":11368,"source_line":1889,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::get_inner
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":4201,"record_offset":9556,"parent_record_offset":9532,"ancestor_record_offset":7988,"inlinee":11705,"source_line":1539,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::impl$4::into_iter
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":4201,"record_offset":12020,"parent_record_offset":11996,"ancestor_record_offset":7988,"inlinee":10314,"source_line":2505,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::system_param::impl$14::deref_mut
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/system_param.rs
//     evidence={"module":4201,"record_offset":8176,"parent_record_offset":8152,"ancestor_record_offset":7988,"inlinee":9614,"source_line":1064,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] core::bool::impl$0::then
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\bool.rs
//     evidence={"module":4201,"record_offset":9876,"parent_record_offset":9840,"ancestor_record_offset":7988,"inlinee":4276,"source_line":65,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::and_then_or_clear
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\chain.rs
//     evidence={"module":4201,"record_offset":9324,"parent_record_offset":9300,"ancestor_record_offset":7988,"inlinee":7044,"source_line":331,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::last
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\chain.rs
//     evidence={"module":4201,"record_offset":11832,"parent_record_offset":11800,"ancestor_record_offset":7988,"inlinee":10307,"source_line":174,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\chain.rs
//     evidence={"module":4201,"record_offset":9300,"parent_record_offset":9272,"ancestor_record_offset":7988,"inlinee":7042,"source_line":81,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\chain.rs
//     evidence={"module":4201,"record_offset":9404,"parent_record_offset":9380,"ancestor_record_offset":7988,"inlinee":7430,"source_line":82,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":4201,"record_offset":8420,"parent_record_offset":8392,"ancestor_record_offset":7988,"inlinee":7937,"source_line":917,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":4201,"record_offset":8280,"parent_record_offset":8240,"ancestor_record_offset":7988,"inlinee":9345,"source_line":2344,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":4201,"record_offset":10456,"parent_record_offset":10432,"ancestor_record_offset":7988,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":4201,"record_offset":10432,"parent_record_offset":10408,"ancestor_record_offset":7988,"inlinee":5449,"source_line":1541,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":4201,"record_offset":9348,"parent_record_offset":9324,"ancestor_record_offset":7988,"inlinee":4102,"source_line":766,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":4201,"record_offset":10480,"parent_record_offset":10456,"ancestor_record_offset":7988,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::filter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":4201,"record_offset":9712,"parent_record_offset":9628,"ancestor_record_offset":7988,"inlinee":8618,"source_line":1578,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":4201,"record_offset":10520,"parent_record_offset":10384,"ancestor_record_offset":7988,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::ok_or
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":4201,"record_offset":9812,"parent_record_offset":9556,"ancestor_record_offset":7988,"inlinee":5838,"source_line":1337,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":4201,"record_offset":11928,"parent_record_offset":11832,"ancestor_record_offset":7988,"inlinee":10308,"source_line":1621,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":4201,"record_offset":9380,"parent_record_offset":9300,"ancestor_record_offset":7988,"inlinee":7428,"source_line":1647,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$47::branch
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":4201,"record_offset":11960,"parent_record_offset":11800,"ancestor_record_offset":7988,"inlinee":5260,"source_line":2765,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":4201,"record_offset":8752,"parent_record_offset":8728,"ancestor_record_offset":7988,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":4201,"record_offset":9480,"parent_record_offset":9456,"ancestor_record_offset":7988,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":4201,"record_offset":12424,"parent_record_offset":12396,"ancestor_record_offset":7988,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":4201,"record_offset":9172,"parent_record_offset":9008,"ancestor_record_offset":7988,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":4201,"record_offset":8336,"parent_record_offset":8240,"ancestor_record_offset":7988,"inlinee":4165,"source_line":572,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":4201,"record_offset":14076,"parent_record_offset":12312,"ancestor_record_offset":7988,"inlinee":4887,"source_line":639,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":4201,"record_offset":8704,"parent_record_offset":8240,"ancestor_record_offset":7988,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":4201,"record_offset":9680,"parent_record_offset":9656,"ancestor_record_offset":7988,"inlinee":4167,"source_line":218,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":4201,"record_offset":10600,"parent_record_offset":10568,"ancestor_record_offset":7988,"inlinee":4183,"source_line":239,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::get
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":4201,"record_offset":8392,"parent_record_offset":8364,"ancestor_record_offset":7988,"inlinee":8594,"source_line":366,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$7::get
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":4201,"record_offset":8364,"parent_record_offset":8336,"ancestor_record_offset":7988,"inlinee":8593,"source_line":545,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":4201,"record_offset":8728,"parent_record_offset":8704,"ancestor_record_offset":7988,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":4201,"record_offset":9456,"parent_record_offset":9404,"ancestor_record_offset":7988,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::contains_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":4201,"record_offset":9924,"parent_record_offset":9900,"ancestor_record_offset":7988,"inlinee":4279,"source_line":408,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::get_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":4201,"record_offset":9948,"parent_record_offset":9924,"ancestor_record_offset":7988,"inlinee":4280,"source_line":171,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::impl$2::contains::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":4201,"record_offset":9900,"parent_record_offset":9876,"ancestor_record_offset":7988,"inlinee":4278,"source_line":395,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxU32::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nonmax","version":"0.5.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":4201,"record_offset":10904,"parent_record_offset":10872,"ancestor_record_offset":7988,"inlinee":8634,"source_line":153,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"nonmax","version":"0.5.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":4201,"record_offset":10568,"parent_record_offset":10544,"ancestor_record_offset":7988,"inlinee":4193,"source_line":153,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::toolbar_button::switch_toolbar_category_button
// 记录形式: compiled_procedure; instances=1
// 源码行: 166
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":4201,"record_offset":33428,"rva":43410512,"va":5412119632}
// 编译实例 RVAs: 0x2966450
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f550
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: toolbar_button.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4230887","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412119632,"raw_hex":"e8e78e4000","size":5,"soft_fail":false,"target_va":5416351056,"va":5412120164,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6fd10
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: toolbar_button.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4232172","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412119632,"raw_hex":"e8ec934000","size":5,"soft_fail":false,"target_va":5416353040,"va":5412120863,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$58::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":4201,"record_offset":37468,"parent_record_offset":33428,"ancestor_record_offset":33428,"inlinee":9404,"source_line":434,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::message::iterators::impl$0::last
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/iterators.rs
//     evidence={"module":4201,"record_offset":36968,"parent_record_offset":33428,"ancestor_record_offset":33428,"inlinee":13153,"source_line":26,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::message::iterators::impl$0::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/iterators.rs
//     evidence={"module":4201,"record_offset":34700,"parent_record_offset":33428,"ancestor_record_offset":33428,"inlinee":11533,"source_line":14,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::message::message_reader::MessageReader::read
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/message_reader.rs
//     evidence={"module":4201,"record_offset":33604,"parent_record_offset":33428,"ancestor_record_offset":33428,"inlinee":9339,"source_line":27,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::message::message_writer::MessageWriter::write
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/message_writer.rs
//     evidence={"module":4201,"record_offset":34312,"parent_record_offset":33428,"ancestor_record_offset":33428,"inlinee":11206,"source_line":69,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::query::iter::impl$1::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs
//     evidence={"module":4201,"record_offset":37532,"parent_record_offset":33428,"ancestor_record_offset":33428,"inlinee":10317,"source_line":909,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::commands::EntityCommands::insert
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/commands/mod.rs
//     evidence={"module":4201,"record_offset":40812,"parent_record_offset":33428,"ancestor_record_offset":33428,"inlinee":11388,"source_line":1390,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::query::Query::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":4201,"record_offset":34984,"parent_record_offset":33428,"ancestor_record_offset":33428,"inlinee":11704,"source_line":1388,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::query::Query::iter_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":4201,"record_offset":37156,"parent_record_offset":33428,"ancestor_record_offset":33428,"inlinee":18903,"source_line":704,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::toolbar::impl$15::eq
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/toolbar.rs
//     evidence={"module":4201,"record_offset":37436,"parent_record_offset":33428,"ancestor_record_offset":33428,"inlinee":35842,"source_line":53,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4201,"record_offset":34536,"parent_record_offset":34504,"ancestor_record_offset":33428,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4201,"record_offset":40972,"parent_record_offset":40940,"ancestor_record_offset":33428,"inlinee":5066,"source_line":338,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4201,"record_offset":41072,"parent_record_offset":41044,"ancestor_record_offset":33428,"inlinee":4788,"source_line":616,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4201,"record_offset":41044,"parent_record_offset":41004,"ancestor_record_offset":33428,"inlinee":5068,"source_line":764,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4201,"record_offset":34584,"parent_record_offset":34560,"ancestor_record_offset":33428,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4201,"record_offset":34560,"parent_record_offset":34536,"ancestor_record_offset":33428,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4201,"record_offset":41004,"parent_record_offset":40972,"ancestor_record_offset":33428,"inlinee":5067,"source_line":653,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":4201,"record_offset":34504,"parent_record_offset":34460,"ancestor_record_offset":33428,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":4201,"record_offset":35560,"parent_record_offset":35536,"ancestor_record_offset":33428,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":4201,"record_offset":33940,"parent_record_offset":33916,"ancestor_record_offset":33428,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":4201,"record_offset":37824,"parent_record_offset":37800,"ancestor_record_offset":33428,"inlinee":5735,"source_line":3021,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":4201,"record_offset":37848,"parent_record_offset":37824,"ancestor_record_offset":33428,"inlinee":4800,"source_line":2996,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":4201,"record_offset":34428,"parent_record_offset":34384,"ancestor_record_offset":33428,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":4201,"record_offset":34460,"parent_record_offset":34428,"ancestor_record_offset":33428,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":4201,"record_offset":40940,"parent_record_offset":40908,"ancestor_record_offset":33428,"inlinee":5090,"source_line":1445,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":4201,"record_offset":33916,"parent_record_offset":33692,"ancestor_record_offset":33428,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::entities
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs
//     evidence={"module":4201,"record_offset":40576,"parent_record_offset":37568,"ancestor_record_offset":33428,"inlinee":10334,"source_line":484,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::is_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs
//     evidence={"module":4201,"record_offset":39248,"parent_record_offset":37568,"ancestor_record_offset":33428,"inlinee":10331,"source_line":655,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::table_id
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs
//     evidence={"module":4201,"record_offset":39396,"parent_record_offset":37568,"ancestor_record_offset":33428,"inlinee":8568,"source_line":478,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::ArchetypeId::index
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs
//     evidence={"module":4201,"record_offset":35292,"parent_record_offset":35008,"ancestor_record_offset":33428,"inlinee":8565,"source_line":122,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetypes::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs
//     evidence={"module":4201,"record_offset":39164,"parent_record_offset":37568,"ancestor_record_offset":33428,"inlinee":10330,"source_line":864,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":4201,"record_offset":34664,"parent_record_offset":34340,"ancestor_record_offset":33428,"inlinee":7224,"source_line":397,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$57::set_changed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":4201,"record_offset":37496,"parent_record_offset":37468,"ancestor_record_offset":33428,"inlinee":9399,"source_line":397,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::Entities::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/entity/mod.rs
//     evidence={"module":4201,"record_offset":35056,"parent_record_offset":35008,"ancestor_record_offset":33428,"inlinee":8602,"source_line":948,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::Entities::get_id_location
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/entity/mod.rs
//     evidence={"module":4201,"record_offset":35080,"parent_record_offset":35056,"ancestor_record_offset":33428,"inlinee":8603,"source_line":955,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::impl$18::get_id_location::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/entity/mod.rs
//     evidence={"module":4201,"record_offset":35196,"parent_record_offset":35164,"ancestor_record_offset":33428,"inlinee":8620,"source_line":958,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::impl$37::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/entity/mod.rs
//     evidence={"module":4201,"record_offset":35220,"parent_record_offset":35196,"ancestor_record_offset":33428,"inlinee":8622,"source_line":214,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::iterators::MessageIteratorWithId::new
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/iterators.rs
//     evidence={"module":4201,"record_offset":33692,"parent_record_offset":33668,"ancestor_record_offset":33428,"inlinee":9344,"source_line":54,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::iterators::impl$3::last
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/iterators.rs
//     evidence={"module":4201,"record_offset":37000,"parent_record_offset":36968,"ancestor_record_offset":33428,"inlinee":10306,"source_line":112,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::iterators::impl$3::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/iterators.rs
//     evidence={"module":4201,"record_offset":34724,"parent_record_offset":34700,"ancestor_record_offset":33428,"inlinee":11534,"source_line":86,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::message_cursor::MessageCursor::read_with_id
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/message_cursor.rs
//     evidence={"module":4201,"record_offset":33668,"parent_record_offset":33628,"ancestor_record_offset":33428,"inlinee":9342,"source_line":89,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::messages::Messages::write
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/messages.rs
//     evidence={"module":4201,"record_offset":34340,"parent_record_offset":34312,"ancestor_record_offset":33428,"inlinee":8680,"source_line":125,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::messages::Messages::write_with_caller
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/messages.rs
//     evidence={"module":4201,"record_offset":34384,"parent_record_offset":34340,"ancestor_record_offset":33428,"inlinee":8681,"source_line":129,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::StorageSwitch::extract
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":4201,"record_offset":36276,"parent_record_offset":36252,"ancestor_record_offset":33428,"inlinee":9590,"source_line":2964,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::set_archetype
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":4201,"record_offset":35736,"parent_record_offset":35712,"ancestor_record_offset":33428,"inlinee":10333,"source_line":1530,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::set_table
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":4201,"record_offset":35760,"parent_record_offset":35736,"ancestor_record_offset":33428,"inlinee":10322,"source_line":1545,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$37::fetch
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":4201,"record_offset":36252,"parent_record_offset":36228,"ancestor_record_offset":33428,"inlinee":10341,"source_line":1598,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$37::fetch::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":4201,"record_offset":36300,"parent_record_offset":36276,"ancestor_record_offset":33428,"inlinee":9575,"source_line":1605,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$48::set_archetype
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":4201,"record_offset":40076,"parent_record_offset":39488,"ancestor_record_offset":33428,"inlinee":11800,"source_line":1912,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$48::set_table
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":4201,"record_offset":38440,"parent_record_offset":37884,"ancestor_record_offset":33428,"inlinee":11801,"source_line":1927,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$92::fetch
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":4201,"record_offset":36228,"parent_record_offset":35008,"ancestor_record_offset":33428,"inlinee":11825,"source_line":2520,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIter::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs
//     evidence={"module":4201,"record_offset":37204,"parent_record_offset":37180,"ancestor_record_offset":33428,"inlinee":10316,"source_line":40,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::init
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs
//     evidence={"module":4201,"record_offset":37228,"parent_record_offset":37204,"ancestor_record_offset":33428,"inlinee":9124,"source_line":2408,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs
//     evidence={"module":4201,"record_offset":37568,"parent_record_offset":37532,"ancestor_record_offset":33428,"inlinee":10318,"source_line":2511,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$3::set_archetype
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/world_query.rs
//     evidence={"module":4201,"record_offset":35712,"parent_record_offset":35008,"ancestor_record_offset":33428,"inlinee":10710,"source_line":182,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$3::set_table
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/world_query.rs
//     evidence={"module":4201,"record_offset":37884,"parent_record_offset":37568,"ancestor_record_offset":33428,"inlinee":10708,"source_line":195,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_array::BlobArray::get_sub_slice
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_array.rs
//     evidence={"module":4201,"record_offset":36172,"parent_record_offset":36148,"ancestor_record_offset":33428,"inlinee":10327,"source_line":143,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseArray::get
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":4201,"record_offset":35860,"parent_record_offset":35836,"ancestor_record_offset":33428,"inlinee":8615,"source_line":56,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":4201,"record_offset":35836,"parent_record_offset":35812,"ancestor_record_offset":33428,"inlinee":8626,"source_line":405,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$16::get::closure$0
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":4201,"record_offset":35996,"parent_record_offset":35972,"ancestor_record_offset":33428,"inlinee":8628,"source_line":406,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::entities
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":4201,"record_offset":38904,"parent_record_offset":37568,"ancestor_record_offset":33428,"inlinee":10328,"source_line":208,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_column
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":4201,"record_offset":35812,"parent_record_offset":35784,"ancestor_record_offset":33428,"inlinee":8625,"source_line":489,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_data_slice_for
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":4201,"record_offset":35784,"parent_record_offset":35760,"ancestor_record_offset":33428,"inlinee":10323,"source_line":377,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::is_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":4201,"record_offset":37800,"parent_record_offset":37568,"ancestor_record_offset":33428,"inlinee":10320,"source_line":646,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::TableRow::index
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":4201,"record_offset":36324,"parent_record_offset":36300,"ancestor_record_offset":33428,"inlinee":8632,"source_line":114,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":4201,"record_offset":35480,"parent_record_offset":35008,"ancestor_record_offset":33428,"inlinee":10319,"source_line":752,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::column::ThinColumn::get_changed_ticks_slice
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/column.rs
//     evidence={"module":4201,"record_offset":38840,"parent_record_offset":38440,"ancestor_record_offset":33428,"inlinee":11819,"source_line":313,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::column::ThinColumn::get_data_slice
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/column.rs
//     evidence={"module":4201,"record_offset":36148,"parent_record_offset":36124,"ancestor_record_offset":33428,"inlinee":10326,"source_line":297,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::impl$4::get_data_slice_for::closure$0
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":4201,"record_offset":36124,"parent_record_offset":36100,"ancestor_record_offset":33428,"inlinee":10325,"source_line":382,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::thin_array_ptr::ThinArrayPtr::as_slice
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/thin_array_ptr.rs
//     evidence={"module":4201,"record_offset":38864,"parent_record_offset":38840,"ancestor_record_offset":33428,"inlinee":11820,"source_line":300,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::queue
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/commands/mod.rs
//     evidence={"module":4201,"record_offset":40876,"parent_record_offset":40844,"ancestor_record_offset":33428,"inlinee":8582,"source_line":588,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::queue_internal
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/commands/mod.rs
//     evidence={"module":4201,"record_offset":40908,"parent_record_offset":40876,"ancestor_record_offset":33428,"inlinee":8583,"source_line":652,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::EntityCommands::queue
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/commands/mod.rs
//     evidence={"module":4201,"record_offset":40844,"parent_record_offset":40812,"ancestor_record_offset":33428,"inlinee":11368,"source_line":1889,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::get_inner
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":4201,"record_offset":35008,"parent_record_offset":34984,"ancestor_record_offset":33428,"inlinee":11705,"source_line":1539,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::impl$4::into_iter
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":4201,"record_offset":37180,"parent_record_offset":37156,"ancestor_record_offset":33428,"inlinee":10314,"source_line":2505,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::system_param::impl$14::deref_mut
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/system_param.rs
//     evidence={"module":4201,"record_offset":33628,"parent_record_offset":33604,"ancestor_record_offset":33428,"inlinee":9614,"source_line":1064,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] core::bool::impl$0::then
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\bool.rs
//     evidence={"module":4201,"record_offset":35328,"parent_record_offset":35292,"ancestor_record_offset":33428,"inlinee":4276,"source_line":65,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::and_then_or_clear
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\chain.rs
//     evidence={"module":4201,"record_offset":34776,"parent_record_offset":34752,"ancestor_record_offset":33428,"inlinee":7044,"source_line":331,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::last
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\chain.rs
//     evidence={"module":4201,"record_offset":37032,"parent_record_offset":37000,"ancestor_record_offset":33428,"inlinee":10307,"source_line":174,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\chain.rs
//     evidence={"module":4201,"record_offset":34752,"parent_record_offset":34724,"ancestor_record_offset":33428,"inlinee":7042,"source_line":81,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\chain.rs
//     evidence={"module":4201,"record_offset":34856,"parent_record_offset":34832,"ancestor_record_offset":33428,"inlinee":7430,"source_line":82,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":4201,"record_offset":33872,"parent_record_offset":33844,"ancestor_record_offset":33428,"inlinee":7937,"source_line":917,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":4201,"record_offset":33732,"parent_record_offset":33692,"ancestor_record_offset":33428,"inlinee":9345,"source_line":2344,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":4201,"record_offset":35908,"parent_record_offset":35884,"ancestor_record_offset":33428,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":4201,"record_offset":35884,"parent_record_offset":35860,"ancestor_record_offset":33428,"inlinee":5449,"source_line":1541,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":4201,"record_offset":34800,"parent_record_offset":34776,"ancestor_record_offset":33428,"inlinee":4102,"source_line":766,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":4201,"record_offset":35932,"parent_record_offset":35908,"ancestor_record_offset":33428,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::filter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":4201,"record_offset":35164,"parent_record_offset":35080,"ancestor_record_offset":33428,"inlinee":8618,"source_line":1578,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":4201,"record_offset":35972,"parent_record_offset":35836,"ancestor_record_offset":33428,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::ok_or
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":4201,"record_offset":35264,"parent_record_offset":35008,"ancestor_record_offset":33428,"inlinee":5838,"source_line":1337,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":4201,"record_offset":34832,"parent_record_offset":34752,"ancestor_record_offset":33428,"inlinee":7428,"source_line":1647,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$47::branch
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":4201,"record_offset":37120,"parent_record_offset":37000,"ancestor_record_offset":33428,"inlinee":5260,"source_line":2765,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":4201,"record_offset":34204,"parent_record_offset":34180,"ancestor_record_offset":33428,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":4201,"record_offset":34932,"parent_record_offset":34908,"ancestor_record_offset":33428,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":4201,"record_offset":37680,"parent_record_offset":37652,"ancestor_record_offset":33428,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":4201,"record_offset":34624,"parent_record_offset":34460,"ancestor_record_offset":33428,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":4201,"record_offset":33788,"parent_record_offset":33692,"ancestor_record_offset":33428,"inlinee":4165,"source_line":572,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":4201,"record_offset":39332,"parent_record_offset":37568,"ancestor_record_offset":33428,"inlinee":4887,"source_line":639,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":4201,"record_offset":34156,"parent_record_offset":33692,"ancestor_record_offset":33428,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":4201,"record_offset":35132,"parent_record_offset":35108,"ancestor_record_offset":33428,"inlinee":4167,"source_line":218,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":4201,"record_offset":36052,"parent_record_offset":36020,"ancestor_record_offset":33428,"inlinee":4183,"source_line":239,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::get
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":4201,"record_offset":33844,"parent_record_offset":33816,"ancestor_record_offset":33428,"inlinee":8594,"source_line":366,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$7::get
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":4201,"record_offset":33816,"parent_record_offset":33788,"ancestor_record_offset":33428,"inlinee":8593,"source_line":545,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":4201,"record_offset":34180,"parent_record_offset":34156,"ancestor_record_offset":33428,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":4201,"record_offset":34908,"parent_record_offset":34856,"ancestor_record_offset":33428,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::contains_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":4201,"record_offset":35376,"parent_record_offset":35352,"ancestor_record_offset":33428,"inlinee":4279,"source_line":408,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::get_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":4201,"record_offset":35400,"parent_record_offset":35376,"ancestor_record_offset":33428,"inlinee":4280,"source_line":171,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::impl$2::contains::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":4201,"record_offset":35352,"parent_record_offset":35328,"ancestor_record_offset":33428,"inlinee":4278,"source_line":395,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxU32::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nonmax","version":"0.5.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":4201,"record_offset":36348,"parent_record_offset":36324,"ancestor_record_offset":33428,"inlinee":8634,"source_line":153,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"nonmax","version":"0.5.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":4201,"record_offset":36020,"parent_record_offset":35996,"ancestor_record_offset":33428,"inlinee":4193,"source_line":153,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::toolbar_button::tod_v2_button
// 记录形式: compiled_procedure; instances=1
// 源码行: 208
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":4201,"record_offset":1692,"rva":43404208,"va":5412113328}
// 编译实例 RVAs: 0x2964bb0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f2b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: toolbar_button.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4236330","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412113328,"raw_hex":"e82aa44000","size":5,"soft_fail":false,"target_va":5416350384,"va":5412114049,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6fd10
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: toolbar_button.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4238884","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412113328,"raw_hex":"e824ae4000","size":5,"soft_fail":false,"target_va":5416353040,"va":5412114151,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":4201,"record_offset":3324,"parent_record_offset":1692,"ancestor_record_offset":1692,"inlinee":7222,"source_line":434,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$58::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":4201,"record_offset":7920,"parent_record_offset":1692,"ancestor_record_offset":1692,"inlinee":9404,"source_line":434,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::message::iterators::impl$0::last
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/iterators.rs
//     evidence={"module":4201,"record_offset":4940,"parent_record_offset":1692,"ancestor_record_offset":1692,"inlinee":13153,"source_line":26,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::message::iterators::impl$0::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/iterators.rs
//     evidence={"module":4201,"record_offset":2556,"parent_record_offset":1692,"ancestor_record_offset":1692,"inlinee":11533,"source_line":14,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::message::message_reader::MessageReader::read
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/message_reader.rs
//     evidence={"module":4201,"record_offset":1856,"parent_record_offset":1692,"ancestor_record_offset":1692,"inlinee":9339,"source_line":27,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::message::message_writer::MessageWriter::write
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/message_writer.rs
//     evidence={"module":4201,"record_offset":3788,"parent_record_offset":1692,"ancestor_record_offset":1692,"inlinee":11206,"source_line":69,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::query::iter::impl$1::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs
//     evidence={"module":4201,"record_offset":5584,"parent_record_offset":1692,"ancestor_record_offset":1692,"inlinee":10317,"source_line":909,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::commands::EntityCommands::insert
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/commands/mod.rs
//     evidence={"module":4201,"record_offset":3380,"parent_record_offset":1692,"ancestor_record_offset":1692,"inlinee":11388,"source_line":1390,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::query::Query::get_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":4201,"record_offset":2840,"parent_record_offset":1692,"ancestor_record_offset":1692,"inlinee":11799,"source_line":1524,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::query::Query::iter_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":4201,"record_offset":5240,"parent_record_offset":1692,"ancestor_record_offset":1692,"inlinee":18903,"source_line":704,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":4201,"record_offset":3732,"parent_record_offset":1692,"ancestor_record_offset":1692,"inlinee":4487,"source_line":264,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4201,"record_offset":4012,"parent_record_offset":3980,"ancestor_record_offset":1692,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4201,"record_offset":3524,"parent_record_offset":3496,"ancestor_record_offset":1692,"inlinee":5066,"source_line":338,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4201,"record_offset":3608,"parent_record_offset":3584,"ancestor_record_offset":1692,"inlinee":4788,"source_line":616,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4201,"record_offset":3584,"parent_record_offset":3552,"ancestor_record_offset":1692,"inlinee":5068,"source_line":764,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4201,"record_offset":4060,"parent_record_offset":4036,"ancestor_record_offset":1692,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4201,"record_offset":4036,"parent_record_offset":4012,"ancestor_record_offset":1692,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4201,"record_offset":3552,"parent_record_offset":3524,"ancestor_record_offset":1692,"inlinee":5067,"source_line":653,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":4201,"record_offset":3980,"parent_record_offset":3936,"ancestor_record_offset":1692,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":4201,"record_offset":5392,"parent_record_offset":5368,"ancestor_record_offset":1692,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":4201,"record_offset":2200,"parent_record_offset":2176,"ancestor_record_offset":1692,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":4201,"record_offset":5892,"parent_record_offset":5868,"ancestor_record_offset":1692,"inlinee":5735,"source_line":3021,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":4201,"record_offset":5916,"parent_record_offset":5892,"ancestor_record_offset":1692,"inlinee":4800,"source_line":2996,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":4201,"record_offset":3908,"parent_record_offset":3872,"ancestor_record_offset":1692,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":4201,"record_offset":3936,"parent_record_offset":3908,"ancestor_record_offset":1692,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":4201,"record_offset":3496,"parent_record_offset":3464,"ancestor_record_offset":1692,"inlinee":5090,"source_line":1445,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":4201,"record_offset":2176,"parent_record_offset":1944,"ancestor_record_offset":1692,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::entities
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs
//     evidence={"module":4201,"record_offset":6552,"parent_record_offset":5616,"ancestor_record_offset":1692,"inlinee":10334,"source_line":484,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::is_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs
//     evidence={"module":4201,"record_offset":5868,"parent_record_offset":5616,"ancestor_record_offset":1692,"inlinee":10331,"source_line":655,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::table_id
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs
//     evidence={"module":4201,"record_offset":5952,"parent_record_offset":5616,"ancestor_record_offset":1692,"inlinee":8568,"source_line":478,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::ArchetypeEntity::table_row
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs
//     evidence={"module":4201,"record_offset":6804,"parent_record_offset":5616,"ancestor_record_offset":1692,"inlinee":10337,"source_line":346,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetypes::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs
//     evidence={"module":4201,"record_offset":5784,"parent_record_offset":5616,"ancestor_record_offset":1692,"inlinee":10330,"source_line":864,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":4201,"record_offset":3348,"parent_record_offset":3324,"ancestor_record_offset":1692,"inlinee":7224,"source_line":397,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":4201,"record_offset":3812,"parent_record_offset":3788,"ancestor_record_offset":1692,"inlinee":7222,"source_line":434,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$57::set_changed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":4201,"record_offset":7948,"parent_record_offset":7920,"ancestor_record_offset":1692,"inlinee":9399,"source_line":397,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::Entities::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/entity/mod.rs
//     evidence={"module":4201,"record_offset":2900,"parent_record_offset":2864,"ancestor_record_offset":1692,"inlinee":8602,"source_line":948,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::Entities::get_id_location
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/entity/mod.rs
//     evidence={"module":4201,"record_offset":2924,"parent_record_offset":2900,"ancestor_record_offset":1692,"inlinee":8603,"source_line":955,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::impl$18::get_id_location::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/entity/mod.rs
//     evidence={"module":4201,"record_offset":3040,"parent_record_offset":3008,"ancestor_record_offset":1692,"inlinee":8620,"source_line":958,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::impl$37::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/entity/mod.rs
//     evidence={"module":4201,"record_offset":3064,"parent_record_offset":3040,"ancestor_record_offset":1692,"inlinee":8622,"source_line":214,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::iterators::MessageIteratorWithId::new
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/iterators.rs
//     evidence={"module":4201,"record_offset":1944,"parent_record_offset":1920,"ancestor_record_offset":1692,"inlinee":9344,"source_line":54,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::iterators::impl$3::last
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/iterators.rs
//     evidence={"module":4201,"record_offset":4980,"parent_record_offset":4940,"ancestor_record_offset":1692,"inlinee":10306,"source_line":112,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::iterators::impl$3::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/iterators.rs
//     evidence={"module":4201,"record_offset":2580,"parent_record_offset":2556,"ancestor_record_offset":1692,"inlinee":11534,"source_line":86,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::message_cursor::MessageCursor::read_with_id
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/message_cursor.rs
//     evidence={"module":4201,"record_offset":1920,"parent_record_offset":1880,"ancestor_record_offset":1692,"inlinee":9342,"source_line":89,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::messages::Messages::write_with_caller
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/messages.rs
//     evidence={"module":4201,"record_offset":3872,"parent_record_offset":3812,"ancestor_record_offset":1692,"inlinee":8681,"source_line":129,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::StorageSwitch::extract
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":4201,"record_offset":6880,"parent_record_offset":6856,"ancestor_record_offset":1692,"inlinee":9590,"source_line":2964,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$48::set_archetype
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":4201,"record_offset":6068,"parent_record_offset":6044,"ancestor_record_offset":1692,"inlinee":11800,"source_line":1912,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$48::set_table
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":4201,"record_offset":6096,"parent_record_offset":6068,"ancestor_record_offset":1692,"inlinee":11801,"source_line":1927,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$49::fetch
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":4201,"record_offset":6856,"parent_record_offset":6832,"ancestor_record_offset":1692,"inlinee":11822,"source_line":1987,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$49::fetch::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":4201,"record_offset":6904,"parent_record_offset":6880,"ancestor_record_offset":1692,"inlinee":9577,"source_line":1994,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$89::fetch
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":4201,"record_offset":6832,"parent_record_offset":5616,"ancestor_record_offset":1692,"inlinee":10339,"source_line":2520,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIter::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs
//     evidence={"module":4201,"record_offset":5288,"parent_record_offset":5264,"ancestor_record_offset":1692,"inlinee":10316,"source_line":40,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::init
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs
//     evidence={"module":4201,"record_offset":5312,"parent_record_offset":5288,"ancestor_record_offset":1692,"inlinee":9124,"source_line":2408,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs
//     evidence={"module":4201,"record_offset":5616,"parent_record_offset":5584,"ancestor_record_offset":1692,"inlinee":10318,"source_line":2511,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$2::set_archetype
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/world_query.rs
//     evidence={"module":4201,"record_offset":6044,"parent_record_offset":5616,"ancestor_record_offset":1692,"inlinee":10332,"source_line":182,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$2::set_table
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/world_query.rs
//     evidence={"module":4201,"record_offset":7036,"parent_record_offset":5616,"ancestor_record_offset":1692,"inlinee":10321,"source_line":195,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_array::BlobArray::get_sub_slice
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_array.rs
//     evidence={"module":4201,"record_offset":6452,"parent_record_offset":6428,"ancestor_record_offset":1692,"inlinee":10327,"source_line":143,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseArray::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":4201,"record_offset":6188,"parent_record_offset":6160,"ancestor_record_offset":1692,"inlinee":8615,"source_line":56,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":4201,"record_offset":6160,"parent_record_offset":6132,"ancestor_record_offset":1692,"inlinee":8626,"source_line":405,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$16::get::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":4201,"record_offset":6324,"parent_record_offset":6300,"ancestor_record_offset":1692,"inlinee":8628,"source_line":406,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_column
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":4201,"record_offset":6132,"parent_record_offset":6096,"ancestor_record_offset":1692,"inlinee":8625,"source_line":489,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::is_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":4201,"record_offset":7828,"parent_record_offset":5616,"ancestor_record_offset":1692,"inlinee":10320,"source_line":646,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::TableRow::index
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":4201,"record_offset":6928,"parent_record_offset":6904,"ancestor_record_offset":1692,"inlinee":8632,"source_line":114,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":4201,"record_offset":7744,"parent_record_offset":5616,"ancestor_record_offset":1692,"inlinee":10319,"source_line":752,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::column::ThinColumn::get_changed_ticks_slice
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/column.rs
//     evidence={"module":4201,"record_offset":6484,"parent_record_offset":6096,"ancestor_record_offset":1692,"inlinee":11819,"source_line":313,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::column::ThinColumn::get_data_slice
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/column.rs
//     evidence={"module":4201,"record_offset":6428,"parent_record_offset":6096,"ancestor_record_offset":1692,"inlinee":10326,"source_line":297,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::thin_array_ptr::ThinArrayPtr::as_slice
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/thin_array_ptr.rs
//     evidence={"module":4201,"record_offset":6508,"parent_record_offset":6484,"ancestor_record_offset":1692,"inlinee":11820,"source_line":300,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::queue
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/commands/mod.rs
//     evidence={"module":4201,"record_offset":3436,"parent_record_offset":3408,"ancestor_record_offset":1692,"inlinee":8582,"source_line":588,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::queue_internal
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/commands/mod.rs
//     evidence={"module":4201,"record_offset":3464,"parent_record_offset":3436,"ancestor_record_offset":1692,"inlinee":8583,"source_line":652,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::EntityCommands::queue
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/commands/mod.rs
//     evidence={"module":4201,"record_offset":3408,"parent_record_offset":3380,"ancestor_record_offset":1692,"inlinee":11368,"source_line":1889,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::get_inner
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":4201,"record_offset":2864,"parent_record_offset":2840,"ancestor_record_offset":1692,"inlinee":11705,"source_line":1539,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::impl$4::into_iter
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":4201,"record_offset":5264,"parent_record_offset":5240,"ancestor_record_offset":1692,"inlinee":10314,"source_line":2505,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::system_param::impl$14::deref_mut
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/system_param.rs
//     evidence={"module":4201,"record_offset":1880,"parent_record_offset":1856,"ancestor_record_offset":1692,"inlinee":9614,"source_line":1064,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ptr::ThinSlicePtr::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ptr","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ptr-0.17.3/src/lib.rs
//     evidence={"module":4201,"record_offset":7608,"parent_record_offset":7584,"ancestor_record_offset":1692,"inlinee":9579,"source_line":1073,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] core::bool::impl$0::then
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\bool.rs
//     evidence={"module":4201,"record_offset":3164,"parent_record_offset":3136,"ancestor_record_offset":1692,"inlinee":4276,"source_line":65,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::and_then_or_clear
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\chain.rs
//     evidence={"module":4201,"record_offset":2632,"parent_record_offset":2608,"ancestor_record_offset":1692,"inlinee":7044,"source_line":331,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::last
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\chain.rs
//     evidence={"module":4201,"record_offset":5020,"parent_record_offset":4980,"ancestor_record_offset":1692,"inlinee":10307,"source_line":174,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\chain.rs
//     evidence={"module":4201,"record_offset":2608,"parent_record_offset":2580,"ancestor_record_offset":1692,"inlinee":7042,"source_line":81,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\chain.rs
//     evidence={"module":4201,"record_offset":2712,"parent_record_offset":2688,"ancestor_record_offset":1692,"inlinee":7430,"source_line":82,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":4201,"record_offset":2132,"parent_record_offset":2104,"ancestor_record_offset":1692,"inlinee":7937,"source_line":917,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":4201,"record_offset":1992,"parent_record_offset":1944,"ancestor_record_offset":1692,"inlinee":9345,"source_line":2344,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":4201,"record_offset":6236,"parent_record_offset":6212,"ancestor_record_offset":1692,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":4201,"record_offset":5052,"parent_record_offset":5020,"ancestor_record_offset":1692,"inlinee":5449,"source_line":1541,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":4201,"record_offset":2656,"parent_record_offset":2632,"ancestor_record_offset":1692,"inlinee":4102,"source_line":766,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":4201,"record_offset":6260,"parent_record_offset":6236,"ancestor_record_offset":1692,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::filter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":4201,"record_offset":3008,"parent_record_offset":2924,"ancestor_record_offset":1692,"inlinee":8618,"source_line":1578,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":4201,"record_offset":6300,"parent_record_offset":6160,"ancestor_record_offset":1692,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::ok_or
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":4201,"record_offset":3108,"parent_record_offset":2864,"ancestor_record_offset":1692,"inlinee":5838,"source_line":1337,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":4201,"record_offset":5108,"parent_record_offset":5020,"ancestor_record_offset":1692,"inlinee":10308,"source_line":1621,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":4201,"record_offset":2688,"parent_record_offset":2608,"ancestor_record_offset":1692,"inlinee":7428,"source_line":1647,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$47::branch
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":4201,"record_offset":5140,"parent_record_offset":4980,"ancestor_record_offset":1692,"inlinee":5260,"source_line":2765,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":4201,"record_offset":2448,"parent_record_offset":2424,"ancestor_record_offset":1692,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":4201,"record_offset":2788,"parent_record_offset":2764,"ancestor_record_offset":1692,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":4201,"record_offset":5748,"parent_record_offset":5720,"ancestor_record_offset":1692,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":4201,"record_offset":4100,"parent_record_offset":3936,"ancestor_record_offset":1692,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":4201,"record_offset":2048,"parent_record_offset":1944,"ancestor_record_offset":1692,"inlinee":4165,"source_line":572,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":4201,"record_offset":6748,"parent_record_offset":5616,"ancestor_record_offset":1692,"inlinee":4887,"source_line":639,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":4201,"record_offset":2400,"parent_record_offset":1944,"ancestor_record_offset":1692,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":4201,"record_offset":2976,"parent_record_offset":2952,"ancestor_record_offset":1692,"inlinee":4167,"source_line":218,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":4201,"record_offset":6380,"parent_record_offset":6348,"ancestor_record_offset":1692,"inlinee":4183,"source_line":239,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::get
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":4201,"record_offset":2104,"parent_record_offset":2076,"ancestor_record_offset":1692,"inlinee":8594,"source_line":366,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$7::get
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":4201,"record_offset":2076,"parent_record_offset":2048,"ancestor_record_offset":1692,"inlinee":8593,"source_line":545,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":4201,"record_offset":2424,"parent_record_offset":2400,"ancestor_record_offset":1692,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":4201,"record_offset":2764,"parent_record_offset":2712,"ancestor_record_offset":1692,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::mode_manager::impl$33::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/mode_manager.rs
//     evidence={"module":4201,"record_offset":3756,"parent_record_offset":3732,"ancestor_record_offset":1692,"inlinee":35837,"source_line":44,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::contains
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":4201,"record_offset":3136,"parent_record_offset":2864,"ancestor_record_offset":1692,"inlinee":4274,"source_line":392,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::contains_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":4201,"record_offset":3212,"parent_record_offset":3188,"ancestor_record_offset":1692,"inlinee":4279,"source_line":408,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::get_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":4201,"record_offset":3236,"parent_record_offset":3212,"ancestor_record_offset":1692,"inlinee":4280,"source_line":171,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::impl$2::contains::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":4201,"record_offset":3188,"parent_record_offset":3164,"ancestor_record_offset":1692,"inlinee":4278,"source_line":395,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxU32::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nonmax","version":"0.5.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":4201,"record_offset":6960,"parent_record_offset":6928,"ancestor_record_offset":1692,"inlinee":8634,"source_line":153,"archive":"_pdb/C/modules/04201.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"nonmax","version":"0.5.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":4201,"record_offset":6348,"parent_record_offset":6324,"ancestor_record_offset":1692,"inlinee":4193,"source_line":153,"archive":"_pdb/C/modules/04201.jsonl.gz"}
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
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 30
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::message::message_reader::MessageReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>", "constructor": "bevy_ecs::message::message_reader::MessageReader", "arguments": ["toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::message::message_reader::MessageReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >),tuple$<>,void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::message::message_reader::MessageReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)>", "constructor": "bevy_ecs::system::function_system::FunctionSystem", "arguments": ["void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::message::message_reader::MessageReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)", "tuple$<>", "void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::message::message_reader::MessageReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::message::message_reader::MessageReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >),tuple$<>,void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::message::message_reader::MessageReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)> >", "constructor": "core::ptr::drop_in_place", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::message::message_reader::MessageReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >),tuple$<>,void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::message::message_reader::MessageReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$7::system_type<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::message::message_reader::MessageReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >),tuple$<>,void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::message::message_reader::MessageReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)> >", "constructor": "bevy_ecs::schedule::set::impl$7::system_type", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::message::message_reader::MessageReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >),tuple$<>,void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::message::message_reader::MessageReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$7::dyn_clone<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::message::message_reader::MessageReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >),tuple$<>,void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::message::message_reader::MessageReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)> >", "constructor": "bevy_ecs::schedule::set::impl$7::dyn_clone", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::message::message_reader::MessageReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >),tuple$<>,void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::message::message_reader::MessageReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::message::message_reader::MessageReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >),tuple$<>,void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::message::message_reader::MessageReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)> >", "constructor": "bevy_ecs::schedule::set::SystemTypeSet", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::message::message_reader::MessageReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >),tuple$<>,void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::message::message_reader::MessageReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::any::impl$0::type_id<bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::message::message_reader::MessageReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >),tuple$<>,void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::message::message_reader::MessageReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)> > >", "constructor": "core::any::impl$0::type_id", "arguments": ["bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::message::message_reader::MessageReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >),tuple$<>,void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::message::message_reader::MessageReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::label::impl$0::dyn_eq<bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::message::message_reader::MessageReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >),tuple$<>,void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::message::message_reader::MessageReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)> > >", "constructor": "bevy_ecs::label::impl$0::dyn_eq", "arguments": ["bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::message::message_reader::MessageReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >),tuple$<>,void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::message::message_reader::MessageReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::label::impl$1::dyn_hash<bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::message::message_reader::MessageReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >),tuple$<>,void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::message::message_reader::MessageReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)> > >", "constructor": "bevy_ecs::label::impl$1::dyn_hash", "arguments": ["bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::message::message_reader::MessageReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >),tuple$<>,void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::message::message_reader::MessageReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::message::message_registry::MessageRegistry::register_message<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>", "constructor": "bevy_ecs::message::message_registry::MessageRegistry::register_message", "arguments": ["toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<void (*)(bevy_ecs::change_detection::ResMut<tg_text::TextRenderer>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PhotomodeScreen>,bevy_ecs::change_detection::Res<country_core::startup::init_photomode_state::PhotomodeState>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::onboarding::WelcomeScreenState> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::widget_params::UiNavState>,bevy_ecs::change_detection::Res<country_core::startup::init_photomode_state::SteamdeckScreenshotMsgScreen>,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<country_core::components::UiInteractionState>,ref$<toolbar::resouces_and_components::HideIfMenuIsOpen>,enum2$<core::option::Option<ref$<toolbar::resouces_and_components::ShowOnlyInPhotomode> > > >,tuple$<> >),void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::components::Ui2dExtent>,ref$<country_core::components::UiMouseInteractable>,ref$<toolbar::resouces_and_components::TaffyNode>,ref_mut$<country_core::components::UiInteractionState>,enum2$<core::option::Option<ref$<country_core::components::HistoryButton> > >,enum2$<core::option::Option<ref$<country_core::components::UiNav> > > >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::widget_params::UiNavState>,bevy_ecs::change_detection::ResMut<toolbar::resouces_and_components::UiDirectFocus>,bevy_ecs::message::message_writer::MessageWriter<toolbar::ui_interaction::UiClickEvent>,bevy_ecs::message::message_writer::MessageWriter<toolbar::ui_interaction::UiDragEvent>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::backend::input::calculate_input_delta::UiHistoryButtonPressed>,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<toolbar::toolbar_update::ToolbarSizeMult>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::popup_bounds::UiPopupBounds>),void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<toolbar::resouces_and_components::UiShowIfParentIsInFocus>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::system::query::Query<ref_mut$<country_core::components::UiInteractionState>,bevy_ecs::query::filter::Without<toolbar::resouces_and_components::UiShowIfParentIsInFocus> >),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::message::message_reader::MessageReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >),void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,enum2$<core::option::Option<ref$<country_core::components::Ui2dExtent> > >,enum2$<core::option::Option<ref$<country_core::components::UiInteractionState> > >,ref$<country_core::components::UiNav> >,tuple$<> >,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<toolbar::resouces_and_components::TaffyNode>,ref$<country_core::components::UiNav> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<toolbar::resouces_and_components::taffy_layout::TaffyLayout>,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::widget_params::UiNavState>,bevy_ecs::message::message_writer::MessageWriter<toolbar::ui_interaction::UiClickEvent>),void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::message::message_writer::MessageWriter<toolbar::ui_interaction::UiJustBecameInFocusEvent>,bevy_ecs::system::system_param::Local<std::collections::hash::set::HashSet<bevy_ecs::entity::Entity,std::hash::random::RandomState,alloc::alloc::Global> >),void (*)(bevy_ecs::system::query::Query<ref$<country_core::components::UiInteractionState>,tuple$<> >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::toolbar::ToolbarUiIsInFocus>),tuple$<void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<toolbar::resouces_and_components::UiShowIfParentIsInFocus>,enum2$<core::option::Option<ref$<toolbar::resouces_and_components::ShowOnlyInFpvAndFree> > > >,tuple$<> >,bevy_ecs::system::query::Query<ref_mut$<country_core::components::UiInteractionState>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::startup::init_photomode_state::PhotomodeState>),void (*)(bevy_ecs::system::query::Query<tuple$<ref$<country_core::components::UiInteractionState>,ref_mut$<toolbar::resouces_and_components::UiAnimationState> >,tuple$<> >,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >),void (*)(bevy_ecs::message::message_reader::MessageReader<toolbar::ui_interaction::UiJustBecameInFocusEvent>,bevy_ecs::message::message_reader::MessageReader<toolbar::ui_interaction::UiClickEvent>,bevy_ecs::message::message_reader::MessageReader<toolbar::ui_interaction::UiDragEvent>,bevy_ecs::message::message_writer::MessageWriter<enum2$<country_core::systems::ui::resources::widget_params::PlayUiSound> >,bevy_ecs::system::query::Query<ref$<toolbar::resouces_and_components::UiLocked>,tuple$<> >,bevy_ecs::system::query::Query<ref$<toolbar::resouces_and_components::SwitchAppModeButton>,tuple$<> >,bevy_ecs::system::query::Query<ref$<toolbar::resouces_and_components::SwitchToolbarCategoryButton>,tuple$<> >,bevy_ecs::system::query::Query<ref$<toolbar::resouces_and_components::UiSlider>,tuple$<> >,bevy_ecs::system::query::Query<ref$<enum2$<toolbar::resouces_and_components::PhotomodeButton> >,tuple$<> >)>,tuple$<void (*)(bevy_ecs::system::query::Query<tuple$<ref$<toolbar::resouces_and_components::UiAnimationState>,ref_mut$<toolbar::resouces_and_components::TaffyNode>,enum2$<core::option::Option<ref$<toolbar::resouces_and_components::UiSlider> > >,enum2$<core::option::Option<ref$<toolbar::resouces_and_components::UiSliderReset> > >,enum2$<core::option::Option<ref$<toolbar::resouces_and_components::UiCustomSize> > > >,tuple$<> >),void (*)(bevy_ecs::system::query::Query<tuple$<ref$<toolbar::resouces_and_components::UiAnimationState>,ref_mut$<toolbar::resouces_and_components::TaffyNode> >,tuple$<> >),void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<toolbar::resouces_and_components::TaffyNode>,ref_mut$<toolbar::resouces_and_components::UiPlaySelectAnimation>,ref_mut$<country_core::components::UiInteractionState>,enum2$<core::option::Option<ref$<enum2$<toolbar::resouces_and_components::PhotomodeButton> > > >,enum2$<core::option::Option<ref$<toolbar::resouces_and_components::UiSliderReset> > > >,tuple$<> >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >),void (*)(bevy_ecs::message::message_reader::MessageReader<toolbar::ui_interaction::UiClickEvent>,bevy_ecs::system::query::Query<ref$<toolbar::resouces_and_components::UiLocked>,tuple$<> >,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<toolbar::resouces_and_components::UiPlayLockedAnimation>,ref_mut$<toolbar::resouces_and_components::TaffyNode> >,tuple$<> >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)> >", "constructor": "tuple$", "arguments": ["void (*)(bevy_ecs::change_detection::ResMut<tg_text::TextRenderer>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>)", "void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PhotomodeScreen>,bevy_ecs::change_detection::Res<country_core::startup::init_photomode_state::PhotomodeState>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::onboarding::WelcomeScreenState> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::widget_params::UiNavState>,bevy_ecs::change_detection::Res<country_core::startup::init_photomode_state::SteamdeckScreenshotMsgScreen>,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<country_core::components::UiInteractionState>,ref$<toolbar::resouces_and_components::HideIfMenuIsOpen>,enum2$<core::option::Option<ref$<toolbar::resouces_and_components::ShowOnlyInPhotomode> > > >,tuple$<> >)", "void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::components::Ui2dExtent>,ref$<country_core::components::UiMouseInteractable>,ref$<toolbar::resouces_and_components::TaffyNode>,ref_mut$<country_core::components::UiInteractionState>,enum2$<core::option::Option<ref$<country_core::components::HistoryButton> > >,enum2$<core::option::Option<ref$<country_core::components::UiNav> > > >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::widget_params::UiNavState>,bevy_ecs::change_detection::ResMut<toolbar::resouces_and_components::UiDirectFocus>,bevy_ecs::message::message_writer::MessageWriter<toolbar::ui_interaction::UiClickEvent>,bevy_ecs::message::message_writer::MessageWriter<toolbar::ui_interaction::UiDragEvent>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::backend::input::calculate_input_delta::UiHistoryButtonPressed>,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<toolbar::toolbar_update::ToolbarSizeMult>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::popup_bounds::UiPopupBounds>)", "void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<toolbar::resouces_and_components::UiShowIfParentIsInFocus>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::system::query::Query<ref_mut$<country_core::components::UiInteractionState>,bevy_ecs::query::filter::Without<toolbar::resouces_and_components::UiShowIfParentIsInFocus> >)", "void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::message::message_reader::MessageReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)", "void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,enum2$<core::option::Option<ref$<country_core::components::Ui2dExtent> > >,enum2$<core::option::Option<ref$<country_core::components::UiInteractionState> > >,ref$<country_core::components::UiNav> >,tuple$<> >,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<toolbar::resouces_and_components::TaffyNode>,ref$<country_core::components::UiNav> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<toolbar::resouces_and_components::taffy_layout::TaffyLayout>,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::widget_params::UiNavState>,bevy_ecs::message::message_writer::MessageWriter<toolbar::ui_interaction::UiClickEvent>)", "void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::message::message_writer::MessageWriter<toolbar::ui_interaction::UiJustBecameInFocusEvent>,bevy_ecs::system::system_param::Local<std::collections::hash::set::HashSet<bevy_ecs::entity::Entity,std::hash::random::RandomState,alloc::alloc::Global> >)", "void (*)(bevy_ecs::system::query::Query<ref$<country_core::components::UiInteractionState>,tuple$<> >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::toolbar::ToolbarUiIsInFocus>)", "tuple$<void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<toolbar::resouces_and_components::UiShowIfParentIsInFocus>,enum2$<core::option::Option<ref$<toolbar::resouces_and_components::ShowOnlyInFpvAndFree> > > >,tuple$<> >,bevy_ecs::system::query::Query<ref_mut$<country_core::components::UiInteractionState>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::startup::init_photomode_state::PhotomodeState>),void (*)(bevy_ecs::system::query::Query<tuple$<ref$<country_core::components::UiInteractionState>,ref_mut$<toolbar::resouces_and_components::UiAnimationState> >,tuple$<> >,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >),void (*)(bevy_ecs::message::message_reader::MessageReader<toolbar::ui_interaction::UiJustBecameInFocusEvent>,bevy_ecs::message::message_reader::MessageReader<toolbar::ui_interaction::UiClickEvent>,bevy_ecs::message::message_reader::MessageReader<toolbar::ui_interaction::UiDragEvent>,bevy_ecs::message::message_writer::MessageWriter<enum2$<country_core::systems::ui::resources::widget_params::PlayUiSound> >,bevy_ecs::system::query::Query<ref$<toolbar::resouces_and_components::UiLocked>,tuple$<> >,bevy_ecs::system::query::Query<ref$<toolbar::resouces_and_components::SwitchAppModeButton>,tuple$<> >,bevy_ecs::system::query::Query<ref$<toolbar::resouces_and_components::SwitchToolbarCategoryButton>,tuple$<> >,bevy_ecs::system::query::Query<ref$<toolbar::resouces_and_components::UiSlider>,tuple$<> >,bevy_ecs::system::query::Query<ref$<enum2$<toolbar::resouces_and_components::PhotomodeButton> >,tuple$<> >)>", "tuple$<void (*)(bevy_ecs::system::query::Query<tuple$<ref$<toolbar::resouces_and_components::UiAnimationState>,ref_mut$<toolbar::resouces_and_components::TaffyNode>,enum2$<core::option::Option<ref$<toolbar::resouces_and_components::UiSlider> > >,enum2$<core::option::Option<ref$<toolbar::resouces_and_components::UiSliderReset> > >,enum2$<core::option::Option<ref$<toolbar::resouces_and_components::UiCustomSize> > > >,tuple$<> >),void (*)(bevy_ecs::system::query::Query<tuple$<ref$<toolbar::resouces_and_components::UiAnimationState>,ref_mut$<toolbar::resouces_and_components::TaffyNode> >,tuple$<> >),void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<toolbar::resouces_and_components::TaffyNode>,ref_mut$<toolbar::resouces_and_components::UiPlaySelectAnimation>,ref_mut$<country_core::components::UiInteractionState>,enum2$<core::option::Option<ref$<enum2$<toolbar::resouces_and_components::PhotomodeButton> > > >,enum2$<core::option::Option<ref$<toolbar::resouces_and_components::UiSliderReset> > > >,tuple$<> >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >),void (*)(bevy_ecs::message::message_reader::MessageReader<toolbar::ui_interaction::UiClickEvent>,bevy_ecs::system::query::Query<ref$<toolbar::resouces_and_components::UiLocked>,tuple$<> >,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<toolbar::resouces_and_components::UiPlayLockedAnimation>,ref_mut$<toolbar::resouces_and_components::TaffyNode> >,tuple$<> >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::message::message_reader::MessageReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)>", "constructor": "tuple$", "arguments": ["bevy_ecs::system::function_system::IsFunctionSystem", "void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::message::message_reader::MessageReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<bevy_ecs::schedule::config::ScheduleConfigTupleMarker,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::ResMut<tg_text::TextRenderer>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>)>,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PhotomodeScreen>,bevy_ecs::change_detection::Res<country_core::startup::init_photomode_state::PhotomodeState>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::onboarding::WelcomeScreenState> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::widget_params::UiNavState>,bevy_ecs::change_detection::Res<country_core::startup::init_photomode_state::SteamdeckScreenshotMsgScreen>,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<country_core::components::UiInteractionState>,ref$<toolbar::resouces_and_components::HideIfMenuIsOpen>,enum2$<core::option::Option<ref$<toolbar::resouces_and_components::ShowOnlyInPhotomode> > > >,tuple$<> >)>,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::components::Ui2dExtent>,ref$<country_core::components::UiMouseInteractable>,ref$<toolbar::resouces_and_components::TaffyNode>,ref_mut$<country_core::components::UiInteractionState>,enum2$<core::option::Option<ref$<country_core::components::HistoryButton> > >,enum2$<core::option::Option<ref$<country_core::components::UiNav> > > >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::widget_params::UiNavState>,bevy_ecs::change_detection::ResMut<toolbar::resouces_and_components::UiDirectFocus>,bevy_ecs::message::message_writer::MessageWriter<toolbar::ui_interaction::UiClickEvent>,bevy_ecs::message::message_writer::MessageWriter<toolbar::ui_interaction::UiDragEvent>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::backend::input::calculate_input_delta::UiHistoryButtonPressed>,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<toolbar::toolbar_update::ToolbarSizeMult>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::popup_bounds::UiPopupBounds>)>,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<toolbar::resouces_and_components::UiShowIfParentIsInFocus>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::system::query::Query<ref_mut$<country_core::components::UiInteractionState>,bevy_ecs::query::filter::Without<toolbar::resouces_and_components::UiShowIfParentIsInFocus> >)>,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::message::message_reader::MessageReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)>,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,enum2$<core::option::Option<ref$<country_core::components::Ui2dExtent> > >,enum2$<core::option::Option<ref$<country_core::components::UiInteractionState> > >,ref$<country_core::components::UiNav> >,tuple$<> >,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<toolbar::resouces_and_components::TaffyNode>,ref$<country_core::components::UiNav> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<toolbar::resouces_and_components::taffy_layout::TaffyLayout>,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::widget_params::UiNavState>,bevy_ecs::message::message_writer::MessageWriter<toolbar::ui_interaction::UiClickEvent>)>,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::message::message_writer::MessageWriter<toolbar::ui_interaction::UiJustBecameInFocusEvent>,bevy_ecs::system::system_param::Local<std::collections::hash::set::HashSet<bevy_ecs::entity::Entity,std::hash::random::RandomState,alloc::alloc::Global> >)>,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::query::Query<ref$<country_core::components::UiInteractionState>,tuple$<> >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::toolbar::ToolbarUiIsInFocus>)>,tuple$<bevy_ecs::schedule::config::ScheduleConfigTupleMarker,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<toolbar::resouces_and_components::UiShowIfParentIsInFocus>,enum2$<core::option::Option<ref$<toolbar::resouces_and_components::ShowOnlyInFpvAndFree> > > >,tuple$<> >,bevy_ecs::system::query::Query<ref_mut$<country_core::components::UiInteractionState>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::startup::init_photomode_state::PhotomodeState>)>,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::query::Query<tuple$<ref$<country_core::components::UiInteractionState>,ref_mut$<toolbar::resouces_and_components::UiAnimationState> >,tuple$<> >,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)>,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::message::message_reader::MessageReader<toolbar::ui_interaction::UiJustBecameInFocusEvent>,bevy_ecs::message::message_reader::MessageReader<toolbar::ui_interaction::UiClickEvent>,bevy_ecs::message::message_reader::MessageReader<toolbar::ui_interaction::UiDragEvent>,bevy_ecs::message::message_writer::MessageWriter<enum2$<country_core::systems::ui::resources::widget_params::PlayUiSound> >,bevy_ecs::system::query::Query<ref$<toolbar::resouces_and_components::UiLocked>,tuple$<> >,bevy_ecs::system::query::Query<ref$<toolbar::resouces_and_components::SwitchAppModeButton>,tuple$<> >,bevy_ecs::system::query::Query<ref$<toolbar::resouces_and_components::SwitchToolbarCategoryButton>,tuple$<> >,bevy_ecs::system::query::Query<ref$<toolbar::resouces_and_components::UiSlider>,tuple$<> >,bevy_ecs::system::query::Query<ref$<enum2$<toolbar::resouces_and_components::PhotomodeButton> >,tuple$<> >)> >,tuple$<bevy_ecs::schedule::config::ScheduleConfigTupleMarker,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::query::Query<tuple$<ref$<toolbar::resouces_and_components::UiAnimationState>,ref_mut$<toolbar::resouces_and_components::TaffyNode>,enum2$<core::option::Option<ref$<toolbar::resouces_and_components::UiSlider> > >,enum2$<core::option::Option<ref$<toolbar::resouces_and_components::UiSliderReset> > >,enum2$<core::option::Option<ref$<toolbar::resouces_and_components::UiCustomSize> > > >,tuple$<> >)>,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::query::Query<tuple$<ref$<toolbar::resouces_and_components::UiAnimationState>,ref_mut$<toolbar::resouces_and_components::TaffyNode> >,tuple$<> >)>,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<toolbar::resouces_and_components::TaffyNode>,ref_mut$<toolbar::resouces_and_components::UiPlaySelectAnimation>,ref_mut$<country_core::components::UiInteractionState>,enum2$<core::option::Option<ref$<enum2$<toolbar::resouces_and_components::PhotomodeButton> > > >,enum2$<core::option::Option<ref$<toolbar::resouces_and_components::UiSliderReset> > > >,tuple$<> >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)>,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::message::message_reader::MessageReader<toolbar::ui_interaction::UiClickEvent>,bevy_ecs::system::query::Query<ref$<toolbar::resouces_and_components::UiLocked>,tuple$<> >,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<toolbar::resouces_and_components::UiPlayLockedAnimation>,ref_mut$<toolbar::resouces_and_components::TaffyNode> >,tuple$<> >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)> > >", "constructor": "tuple$", "arguments": ["bevy_ecs::schedule::config::ScheduleConfigTupleMarker", "tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::ResMut<tg_text::TextRenderer>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>)>", "tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PhotomodeScreen>,bevy_ecs::change_detection::Res<country_core::startup::init_photomode_state::PhotomodeState>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::onboarding::WelcomeScreenState> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::widget_params::UiNavState>,bevy_ecs::change_detection::Res<country_core::startup::init_photomode_state::SteamdeckScreenshotMsgScreen>,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<country_core::components::UiInteractionState>,ref$<toolbar::resouces_and_components::HideIfMenuIsOpen>,enum2$<core::option::Option<ref$<toolbar::resouces_and_components::ShowOnlyInPhotomode> > > >,tuple$<> >)>", "tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::components::Ui2dExtent>,ref$<country_core::components::UiMouseInteractable>,ref$<toolbar::resouces_and_components::TaffyNode>,ref_mut$<country_core::components::UiInteractionState>,enum2$<core::option::Option<ref$<country_core::components::HistoryButton> > >,enum2$<core::option::Option<ref$<country_core::components::UiNav> > > >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::widget_params::UiNavState>,bevy_ecs::change_detection::ResMut<toolbar::resouces_and_components::UiDirectFocus>,bevy_ecs::message::message_writer::MessageWriter<toolbar::ui_interaction::UiClickEvent>,bevy_ecs::message::message_writer::MessageWriter<toolbar::ui_interaction::UiDragEvent>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::backend::input::calculate_input_delta::UiHistoryButtonPressed>,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<toolbar::toolbar_update::ToolbarSizeMult>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::popup_bounds::UiPopupBounds>)>", "tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<toolbar::resouces_and_components::UiShowIfParentIsInFocus>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::system::query::Query<ref_mut$<country_core::components::UiInteractionState>,bevy_ecs::query::filter::Without<toolbar::resouces_and_components::UiShowIfParentIsInFocus> >)>", "tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::message::message_reader::MessageReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)>", "tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,enum2$<core::option::Option<ref$<country_core::components::Ui2dExtent> > >,enum2$<core::option::Option<ref$<country_core::components::UiInteractionState> > >,ref$<country_core::components::UiNav> >,tuple$<> >,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<toolbar::resouces_and_components::TaffyNode>,ref$<country_core::components::UiNav> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<toolbar::resouces_and_components::taffy_layout::TaffyLayout>,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::widget_params::UiNavState>,bevy_ecs::message::message_writer::MessageWriter<toolbar::ui_interaction::UiClickEvent>)>", "tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::message::message_writer::MessageWriter<toolbar::ui_interaction::UiJustBecameInFocusEvent>,bevy_ecs::system::system_param::Local<std::collections::hash::set::HashSet<bevy_ecs::entity::Entity,std::hash::random::RandomState,alloc::alloc::Global> >)>", "tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::query::Query<ref$<country_core::components::UiInteractionState>,tuple$<> >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::toolbar::ToolbarUiIsInFocus>)>", "tuple$<bevy_ecs::schedule::config::ScheduleConfigTupleMarker,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<toolbar::resouces_and_components::UiShowIfParentIsInFocus>,enum2$<core::option::Option<ref$<toolbar::resouces_and_components::ShowOnlyInFpvAndFree> > > >,tuple$<> >,bevy_ecs::system::query::Query<ref_mut$<country_core::components::UiInteractionState>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::startup::init_photomode_state::PhotomodeState>)>,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::query::Query<tuple$<ref$<country_core::components::UiInteractionState>,ref_mut$<toolbar::resouces_and_components::UiAnimationState> >,tuple$<> >,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)>,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::message::message_reader::MessageReader<toolbar::ui_interaction::UiJustBecameInFocusEvent>,bevy_ecs::message::message_reader::MessageReader<toolbar::ui_interaction::UiClickEvent>,bevy_ecs::message::message_reader::MessageReader<toolbar::ui_interaction::UiDragEvent>,bevy_ecs::message::message_writer::MessageWriter<enum2$<country_core::systems::ui::resources::widget_params::PlayUiSound> >,bevy_ecs::system::query::Query<ref$<toolbar::resouces_and_components::UiLocked>,tuple$<> >,bevy_ecs::system::query::Query<ref$<toolbar::resouces_and_components::SwitchAppModeButton>,tuple$<> >,bevy_ecs::system::query::Query<ref$<toolbar::resouces_and_components::SwitchToolbarCategoryButton>,tuple$<> >,bevy_ecs::system::query::Query<ref$<toolbar::resouces_and_components::UiSlider>,tuple$<> >,bevy_ecs::system::query::Query<ref$<enum2$<toolbar::resouces_and_components::PhotomodeButton> >,tuple$<> >)> >", "tuple$<bevy_ecs::schedule::config::ScheduleConfigTupleMarker,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::query::Query<tuple$<ref$<toolbar::resouces_and_components::UiAnimationState>,ref_mut$<toolbar::resouces_and_components::TaffyNode>,enum2$<core::option::Option<ref$<toolbar::resouces_and_components::UiSlider> > >,enum2$<core::option::Option<ref$<toolbar::resouces_and_components::UiSliderReset> > >,enum2$<core::option::Option<ref$<toolbar::resouces_and_components::UiCustomSize> > > >,tuple$<> >)>,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::query::Query<tuple$<ref$<toolbar::resouces_and_components::UiAnimationState>,ref_mut$<toolbar::resouces_and_components::TaffyNode> >,tuple$<> >)>,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<toolbar::resouces_and_components::TaffyNode>,ref_mut$<toolbar::resouces_and_components::UiPlaySelectAnimation>,ref_mut$<country_core::components::UiInteractionState>,enum2$<core::option::Option<ref$<enum2$<toolbar::resouces_and_components::PhotomodeButton> > > >,enum2$<core::option::Option<ref$<toolbar::resouces_and_components::UiSliderReset> > > >,tuple$<> >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)>,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::message::message_reader::MessageReader<toolbar::ui_interaction::UiClickEvent>,bevy_ecs::system::query::Query<ref$<toolbar::resouces_and_components::UiLocked>,tuple$<> >,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<toolbar::resouces_and_components::UiPlayLockedAnimation>,ref_mut$<toolbar::resouces_and_components::TaffyNode> >,tuple$<> >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::config::IntoScheduleConfigs::chain_ignore_deferred<tuple$<void (*)(bevy_ecs::change_detection::ResMut<tg_text::TextRenderer>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PhotomodeScreen>,bevy_ecs::change_detection::Res<country_core::startup::init_photomode_state::PhotomodeState>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::onboarding::WelcomeScreenState> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::widget_params::UiNavState>,bevy_ecs::change_detection::Res<country_core::startup::init_photomode_state::SteamdeckScreenshotMsgScreen>,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<country_core::components::UiInteractionState>,ref$<toolbar::resouces_and_components::HideIfMenuIsOpen>,enum2$<core::option::Option<ref$<toolbar::resouces_and_components::ShowOnlyInPhotomode> > > >,tuple$<> >),void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::components::Ui2dExtent>,ref$<country_core::components::UiMouseInteractable>,ref$<toolbar::resouces_and_components::TaffyNode>,ref_mut$<country_core::components::UiInteractionState>,enum2$<core::option::Option<ref$<country_core::components::HistoryButton> > >,enum2$<core::option::Option<ref$<country_core::components::UiNav> > > >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::widget_params::UiNavState>,bevy_ecs::change_detection::ResMut<toolbar::resouces_and_components::UiDirectFocus>,bevy_ecs::message::message_writer::MessageWriter<toolbar::ui_interaction::UiClickEvent>,bevy_ecs::message::message_writer::MessageWriter<toolbar::ui_interaction::UiDragEvent>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::backend::input::calculate_input_delta::UiHistoryButtonPressed>,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<toolbar::toolbar_update::ToolbarSizeMult>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::popup_bounds::UiPopupBounds>),void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<toolbar::resouces_and_components::UiShowIfParentIsInFocus>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::system::query::Query<ref_mut$<country_core::components::UiInteractionState>,bevy_ecs::query::filter::Without<toolbar::resouces_and_components::UiShowIfParentIsInFocus> >),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::message::message_reader::MessageReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >),void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,enum2$<core::option::Option<ref$<country_core::components::Ui2dExtent> > >,enum2$<core::option::Option<ref$<country_core::components::UiInteractionState> > >,ref$<country_core::components::UiNav> >,tuple$<> >,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<toolbar::resouces_and_components::TaffyNode>,ref$<country_core::components::UiNav> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<toolbar::resouces_and_components::taffy_layout::TaffyLayout>,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::widget_params::UiNavState>,bevy_ecs::message::message_writer::MessageWriter<toolbar::ui_interaction::UiClickEvent>),void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::message::message_writer::MessageWriter<toolbar::ui_interaction::UiJustBecameInFocusEvent>,bevy_ecs::system::system_param::Local<std::collections::hash::set::HashSet<bevy_ecs::entity::Entity,std::hash::random::RandomState,alloc::alloc::Global> >),void (*)(bevy_ecs::system::query::Query<ref$<country_core::components::UiInteractionState>,tuple$<> >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::toolbar::ToolbarUiIsInFocus>),tuple$<void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<toolbar::resouces_and_components::UiShowIfParentIsInFocus>,enum2$<core::option::Option<ref$<toolbar::resouces_and_components::ShowOnlyInFpvAndFree> > > >,tuple$<> >,bevy_ecs::system::query::Query<ref_mut$<country_core::components::UiInteractionState>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::startup::init_photomode_state::PhotomodeState>),void (*)(bevy_ecs::system::query::Query<tuple$<ref$<country_core::components::UiInteractionState>,ref_mut$<toolbar::resouces_and_components::UiAnimationState> >,tuple$<> >,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >),void (*)(bevy_ecs::message::message_reader::MessageReader<toolbar::ui_interaction::UiJustBecameInFocusEvent>,bevy_ecs::message::message_reader::MessageReader<toolbar::ui_interaction::UiClickEvent>,bevy_ecs::message::message_reader::MessageReader<toolbar::ui_interaction::UiDragEvent>,bevy_ecs::message::message_writer::MessageWriter<enum2$<country_core::systems::ui::resources::widget_params::PlayUiSound> >,bevy_ecs::system::query::Query<ref$<toolbar::resouces_and_components::UiLocked>,tuple$<> >,bevy_ecs::system::query::Query<ref$<toolbar::resouces_and_components::SwitchAppModeButton>,tuple$<> >,bevy_ecs::system::query::Query<ref$<toolbar::resouces_and_components::SwitchToolbarCategoryButton>,tuple$<> >,bevy_ecs::system::query::Query<ref$<toolbar::resouces_and_components::UiSlider>,tuple$<> >,bevy_ecs::system::query::Query<ref$<enum2$<toolbar::resouces_and_components::PhotomodeButton> >,tuple$<> >)>,tuple$<void (*)(bevy_ecs::system::query::Query<tuple$<ref$<toolbar::resouces_and_components::UiAnimationState>,ref_mut$<toolbar::resouces_and_components::TaffyNode>,enum2$<core::option::Option<ref$<toolbar::resouces_and_components::UiSlider> > >,enum2$<core::option::Option<ref$<toolbar::resouces_and_components::UiSliderReset> > >,enum2$<core::option::Option<ref$<toolbar::resouces_and_components::UiCustomSize> > > >,tuple$<> >),void (*)(bevy_ecs::system::query::Query<tuple$<ref$<toolbar::resouces_and_components::UiAnimationState>,ref_mut$<toolbar::resouces_and_components::TaffyNode> >,tuple$<> >),void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<toolbar::resouces_and_components::TaffyNode>,ref_mut$<toolbar::resouces_and_components::UiPlaySelectAnimation>,ref_mut$<country_core::components::UiInteractionState>,enum2$<core::option::Option<ref$<enum2$<toolbar::resouces_and_components::PhotomodeButton> > > >,enum2$<core::option::Option<ref$<toolbar::resouces_and_components::UiSliderReset> > > >,tuple$<> >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >),void (*)(bevy_ecs::message::message_reader::MessageReader<toolbar::ui_interaction::UiClickEvent>,bevy_ecs::system::query::Query<ref$<toolbar::resouces_and_components::UiLocked>,tuple$<> >,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<toolbar::resouces_and_components::UiPlayLockedAnimation>,ref_mut$<toolbar::resouces_and_components::TaffyNode> >,tuple$<> >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)> >,alloc::boxed::Box<dyn$<bevy_ecs::system::system::System<assoc$<In,tuple$<> >,assoc$<Out,tuple$<> > > >,alloc::alloc::Global>,tuple$<bevy_ecs::schedule::config::ScheduleConfigTupleMarker,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::ResMut<tg_text::TextRenderer>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>)>,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PhotomodeScreen>,bevy_ecs::change_detection::Res<country_core::startup::init_photomode_state::PhotomodeState>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::onboarding::WelcomeScreenState> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::widget_params::UiNavState>,bevy_ecs::change_detection::Res<country_core::startup::init_photomode_state::SteamdeckScreenshotMsgScreen>,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<country_core::components::UiInteractionState>,ref$<toolbar::resouces_and_components::HideIfMenuIsOpen>,enum2$<core::option::Option<ref$<toolbar::resouces_and_components::ShowOnlyInPhotomode> > > >,tuple$<> >)>,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::components::Ui2dExtent>,ref$<country_core::components::UiMouseInteractable>,ref$<toolbar::resouces_and_components::TaffyNode>,ref_mut$<country_core::components::UiInteractionState>,enum2$<core::option::Option<ref$<country_core::components::HistoryButton> > >,enum2$<core::option::Option<ref$<country_core::components::UiNav> > > >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::widget_params::UiNavState>,bevy_ecs::change_detection::ResMut<toolbar::resouces_and_components::UiDirectFocus>,bevy_ecs::message::message_writer::MessageWriter<toolbar::ui_interaction::UiClickEvent>,bevy_ecs::message::message_writer::MessageWriter<toolbar::ui_interaction::UiDragEvent>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::backend::input::calculate_input_delta::UiHistoryButtonPressed>,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<toolbar::toolbar_update::ToolbarSizeMult>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::popup_bounds::UiPopupBounds>)>,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<toolbar::resouces_and_components::UiShowIfParentIsInFocus>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::system::query::Query<ref_mut$<country_core::components::UiInteractionState>,bevy_ecs::query::filter::Without<toolbar::resouces_and_components::UiShowIfParentIsInFocus> >)>,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::message::message_reader::MessageReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)>,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,enum2$<core::option::Option<ref$<country_core::components::Ui2dExtent> > >,enum2$<core::option::Option<ref$<country_core::components::UiInteractionState> > >,ref$<country_core::components::UiNav> >,tuple$<> >,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<toolbar::resouces_and_components::TaffyNode>,ref$<country_core::components::UiNav> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<toolbar::resouces_and_components::taffy_layout::TaffyLayout>,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::widget_params::UiNavState>,bevy_ecs::message::message_writer::MessageWriter<toolbar::ui_interaction::UiClickEvent>)>,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::message::message_writer::MessageWriter<toolbar::ui_interaction::UiJustBecameInFocusEvent>,bevy_ecs::system::system_param::Local<std::collections::hash::set::HashSet<bevy_ecs::entity::Entity,std::hash::random::RandomState,alloc::alloc::Global> >)>,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::query::Query<ref$<country_core::components::UiInteractionState>,tuple$<> >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::toolbar::ToolbarUiIsInFocus>)>,tuple$<bevy_ecs::schedule::config::ScheduleConfigTupleMarker,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<toolbar::resouces_and_components::UiShowIfParentIsInFocus>,enum2$<core::option::Option<ref$<toolbar::resouces_and_components::ShowOnlyInFpvAndFree> > > >,tuple$<> >,bevy_ecs::system::query::Query<ref_mut$<country_core::components::UiInteractionState>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::startup::init_photomode_state::PhotomodeState>)>,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::query::Query<tuple$<ref$<country_core::components::UiInteractionState>,ref_mut$<toolbar::resouces_and_components::UiAnimationState> >,tuple$<> >,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)>,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::message::message_reader::MessageReader<toolbar::ui_interaction::UiJustBecameInFocusEvent>,bevy_ecs::message::message_reader::MessageReader<toolbar::ui_interaction::UiClickEvent>,bevy_ecs::message::message_reader::MessageReader<toolbar::ui_interaction::UiDragEvent>,bevy_ecs::message::message_writer::MessageWriter<enum2$<country_core::systems::ui::resources::widget_params::PlayUiSound> >,bevy_ecs::system::query::Query<ref$<toolbar::resouces_and_components::UiLocked>,tuple$<> >,bevy_ecs::system::query::Query<ref$<toolbar::resouces_and_components::SwitchAppModeButton>,tuple$<> >,bevy_ecs::system::query::Query<ref$<toolbar::resouces_and_components::SwitchToolbarCategoryButton>,tuple$<> >,bevy_ecs::system::query::Query<ref$<toolbar::resouces_and_components::UiSlider>,tuple$<> >,bevy_ecs::system::query::Query<ref$<enum2$<toolbar::resouces_and_components::PhotomodeButton> >,tuple$<> >)> >,tuple$<bevy_ecs::schedule::config::ScheduleConfigTupleMarker,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::query::Query<tuple$<ref$<toolbar::resouces_and_components::UiAnimationState>,ref_mut$<toolbar::resouces_and_components::TaffyNode>,enum2$<core::option::Option<ref$<toolbar::resouces_and_components::UiSlider> > >,enum2$<core::option::Option<ref$<toolbar::resouces_and_components::UiSliderReset> > >,enum2$<core::option::Option<ref$<toolbar::resouces_and_components::UiCustomSize> > > >,tuple$<> >)>,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::query::Query<tuple$<ref$<toolbar::resouces_and_components::UiAnimationState>,ref_mut$<toolbar::resouces_and_components::TaffyNode> >,tuple$<> >)>,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<toolbar::resouces_and_components::TaffyNode>,ref_mut$<toolbar::resouces_and_components::UiPlaySelectAnimation>,ref_mut$<country_core::components::UiInteractionState>,enum2$<core::option::Option<ref$<enum2$<toolbar::resouces_and_components::PhotomodeButton> > > >,enum2$<core::option::Option<ref$<toolbar::resouces_and_components::UiSliderReset> > > >,tuple$<> >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)>,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::message::message_reader::MessageReader<toolbar::ui_interaction::UiClickEvent>,bevy_ecs::system::query::Query<ref$<toolbar::resouces_and_components::UiLocked>,tuple$<> >,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<toolbar::resouces_and_components::UiPlayLockedAnimation>,ref_mut$<toolbar::resouces_and_components::TaffyNode> >,tuple$<> >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)> > > >", "constructor": "bevy_ecs::schedule::config::IntoScheduleConfigs::chain_ignore_deferred", "arguments": ["tuple$<void (*)(bevy_ecs::change_detection::ResMut<tg_text::TextRenderer>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PhotomodeScreen>,bevy_ecs::change_detection::Res<country_core::startup::init_photomode_state::PhotomodeState>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::onboarding::WelcomeScreenState> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::widget_params::UiNavState>,bevy_ecs::change_detection::Res<country_core::startup::init_photomode_state::SteamdeckScreenshotMsgScreen>,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<country_core::components::UiInteractionState>,ref$<toolbar::resouces_and_components::HideIfMenuIsOpen>,enum2$<core::option::Option<ref$<toolbar::resouces_and_components::ShowOnlyInPhotomode> > > >,tuple$<> >),void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::components::Ui2dExtent>,ref$<country_core::components::UiMouseInteractable>,ref$<toolbar::resouces_and_components::TaffyNode>,ref_mut$<country_core::components::UiInteractionState>,enum2$<core::option::Option<ref$<country_core::components::HistoryButton> > >,enum2$<core::option::Option<ref$<country_core::components::UiNav> > > >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::widget_params::UiNavState>,bevy_ecs::change_detection::ResMut<toolbar::resouces_and_components::UiDirectFocus>,bevy_ecs::message::message_writer::MessageWriter<toolbar::ui_interaction::UiClickEvent>,bevy_ecs::message::message_writer::MessageWriter<toolbar::ui_interaction::UiDragEvent>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::backend::input::calculate_input_delta::UiHistoryButtonPressed>,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<toolbar::toolbar_update::ToolbarSizeMult>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::popup_bounds::UiPopupBounds>),void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<toolbar::resouces_and_components::UiShowIfParentIsInFocus>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::system::query::Query<ref_mut$<country_core::components::UiInteractionState>,bevy_ecs::query::filter::Without<toolbar::resouces_and_components::UiShowIfParentIsInFocus> >),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::message::message_reader::MessageReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >),void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,enum2$<core::option::Option<ref$<country_core::components::Ui2dExtent> > >,enum2$<core::option::Option<ref$<country_core::components::UiInteractionState> > >,ref$<country_core::components::UiNav> >,tuple$<> >,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<toolbar::resouces_and_components::TaffyNode>,ref$<country_core::components::UiNav> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<toolbar::resouces_and_components::taffy_layout::TaffyLayout>,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::widget_params::UiNavState>,bevy_ecs::message::message_writer::MessageWriter<toolbar::ui_interaction::UiClickEvent>),void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::message::message_writer::MessageWriter<toolbar::ui_interaction::UiJustBecameInFocusEvent>,bevy_ecs::system::system_param::Local<std::collections::hash::set::HashSet<bevy_ecs::entity::Entity,std::hash::random::RandomState,alloc::alloc::Global> >),void (*)(bevy_ecs::system::query::Query<ref$<country_core::components::UiInteractionState>,tuple$<> >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::toolbar::ToolbarUiIsInFocus>),tuple$<void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<toolbar::resouces_and_components::UiShowIfParentIsInFocus>,enum2$<core::option::Option<ref$<toolbar::resouces_and_components::ShowOnlyInFpvAndFree> > > >,tuple$<> >,bevy_ecs::system::query::Query<ref_mut$<country_core::components::UiInteractionState>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::startup::init_photomode_state::PhotomodeState>),void (*)(bevy_ecs::system::query::Query<tuple$<ref$<country_core::components::UiInteractionState>,ref_mut$<toolbar::resouces_and_components::UiAnimationState> >,tuple$<> >,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >),void (*)(bevy_ecs::message::message_reader::MessageReader<toolbar::ui_interaction::UiJustBecameInFocusEvent>,bevy_ecs::message::message_reader::MessageReader<toolbar::ui_interaction::UiClickEvent>,bevy_ecs::message::message_reader::MessageReader<toolbar::ui_interaction::UiDragEvent>,bevy_ecs::message::message_writer::MessageWriter<enum2$<country_core::systems::ui::resources::widget_params::PlayUiSound> >,bevy_ecs::system::query::Query<ref$<toolbar::resouces_and_components::UiLocked>,tuple$<> >,bevy_ecs::system::query::Query<ref$<toolbar::resouces_and_components::SwitchAppModeButton>,tuple$<> >,bevy_ecs::system::query::Query<ref$<toolbar::resouces_and_components::SwitchToolbarCategoryButton>,tuple$<> >,bevy_ecs::system::query::Query<ref$<toolbar::resouces_and_components::UiSlider>,tuple$<> >,bevy_ecs::system::query::Query<ref$<enum2$<toolbar::resouces_and_components::PhotomodeButton> >,tuple$<> >)>,tuple$<void (*)(bevy_ecs::system::query::Query<tuple$<ref$<toolbar::resouces_and_components::UiAnimationState>,ref_mut$<toolbar::resouces_and_components::TaffyNode>,enum2$<core::option::Option<ref$<toolbar::resouces_and_components::UiSlider> > >,enum2$<core::option::Option<ref$<toolbar::resouces_and_components::UiSliderReset> > >,enum2$<core::option::Option<ref$<toolbar::resouces_and_components::UiCustomSize> > > >,tuple$<> >),void (*)(bevy_ecs::system::query::Query<tuple$<ref$<toolbar::resouces_and_components::UiAnimationState>,ref_mut$<toolbar::resouces_and_components::TaffyNode> >,tuple$<> >),void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<toolbar::resouces_and_components::TaffyNode>,ref_mut$<toolbar::resouces_and_components::UiPlaySelectAnimation>,ref_mut$<country_core::components::UiInteractionState>,enum2$<core::option::Option<ref$<enum2$<toolbar::resouces_and_components::PhotomodeButton> > > >,enum2$<core::option::Option<ref$<toolbar::resouces_and_components::UiSliderReset> > > >,tuple$<> >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >),void (*)(bevy_ecs::message::message_reader::MessageReader<toolbar::ui_interaction::UiClickEvent>,bevy_ecs::system::query::Query<ref$<toolbar::resouces_and_components::UiLocked>,tuple$<> >,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<toolbar::resouces_and_components::UiPlayLockedAnimation>,ref_mut$<toolbar::resouces_and_components::TaffyNode> >,tuple$<> >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)> >", "alloc::boxed::Box<dyn$<bevy_ecs::system::system::System<assoc$<In,tuple$<> >,assoc$<Out,tuple$<> > > >,alloc::alloc::Global>", "tuple$<bevy_ecs::schedule::config::ScheduleConfigTupleMarker,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::ResMut<tg_text::TextRenderer>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>)>,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PhotomodeScreen>,bevy_ecs::change_detection::Res<country_core::startup::init_photomode_state::PhotomodeState>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::onboarding::WelcomeScreenState> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::widget_params::UiNavState>,bevy_ecs::change_detection::Res<country_core::startup::init_photomode_state::SteamdeckScreenshotMsgScreen>,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<country_core::components::UiInteractionState>,ref$<toolbar::resouces_and_components::HideIfMenuIsOpen>,enum2$<core::option::Option<ref$<toolbar::resouces_and_components::ShowOnlyInPhotomode> > > >,tuple$<> >)>,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::components::Ui2dExtent>,ref$<country_core::components::UiMouseInteractable>,ref$<toolbar::resouces_and_components::TaffyNode>,ref_mut$<country_core::components::UiInteractionState>,enum2$<core::option::Option<ref$<country_core::components::HistoryButton> > >,enum2$<core::option::Option<ref$<country_core::components::UiNav> > > >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::widget_params::UiNavState>,bevy_ecs::change_detection::ResMut<toolbar::resouces_and_components::UiDirectFocus>,bevy_ecs::message::message_writer::MessageWriter<toolbar::ui_interaction::UiClickEvent>,bevy_ecs::message::message_writer::MessageWriter<toolbar::ui_interaction::UiDragEvent>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::backend::input::calculate_input_delta::UiHistoryButtonPressed>,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<toolbar::toolbar_update::ToolbarSizeMult>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::popup_bounds::UiPopupBounds>)>,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<toolbar::resouces_and_components::UiShowIfParentIsInFocus>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::system::query::Query<ref_mut$<country_core::components::UiInteractionState>,bevy_ecs::query::filter::Without<toolbar::resouces_and_components::UiShowIfParentIsInFocus> >)>,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::message::message_reader::MessageReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)>,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,enum2$<core::option::Option<ref$<country_core::components::Ui2dExtent> > >,enum2$<core::option::Option<ref$<country_core::components::UiInteractionState> > >,ref$<country_core::components::UiNav> >,tuple$<> >,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<toolbar::resouces_and_components::TaffyNode>,ref$<country_core::components::UiNav> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<toolbar::resouces_and_components::taffy_layout::TaffyLayout>,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::widget_params::UiNavState>,bevy_ecs::message::message_writer::MessageWriter<toolbar::ui_interaction::UiClickEvent>)>,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::message::message_writer::MessageWriter<toolbar::ui_interaction::UiJustBecameInFocusEvent>,bevy_ecs::system::system_param::Local<std::collections::hash::set::HashSet<bevy_ecs::entity::Entity,std::hash::random::RandomState,alloc::alloc::Global> >)>,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::query::Query<ref$<country_core::components::UiInteractionState>,tuple$<> >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::toolbar::ToolbarUiIsInFocus>)>,tuple$<bevy_ecs::schedule::config::ScheduleConfigTupleMarker,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<toolbar::resouces_and_components::UiShowIfParentIsInFocus>,enum2$<core::option::Option<ref$<toolbar::resouces_and_components::ShowOnlyInFpvAndFree> > > >,tuple$<> >,bevy_ecs::system::query::Query<ref_mut$<country_core::components::UiInteractionState>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::startup::init_photomode_state::PhotomodeState>)>,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::query::Query<tuple$<ref$<country_core::components::UiInteractionState>,ref_mut$<toolbar::resouces_and_components::UiAnimationState> >,tuple$<> >,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)>,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::message::message_reader::MessageReader<toolbar::ui_interaction::UiJustBecameInFocusEvent>,bevy_ecs::message::message_reader::MessageReader<toolbar::ui_interaction::UiClickEvent>,bevy_ecs::message::message_reader::MessageReader<toolbar::ui_interaction::UiDragEvent>,bevy_ecs::message::message_writer::MessageWriter<enum2$<country_core::systems::ui::resources::widget_params::PlayUiSound> >,bevy_ecs::system::query::Query<ref$<toolbar::resouces_and_components::UiLocked>,tuple$<> >,bevy_ecs::system::query::Query<ref$<toolbar::resouces_and_components::SwitchAppModeButton>,tuple$<> >,bevy_ecs::system::query::Query<ref$<toolbar::resouces_and_components::SwitchToolbarCategoryButton>,tuple$<> >,bevy_ecs::system::query::Query<ref$<toolbar::resouces_and_components::UiSlider>,tuple$<> >,bevy_ecs::system::query::Query<ref$<enum2$<toolbar::resouces_and_components::PhotomodeButton> >,tuple$<> >)> >,tuple$<bevy_ecs::schedule::config::ScheduleConfigTupleMarker,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::query::Query<tuple$<ref$<toolbar::resouces_and_components::UiAnimationState>,ref_mut$<toolbar::resouces_and_components::TaffyNode>,enum2$<core::option::Option<ref$<toolbar::resouces_and_components::UiSlider> > >,enum2$<core::option::Option<ref$<toolbar::resouces_and_components::UiSliderReset> > >,enum2$<core::option::Option<ref$<toolbar::resouces_and_components::UiCustomSize> > > >,tuple$<> >)>,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::query::Query<tuple$<ref$<toolbar::resouces_and_components::UiAnimationState>,ref_mut$<toolbar::resouces_and_components::TaffyNode> >,tuple$<> >)>,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<toolbar::resouces_and_components::TaffyNode>,ref_mut$<toolbar::resouces_and_components::UiPlaySelectAnimation>,ref_mut$<country_core::components::UiInteractionState>,enum2$<core::option::Option<ref$<enum2$<toolbar::resouces_and_components::PhotomodeButton> > > >,enum2$<core::option::Option<ref$<toolbar::resouces_and_components::UiSliderReset> > > >,tuple$<> >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)>,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::message::message_reader::MessageReader<toolbar::ui_interaction::UiClickEvent>,bevy_ecs::system::query::Query<ref$<toolbar::resouces_and_components::UiLocked>,tuple$<> >,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<toolbar::resouces_and_components::UiPlayLockedAnimation>,ref_mut$<toolbar::resouces_and_components::TaffyNode> >,tuple$<> >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::message::messages::Messages<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>", "constructor": "bevy_ecs::message::messages::Messages", "arguments": ["toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system_param::impl$7::init_state<bevy_ecs::message::messages::Messages<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns> >", "constructor": "bevy_ecs::system::system_param::impl$7::init_state", "arguments": ["bevy_ecs::message::messages::Messages<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::default_system_sets<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::message::message_reader::MessageReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >),tuple$<>,void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::message::message_reader::MessageReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)>", "constructor": "bevy_ecs::system::function_system::impl$12::default_system_sets", "arguments": ["void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::message::message_reader::MessageReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)", "tuple$<>", "void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::message::message_reader::MessageReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::validate_param<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::message::message_reader::MessageReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >),tuple$<>,void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::message::message_reader::MessageReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)> >", "constructor": "bevy_ecs::system::system::System::validate_param", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::message::message_reader::MessageReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >),tuple$<>,void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::message::message_reader::MessageReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::message::message_reader::MessageReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >),tuple$<>,void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::message::message_reader::MessageReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)> >", "constructor": "bevy_ecs::system::system::System::run_without_applying_deferred", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::message::message_reader::MessageReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >),tuple$<>,void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::message::message_reader::MessageReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::message::message_reader::MessageReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >),tuple$<>,void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::message::message_reader::MessageReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)> >", "constructor": "bevy_ecs::system::system::System::run", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::message::message_reader::MessageReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >),tuple$<>,void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::message::message_reader::MessageReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::initialize<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::message::message_reader::MessageReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >),tuple$<>,void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::message::message_reader::MessageReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)>", "constructor": "bevy_ecs::system::function_system::impl$12::initialize", "arguments": ["void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::message::message_reader::MessageReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)", "tuple$<>", "void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::message::message_reader::MessageReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::run_unsafe<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::message::message_reader::MessageReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >),tuple$<>,void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::message::message_reader::MessageReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)>", "constructor": "bevy_ecs::system::function_system::impl$12::run_unsafe", "arguments": ["void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::message::message_reader::MessageReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)", "tuple$<>", "void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::message::message_reader::MessageReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::get_last_run<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::message::message_reader::MessageReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >),tuple$<>,void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::message::message_reader::MessageReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)>", "constructor": "bevy_ecs::system::function_system::impl$12::get_last_run", "arguments": ["void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::message::message_reader::MessageReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)", "tuple$<>", "void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::message::message_reader::MessageReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::set_last_run<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::message::message_reader::MessageReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >),tuple$<>,void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::message::message_reader::MessageReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)>", "constructor": "bevy_ecs::system::function_system::impl$12::set_last_run", "arguments": ["void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::message::message_reader::MessageReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)", "tuple$<>", "void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::message::message_reader::MessageReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::apply_deferred<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::message::message_reader::MessageReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >),tuple$<>,void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::message::message_reader::MessageReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)>", "constructor": "bevy_ecs::system::function_system::impl$12::apply_deferred", "arguments": ["void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::message::message_reader::MessageReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)", "tuple$<>", "void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::message::message_reader::MessageReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::queue_deferred<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::message::message_reader::MessageReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >),tuple$<>,void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::message::message_reader::MessageReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)>", "constructor": "bevy_ecs::system::function_system::impl$12::queue_deferred", "arguments": ["void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::message::message_reader::MessageReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)", "tuple$<>", "void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::message::message_reader::MessageReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::check_change_tick<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::message::message_reader::MessageReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >),tuple$<>,void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::message::message_reader::MessageReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)>", "constructor": "bevy_ecs::system::function_system::impl$12::check_change_tick", "arguments": ["void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::message::message_reader::MessageReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)", "tuple$<>", "void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::message::message_reader::MessageReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::validate_param_unsafe<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::message::message_reader::MessageReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >),tuple$<>,void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::message::message_reader::MessageReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)>", "constructor": "bevy_ecs::system::function_system::impl$12::validate_param_unsafe", "arguments": ["void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::message::message_reader::MessageReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)", "tuple$<>", "void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::message::message_reader::MessageReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::name<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::message::message_reader::MessageReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >),tuple$<>,void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::message::message_reader::MessageReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)>", "constructor": "bevy_ecs::system::function_system::impl$12::name", "arguments": ["void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::message::message_reader::MessageReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)", "tuple$<>", "void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::message::message_reader::MessageReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::flags<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::message::message_reader::MessageReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >),tuple$<>,void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::message::message_reader::MessageReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)>", "constructor": "bevy_ecs::system::function_system::impl$12::flags", "arguments": ["void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::message::message_reader::MessageReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)", "tuple$<>", "void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::message::message_reader::MessageReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::has_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::message::message_reader::MessageReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >),tuple$<>,void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::message::message_reader::MessageReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)> >", "constructor": "bevy_ecs::system::system::System::has_deferred", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::message::message_reader::MessageReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >),tuple$<>,void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::message::message_reader::MessageReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::is_exclusive<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::message::message_reader::MessageReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >),tuple$<>,void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::message::message_reader::MessageReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)> >", "constructor": "bevy_ecs::system::system::System::is_exclusive", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::message::message_reader::MessageReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >),tuple$<>,void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::message::message_reader::MessageReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::is_send<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::message::message_reader::MessageReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >),tuple$<>,void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::message::message_reader::MessageReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)> >", "constructor": "bevy_ecs::system::system::System::is_send", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::message::message_reader::MessageReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >),tuple$<>,void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::message::message_reader::MessageReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::type_id<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::message::message_reader::MessageReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >),tuple$<>,void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::message::message_reader::MessageReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)> >", "constructor": "bevy_ecs::system::system::System::type_id", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::message::message_reader::MessageReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >),tuple$<>,void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::message::message_reader::MessageReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system_param::impl$108::init_access<bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::message::message_reader::MessageReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > > >", "constructor": "bevy_ecs::system::system_param::impl$108::init_access", "arguments": ["bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >", "bevy_ecs::message::message_reader::MessageReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>", "bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
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
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 5; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::query::Query<tuple$<ref$<toolbar::resouces_and_components::TimeOfDayButtonV2>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::message::message_reader::MessageReader<toolbar::ui_interaction::UiClickEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::TimeOfDayScreen>,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::widget_params::UiNavState>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::mode_manager::RequestAppModeChange>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::system::query::Query<tuple$<ref$<toolbar::resouces_and_components::TimeOfDayButtonV2>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >", "bevy_ecs::message::message_reader::MessageReader<toolbar::ui_interaction::UiClickEvent>", "bevy_ecs::change_detection::ResMut<country_core::resources::gui::TimeOfDayScreen>", "bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::widget_params::UiNavState>", "bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >", "bevy_ecs::message::message_writer::MessageWriter<country_core::systems::mode_manager::RequestAppModeChange>", "bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::ModeJustChanged>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<ref$<toolbar::resouces_and_components::TimeOfDayButtonV2>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<toolbar::ui_interaction::UiClickEvent>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::gui::TimeOfDayScreen>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::widget_params::UiNavState>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::systems::mode_manager::RequestAppModeChange>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::ModeJustChanged>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<toolbar::resouces_and_components::SwitchAppModeButton>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::message::message_reader::MessageReader<toolbar::ui_interaction::UiClickEvent>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::mode_manager::RequestAppModeChange>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<toolbar::resouces_and_components::SwitchAppModeButton>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >", "bevy_ecs::message::message_reader::MessageReader<toolbar::ui_interaction::UiClickEvent>", "bevy_ecs::message::message_writer::MessageWriter<country_core::systems::mode_manager::RequestAppModeChange>", "bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >", "bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::ModeJustChanged>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<toolbar::resouces_and_components::SwitchAppModeButton>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<toolbar::ui_interaction::UiClickEvent>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::systems::mode_manager::RequestAppModeChange>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::ModeJustChanged>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::message::message_reader::MessageReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)
// 候选参数类型: ["bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >", "bevy_ecs::message::message_reader::MessageReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>", "bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >"]
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<toolbar::resouces_and_components::SwitchToolbarCategoryButton>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::message::message_reader::MessageReader<toolbar::ui_interaction::UiClickEvent>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::mode_manager::RequestToolbarCategoryChange>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::change_detection::Res<country_core::resources::toolbar::Toolbar>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<toolbar::resouces_and_components::SwitchToolbarCategoryButton>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >", "bevy_ecs::message::message_reader::MessageReader<toolbar::ui_interaction::UiClickEvent>", "bevy_ecs::message::message_writer::MessageWriter<country_core::systems::mode_manager::RequestToolbarCategoryChange>", "bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::ModeJustChanged>", "bevy_ecs::change_detection::Res<country_core::resources::toolbar::Toolbar>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<toolbar::resouces_and_components::SwitchToolbarCategoryButton>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<toolbar::ui_interaction::UiClickEvent>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::systems::mode_manager::RequestToolbarCategoryChange>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::ModeJustChanged>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::toolbar::Toolbar>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<toolbar::resouces_and_components::ToggleBuildingMaterialButton>,ref_mut$<country_core::components::Ui2dImage> >,tuple$<> >,bevy_ecs::system::query::Query<tuple$<ref$<toolbar::resouces_and_components::SwitchAppModeButton>,ref_mut$<country_core::components::Ui2dImage> >,bevy_ecs::query::filter::Without<toolbar::resouces_and_components::ToggleBuildingMaterialButton> >,bevy_ecs::message::message_reader::MessageReader<toolbar::ui_interaction::UiClickEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::toolbar::BuildingPlacementMaterial>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<toolbar::resouces_and_components::ToggleBuildingMaterialButton>,ref_mut$<country_core::components::Ui2dImage> >,tuple$<> >", "bevy_ecs::system::query::Query<tuple$<ref$<toolbar::resouces_and_components::SwitchAppModeButton>,ref_mut$<country_core::components::Ui2dImage> >,bevy_ecs::query::filter::Without<toolbar::resouces_and_components::ToggleBuildingMaterialButton> >", "bevy_ecs::message::message_reader::MessageReader<toolbar::ui_interaction::UiClickEvent>", "bevy_ecs::change_detection::ResMut<country_core::resources::toolbar::BuildingPlacementMaterial>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<toolbar::resouces_and_components::ToggleBuildingMaterialButton>,ref_mut$<country_core::components::Ui2dImage> >,tuple$<> >
// [候选系统接口] query_without_filter: bevy_ecs::query::filter::Without<toolbar::resouces_and_components::ToggleBuildingMaterialButton>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<ref$<toolbar::resouces_and_components::SwitchAppModeButton>,ref_mut$<country_core::components::Ui2dImage> >,bevy_ecs::query::filter::Without<toolbar::resouces_and_components::ToggleBuildingMaterialButton> >
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<toolbar::ui_interaction::UiClickEvent>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::toolbar::BuildingPlacementMaterial>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands

// ------------------------------------------------------------------
// 函数: toolbar::toolbar_button::switch_app_mode_button
// 信息 ID: a79ff54e4f167ee6d4e22ec8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] b6b4f6597e31860bc03d677a: void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<toolbar::resouces_and_components::SwitchAppModeButton>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::message::message_reader::MessageReader<toolbar::ui_interaction::UiClickEvent>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::mode_manager::RequestAppModeChange>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::system::commands::Commands)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<toolbar::resouces_and_components::SwitchAppModeButton>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::message::message_reader::MessageReader<toolbar::ui_interaction::UiClickEvent>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::mode_manager::RequestAppModeChange>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::system::commands::Commands),tuple$<>,void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<toolbar::resouces_and_components::SwitchAppModeButton>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::message::message_reader::MessageReader<toolbar::ui_interaction::UiClickEvent>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::mode_manager::RequestAppModeChange>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::system::commands::Commands)> > @ 0x2313720 -> 0x29651b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$12::run_unsafe<void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<toolbar::resouces_and_components::SwitchAppModeButton>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::message::message_reader::MessageReader<toolbar::ui_interaction::UiClickEvent>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::mode_manager::RequestAppModeChange>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::system::commands::Commands),tuple$<>,void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<toolbar::resouces_and_components::SwitchAppModeButton>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::message::message_reader::MessageReader<toolbar::ui_interaction::UiClickEvent>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::mode_manager::RequestAppModeChange>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::system::commands::Commands)> @ 0x23af48c -> 0x29651b0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::toolbar_button::switch_toolbar_category_button
// 信息 ID: 8e37c09503f6d9b0f189bbe0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] c2e10a2f4d28e68750e2da78: void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<toolbar::resouces_and_components::SwitchToolbarCategoryButton>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::message::message_reader::MessageReader<toolbar::ui_interaction::UiClickEvent>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::mode_manager::RequestToolbarCategoryChange>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::change_detection::Res<country_core::resources::toolbar::Toolbar>,bevy_ecs::system::commands::Commands)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<toolbar::resouces_and_components::SwitchToolbarCategoryButton>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::message::message_reader::MessageReader<toolbar::ui_interaction::UiClickEvent>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::mode_manager::RequestToolbarCategoryChange>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::change_detection::Res<country_core::resources::toolbar::Toolbar>,bevy_ecs::system::commands::Commands),tuple$<>,void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<toolbar::resouces_and_components::SwitchToolbarCategoryButton>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::message::message_reader::MessageReader<toolbar::ui_interaction::UiClickEvent>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::mode_manager::RequestToolbarCategoryChange>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::change_detection::Res<country_core::resources::toolbar::Toolbar>,bevy_ecs::system::commands::Commands)> > @ 0x230aa50 -> 0x2966450 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$12::run_unsafe<void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<toolbar::resouces_and_components::SwitchToolbarCategoryButton>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::message::message_reader::MessageReader<toolbar::ui_interaction::UiClickEvent>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::mode_manager::RequestToolbarCategoryChange>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::change_detection::Res<country_core::resources::toolbar::Toolbar>,bevy_ecs::system::commands::Commands),tuple$<>,void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<toolbar::resouces_and_components::SwitchToolbarCategoryButton>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::message::message_reader::MessageReader<toolbar::ui_interaction::UiClickEvent>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::mode_manager::RequestToolbarCategoryChange>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::change_detection::Res<country_core::resources::toolbar::Toolbar>,bevy_ecs::system::commands::Commands)> @ 0x23aec0c -> 0x2966450 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::toolbar_button::tod_v2_button
// 信息 ID: 80afef813770297258d62f98
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 6c5ef8e8d7fdd69572515b8a: void (*)(bevy_ecs::system::query::Query<tuple$<ref$<toolbar::resouces_and_components::TimeOfDayButtonV2>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::message::message_reader::MessageReader<toolbar::ui_interaction::UiClickEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::TimeOfDayScreen>,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::widget_params::UiNavState>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::mode_manager::RequestAppModeChange>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::system::commands::Commands)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref$<toolbar::resouces_and_components::TimeOfDayButtonV2>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::message::message_reader::MessageReader<toolbar::ui_interaction::UiClickEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::TimeOfDayScreen>,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::widget_params::UiNavState>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::mode_manager::RequestAppModeChange>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::system::commands::Commands),tuple$<>,void (*)(bevy_ecs::system::query::Query<tuple$<ref$<toolbar::resouces_and_components::TimeOfDayButtonV2>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::message::message_reader::MessageReader<toolbar::ui_interaction::UiClickEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::TimeOfDayScreen>,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::widget_params::UiNavState>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::mode_manager::RequestAppModeChange>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::system::commands::Commands)> > @ 0x2316316 -> 0x2964bb0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$12::run_unsafe<void (*)(bevy_ecs::system::query::Query<tuple$<ref$<toolbar::resouces_and_components::TimeOfDayButtonV2>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::message::message_reader::MessageReader<toolbar::ui_interaction::UiClickEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::TimeOfDayScreen>,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::widget_params::UiNavState>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::mode_manager::RequestAppModeChange>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::system::commands::Commands),tuple$<>,void (*)(bevy_ecs::system::query::Query<tuple$<ref$<toolbar::resouces_and_components::TimeOfDayButtonV2>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::message::message_reader::MessageReader<toolbar::ui_interaction::UiClickEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::TimeOfDayScreen>,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::widget_params::UiNavState>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::mode_manager::RequestAppModeChange>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::system::commands::Commands)> @ 0x23ae872 -> 0x2964bb0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::toolbar_button::toggle_building_material_button
// 信息 ID: 488c562b2be705ee8d4879c0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 6f9eb33c76a93549b54b986d: void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<toolbar::resouces_and_components::ToggleBuildingMaterialButton>,ref_mut$<country_core::components::Ui2dImage> >,tuple$<> >,bevy_ecs::system::query::Query<tuple$<ref$<toolbar::resouces_and_components::SwitchAppModeButton>,ref_mut$<country_core::components::Ui2dImage> >,bevy_ecs::query::filter::Without<toolbar::resouces_and_components::ToggleBuildingMaterialButton> >,bevy_ecs::message::message_reader::MessageReader<toolbar::ui_interaction::UiClickEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::toolbar::BuildingPlacementMaterial>,bevy_ecs::system::commands::Commands)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<toolbar::resouces_and_components::ToggleBuildingMaterialButton>,ref_mut$<country_core::components::Ui2dImage> >,tuple$<> >,bevy_ecs::system::query::Query<tuple$<ref$<toolbar::resouces_and_components::SwitchAppModeButton>,ref_mut$<country_core::components::Ui2dImage> >,bevy_ecs::query::filter::Without<toolbar::resouces_and_components::ToggleBuildingMaterialButton> >,bevy_ecs::message::message_reader::MessageReader<toolbar::ui_interaction::UiClickEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::toolbar::BuildingPlacementMaterial>,bevy_ecs::system::commands::Commands),tuple$<>,void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<toolbar::resouces_and_components::ToggleBuildingMaterialButton>,ref_mut$<country_core::components::Ui2dImage> >,tuple$<> >,bevy_ecs::system::query::Query<tuple$<ref$<toolbar::resouces_and_components::SwitchAppModeButton>,ref_mut$<country_core::components::Ui2dImage> >,bevy_ecs::query::filter::Without<toolbar::resouces_and_components::ToggleBuildingMaterialButton> >,bevy_ecs::message::message_reader::MessageReader<toolbar::ui_interaction::UiClickEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::toolbar::BuildingPlacementMaterial>,bevy_ecs::system::commands::Commands)> > @ 0x23158fd -> 0x2966b70 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$12::run_unsafe<void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<toolbar::resouces_and_components::ToggleBuildingMaterialButton>,ref_mut$<country_core::components::Ui2dImage> >,tuple$<> >,bevy_ecs::system::query::Query<tuple$<ref$<toolbar::resouces_and_components::SwitchAppModeButton>,ref_mut$<country_core::components::Ui2dImage> >,bevy_ecs::query::filter::Without<toolbar::resouces_and_components::ToggleBuildingMaterialButton> >,bevy_ecs::message::message_reader::MessageReader<toolbar::ui_interaction::UiClickEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::toolbar::BuildingPlacementMaterial>,bevy_ecs::system::commands::Commands),tuple$<>,void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<toolbar::resouces_and_components::ToggleBuildingMaterialButton>,ref_mut$<country_core::components::Ui2dImage> >,tuple$<> >,bevy_ecs::system::query::Query<tuple$<ref$<toolbar::resouces_and_components::SwitchAppModeButton>,ref_mut$<country_core::components::Ui2dImage> >,bevy_ecs::query::filter::Without<toolbar::resouces_and_components::ToggleBuildingMaterialButton> >,bevy_ecs::message::message_reader::MessageReader<toolbar::ui_interaction::UiClickEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::toolbar::BuildingPlacementMaterial>,bevy_ecs::system::commands::Commands)> @ 0x23af0ea -> 0x2966b70 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::toolbar_button::toolbar_button_keep_focus
// 信息 ID: 99a214439b97bd618dca18b1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 8aec120b0a87428e67a19eb3: void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::message::message_reader::MessageReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::message::message_reader::MessageReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >),tuple$<>,void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::message::message_reader::MessageReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)> > @ 0x2309135 -> 0x2965920 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$12::run_unsafe<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::message::message_reader::MessageReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >),tuple$<>,void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::message::message_reader::MessageReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)> @ 0x23aee5f -> 0x2965920 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
