// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/systems/decorator/src/window_lights.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: b537fb3f5e63d247e126814a7eb2b8ed88ba4575c737b4edd4176bf9f7dd1152
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: window_lights.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 1278, 3857
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use bevy_ecs; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: []
// use puffin; // namespace candidate
//   package identities: [{"name":"puffin","source":"PDB_registry_path_only","version":"0.14.1"}]
// use std; // namespace candidate
//   package identities: []
// use system_decorator; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_decorator::window_lights::window_lights
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3857,"record_offset":3064,"rva":29699120,"va":5398408240}
// 编译实例 RVAs: 0x1c52c30
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dccd0
//     owner=unresolved_namespace; count=1
//     target_aliases=9 candidates; complete list: window_lights.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-28795990","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398408240,"raw_hex":"e8aa9b48fe","size":5,"soft_fail":false,"target_va":5369613520,"va":5398409505,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: window_lights.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-25660858","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398408240,"raw_hex":"e8467278fe","size":5,"soft_fail":false,"target_va":5372749712,"va":5398410565,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140549fe0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: window_lights.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-24153872","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398408240,"raw_hex":"e8f0708ffe","size":5,"soft_fail":false,"target_va":5374255072,"va":5398408939,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1420c57c0
//     owner=unresolved_namespace; count=1
//     target_aliases=8 candidates; complete list: window_lights.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4662742","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398408240,"raw_hex":"e8d6254700","size":5,"soft_fail":false,"target_va":5403072448,"va":5398409701,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-27104412","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398408240,"raw_hex":"e8646b62fe","size":5,"soft_fail":false,"target_va":5371305056,"va":5398409463,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t4568060","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398408240,"raw_hex":"e8fcb34500","size":5,"soft_fail":false,"target_va":5402978688,"va":5398410623,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<core::option::Option<puffin::ProfilerScope> > >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-3623","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398408240,"raw_hex":"e8d9f1ffff","size":5,"soft_fail":false,"target_va":5398406960,"va":5398410578,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t4571382","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398408240,"raw_hex":"e8f6c04500","size":5,"soft_fail":false,"target_va":5402980128,"va":5398408741,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-1178","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398408240,"raw_hex":"e866fbffff","size":5,"soft_fail":false,"target_va":5398407152,"va":5398408325,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t4572249","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398408240,"raw_hex":"e859c44500","size":5,"soft_fail":false,"target_va":5402980656,"va":5398408402,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] fastrand::Rng::f32
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-12559168","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398408240,"raw_hex":"e8c05c40ff","size":5,"soft_fail":false,"target_va":5385850032,"va":5398409195,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-22051997","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398408240,"raw_hex":"e86383affe","size":5,"soft_fail":false,"target_va":5376356912,"va":5398408904,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::__getit::__KEY$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-22048117","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398408240,"raw_hex":"e88b92affe","size":5,"soft_fail":false,"target_va":5376360560,"va":5398408672,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-22052162","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398408240,"raw_hex":"e8be82affe","size":5,"soft_fail":false,"target_va":5376356144,"va":5398408301,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::thread_local::fast_local::Key$LT$T$GT$::try_initialize::hcc99216871ec738f (.llvm.16000913382889057913)
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-16118889","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398408240,"raw_hex":"e8970b0aff","size":5,"soft_fail":false,"target_va":5382289808,"va":5398408692,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::time_continuous_lerp
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-21470548","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398408240,"raw_hex":"e8ac62b8fe","size":5,"soft_fail":false,"target_va":5376939968,"va":5398410511,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3857,"record_offset":7120,"parent_record_offset":3064,"ancestor_record_offset":3064,"inlinee":4175,"source_line":422,"archive":"_pdb/A/modules/03857.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3857,"record_offset":7724,"parent_record_offset":3064,"ancestor_record_offset":3064,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/03857.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=5
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":3857,"record_offset":5988,"parent_record_offset":3064,"ancestor_record_offset":3064,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/03857.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::query::iter::impl$1::next
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs
//     evidence={"module":3857,"record_offset":7348,"parent_record_offset":3064,"ancestor_record_offset":3064,"inlinee":14613,"source_line":881,"archive":"_pdb/A/modules/03857.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::query::Query::iter
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/query.rs
//     evidence={"module":3857,"record_offset":7148,"parent_record_offset":3064,"ancestor_record_offset":3064,"inlinee":14606,"source_line":435,"archive":"_pdb/A/modules/03857.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":3857,"record_offset":12784,"parent_record_offset":3064,"ancestor_record_offset":3064,"inlinee":5741,"source_line":924,"archive":"_pdb/A/modules/03857.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":3857,"record_offset":6440,"parent_record_offset":3064,"ancestor_record_offset":3064,"inlinee":5443,"source_line":46,"archive":"_pdb/A/modules/03857.jsonl.gz"}
//   [pdb_inline_direct] core::ops::range::Range::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\range.rs
//     evidence={"module":3857,"record_offset":5928,"parent_record_offset":3064,"ancestor_record_offset":3064,"inlinee":10010,"source_line":120,"archive":"_pdb/A/modules/03857.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3857,"record_offset":12812,"parent_record_offset":3064,"ancestor_record_offset":3064,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/03857.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::session::ActiveSession::is_olden
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":3857,"record_offset":5900,"parent_record_offset":3064,"ancestor_record_offset":3064,"inlinee":27890,"source_line":215,"archive":"_pdb/A/modules/03857.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3857,"record_offset":4852,"parent_record_offset":3064,"ancestor_record_offset":3064,"inlinee":7309,"source_line":589,"archive":"_pdb/A/modules/03857.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3857,"record_offset":3264,"parent_record_offset":3064,"ancestor_record_offset":3064,"inlinee":7284,"source_line":626,"archive":"_pdb/A/modules/03857.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3857,"record_offset":3840,"parent_record_offset":3064,"ancestor_record_offset":3064,"inlinee":7291,"source_line":659,"archive":"_pdb/A/modules/03857.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::Entry::or_insert
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":3857,"record_offset":8884,"parent_record_offset":3064,"ancestor_record_offset":3064,"inlinee":30810,"source_line":2639,"archive":"_pdb/A/modules/03857.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::entry
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":3857,"record_offset":7900,"parent_record_offset":3064,"ancestor_record_offset":3064,"inlinee":6540,"source_line":855,"archive":"_pdb/A/modules/03857.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::iter_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":3857,"record_offset":6108,"parent_record_offset":3064,"ancestor_record_offset":3064,"inlinee":6418,"source_line":553,"archive":"_pdb/A/modules/03857.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::retain
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":3857,"record_offset":10076,"parent_record_offset":3064,"ancestor_record_offset":3064,"inlinee":14994,"source_line":688,"archive":"_pdb/A/modules/03857.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$42::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":3857,"record_offset":12172,"parent_record_offset":3064,"ancestor_record_offset":3064,"inlinee":8351,"source_line":2269,"archive":"_pdb/A/modules/03857.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3857,"record_offset":12988,"parent_record_offset":12964,"ancestor_record_offset":3064,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/03857.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3857,"record_offset":12964,"parent_record_offset":12864,"ancestor_record_offset":3064,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/03857.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3857,"record_offset":12896,"parent_record_offset":12864,"ancestor_record_offset":3064,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/03857.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3857,"record_offset":7780,"parent_record_offset":7756,"ancestor_record_offset":3064,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/03857.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3857,"record_offset":12864,"parent_record_offset":12836,"ancestor_record_offset":3064,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/03857.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3857,"record_offset":7756,"parent_record_offset":7724,"ancestor_record_offset":3064,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/03857.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$29::set_changed
//     owner=git_dependency_definition; count=5
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":3857,"record_offset":6016,"parent_record_offset":5988,"ancestor_record_offset":3064,"inlinee":5812,"source_line":309,"archive":"_pdb/A/modules/03857.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$21::fetch
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":3857,"record_offset":7660,"parent_record_offset":7376,"ancestor_record_offset":3064,"inlinee":14627,"source_line":963,"archive":"_pdb/A/modules/03857.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIter::new
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs
//     evidence={"module":3857,"record_offset":7200,"parent_record_offset":7176,"ancestor_record_offset":3064,"inlinee":14609,"source_line":36,"archive":"_pdb/A/modules/03857.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::init
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs
//     evidence={"module":3857,"record_offset":7224,"parent_record_offset":7200,"ancestor_record_offset":3064,"inlinee":14611,"source_line":1520,"archive":"_pdb/A/modules/03857.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::next
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs
//     evidence={"module":3857,"record_offset":7376,"parent_record_offset":7348,"ancestor_record_offset":3064,"inlinee":14614,"source_line":1587,"archive":"_pdb/A/modules/03857.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::state::QueryState::iter_unchecked_manual
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/state.rs
//     evidence={"module":3857,"record_offset":7176,"parent_record_offset":7148,"ancestor_record_offset":3064,"inlinee":14607,"source_line":1240,"archive":"_pdb/A/modules/03857.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::entities
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":3857,"record_offset":7632,"parent_record_offset":7376,"ancestor_record_offset":3064,"inlinee":14616,"source_line":568,"archive":"_pdb/A/modules/03857.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::TableRow::as_usize
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":3857,"record_offset":7684,"parent_record_offset":7660,"ancestor_record_offset":3064,"inlinee":14629,"source_line":125,"archive":"_pdb/A/modules/03857.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":3857,"record_offset":7548,"parent_record_offset":7376,"ancestor_record_offset":3064,"inlinee":14618,"source_line":834,"archive":"_pdb/A/modules/03857.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":3857,"record_offset":5252,"parent_record_offset":5224,"ancestor_record_offset":3064,"inlinee":4785,"source_line":1782,"archive":"_pdb/A/modules/03857.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":3857,"record_offset":5304,"parent_record_offset":5280,"ancestor_record_offset":3064,"inlinee":4790,"source_line":479,"archive":"_pdb/A/modules/03857.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":3857,"record_offset":5280,"parent_record_offset":5252,"ancestor_record_offset":3064,"inlinee":4789,"source_line":410,"archive":"_pdb/A/modules/03857.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":3857,"record_offset":5192,"parent_record_offset":5164,"ancestor_record_offset":3064,"inlinee":4782,"source_line":1058,"archive":"_pdb/A/modules/03857.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":3857,"record_offset":5224,"parent_record_offset":5192,"ancestor_record_offset":3064,"inlinee":4783,"source_line":1090,"archive":"_pdb/A/modules/03857.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":3857,"record_offset":5396,"parent_record_offset":5224,"ancestor_record_offset":3064,"inlinee":5305,"source_line":2122,"archive":"_pdb/A/modules/03857.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$42::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":3857,"record_offset":5508,"parent_record_offset":5484,"ancestor_record_offset":3064,"inlinee":4787,"source_line":1773,"archive":"_pdb/A/modules/03857.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$21::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":3857,"record_offset":8648,"parent_record_offset":8624,"ancestor_record_offset":3064,"inlinee":19598,"source_line":1469,"archive":"_pdb/A/modules/03857.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_cmpeq_epi8
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":3857,"record_offset":11256,"parent_record_offset":11228,"ancestor_record_offset":3064,"inlinee":9626,"source_line":843,"archive":"_pdb/A/modules/03857.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":3857,"record_offset":6296,"parent_record_offset":6268,"ancestor_record_offset":3064,"inlinee":4898,"source_line":1225,"archive":"_pdb/A/modules/03857.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":3857,"record_offset":8140,"parent_record_offset":8116,"ancestor_record_offset":3064,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/03857.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":3857,"record_offset":6356,"parent_record_offset":6324,"ancestor_record_offset":3064,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/03857.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":3857,"record_offset":8164,"parent_record_offset":8140,"ancestor_record_offset":3064,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/03857.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":3857,"record_offset":5328,"parent_record_offset":5304,"ancestor_record_offset":3064,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/03857.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":3857,"record_offset":12928,"parent_record_offset":12896,"ancestor_record_offset":3064,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/03857.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::wrapping_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":3857,"record_offset":9724,"parent_record_offset":9696,"ancestor_record_offset":3064,"inlinee":4907,"source_line":1680,"archive":"_pdb/A/modules/03857.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\mod.rs
//     evidence={"module":3857,"record_offset":3476,"parent_record_offset":3380,"ancestor_record_offset":3064,"inlinee":6114,"source_line":1101,"archive":"_pdb/A/modules/03857.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$7::leading_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":3857,"record_offset":11348,"parent_record_offset":11320,"ancestor_record_offset":3064,"inlinee":9628,"source_line":148,"archive":"_pdb/A/modules/03857.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$7::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":3857,"record_offset":11380,"parent_record_offset":11320,"ancestor_record_offset":3064,"inlinee":6977,"source_line":169,"archive":"_pdb/A/modules/03857.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":3857,"record_offset":6704,"parent_record_offset":6680,"ancestor_record_offset":3064,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/03857.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\range.rs
//     evidence={"module":3857,"record_offset":5956,"parent_record_offset":5928,"ancestor_record_offset":3064,"inlinee":6738,"source_line":817,"archive":"_pdb/A/modules/03857.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3857,"record_offset":5088,"parent_record_offset":5060,"ancestor_record_offset":3064,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/03857.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3857,"record_offset":4436,"parent_record_offset":3976,"ancestor_record_offset":3064,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/03857.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::ok_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3857,"record_offset":5136,"parent_record_offset":4960,"ancestor_record_offset":3064,"inlinee":7261,"source_line":1206,"archive":"_pdb/A/modules/03857.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":3857,"record_offset":3788,"parent_record_offset":3760,"ancestor_record_offset":3064,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/03857.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::offset_from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":3857,"record_offset":11492,"parent_record_offset":11468,"ancestor_record_offset":3064,"inlinee":9624,"source_line":716,"archive":"_pdb/A/modules/03857.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3857,"record_offset":5460,"parent_record_offset":5164,"ancestor_record_offset":3064,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/03857.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3857,"record_offset":7296,"parent_record_offset":7272,"ancestor_record_offset":3064,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/03857.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3857,"record_offset":6952,"parent_record_offset":6928,"ancestor_record_offset":3064,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/03857.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3857,"record_offset":6928,"parent_record_offset":6904,"ancestor_record_offset":3064,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/03857.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3857,"record_offset":9944,"parent_record_offset":9920,"ancestor_record_offset":3064,"inlinee":7069,"source_line":1486,"archive":"_pdb/A/modules/03857.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":3857,"record_offset":7512,"parent_record_offset":7488,"ancestor_record_offset":3064,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/03857.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":3857,"record_offset":4212,"parent_record_offset":4188,"ancestor_record_offset":3064,"inlinee":7305,"source_line":531,"archive":"_pdb/A/modules/03857.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":3857,"record_offset":4188,"parent_record_offset":4164,"ancestor_record_offset":3064,"inlinee":7304,"source_line":709,"archive":"_pdb/A/modules/03857.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":3857,"record_offset":4396,"parent_record_offset":4364,"ancestor_record_offset":3064,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/03857.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3857,"record_offset":5352,"parent_record_offset":5328,"ancestor_record_offset":3064,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/03857.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":3857,"record_offset":5860,"parent_record_offset":4924,"ancestor_record_offset":3064,"inlinee":5301,"source_line":1028,"archive":"_pdb/A/modules/03857.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$10::slice_contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":3857,"record_offset":11588,"parent_record_offset":11564,"ancestor_record_offset":3064,"inlinee":4965,"source_line":215,"archive":"_pdb/A/modules/03857.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$10::slice_contains::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":3857,"record_offset":11704,"parent_record_offset":11612,"ancestor_record_offset":3064,"inlinee":4968,"source_line":216,"archive":"_pdb/A/modules/03857.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":3857,"record_offset":11564,"parent_record_offset":11540,"ancestor_record_offset":3064,"inlinee":4963,"source_line":2524,"archive":"_pdb/A/modules/03857.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":3857,"record_offset":3412,"parent_record_offset":3380,"ancestor_record_offset":3064,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/03857.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":3857,"record_offset":7248,"parent_record_offset":7224,"ancestor_record_offset":3064,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/03857.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":3857,"record_offset":3440,"parent_record_offset":3412,"ancestor_record_offset":3064,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/03857.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":3857,"record_offset":7272,"parent_record_offset":7248,"ancestor_record_offset":3064,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/03857.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":3857,"record_offset":7488,"parent_record_offset":7424,"ancestor_record_offset":3064,"inlinee":5091,"source_line":101,"archive":"_pdb/A/modules/03857.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":3857,"record_offset":4164,"parent_record_offset":4140,"ancestor_record_offset":3064,"inlinee":7303,"source_line":119,"archive":"_pdb/A/modules/03857.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::any
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":3857,"record_offset":4492,"parent_record_offset":4468,"ancestor_record_offset":3064,"inlinee":4966,"source_line":279,"archive":"_pdb/A/modules/03857.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":3857,"record_offset":7424,"parent_record_offset":7376,"ancestor_record_offset":3064,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/03857.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":3857,"record_offset":4140,"parent_record_offset":4032,"ancestor_record_offset":3064,"inlinee":7302,"source_line":392,"archive":"_pdb/A/modules/03857.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":3857,"record_offset":3380,"parent_record_offset":3352,"ancestor_record_offset":3064,"inlinee":6113,"source_line":217,"archive":"_pdb/A/modules/03857.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":3857,"record_offset":3872,"parent_record_offset":3840,"ancestor_record_offset":3064,"inlinee":7292,"source_line":1310,"archive":"_pdb/A/modules/03857.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":3857,"record_offset":4004,"parent_record_offset":3976,"ancestor_record_offset":3064,"inlinee":7272,"source_line":131,"archive":"_pdb/A/modules/03857.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":3857,"record_offset":3976,"parent_record_offset":3944,"ancestor_record_offset":3064,"inlinee":7300,"source_line":221,"archive":"_pdb/A/modules/03857.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":3857,"record_offset":3920,"parent_record_offset":3896,"ancestor_record_offset":3064,"inlinee":7296,"source_line":301,"archive":"_pdb/A/modules/03857.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":3857,"record_offset":3944,"parent_record_offset":3920,"ancestor_record_offset":3064,"inlinee":7298,"source_line":690,"archive":"_pdb/A/modules/03857.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":3857,"record_offset":3896,"parent_record_offset":3872,"ancestor_record_offset":3064,"inlinee":7294,"source_line":779,"archive":"_pdb/A/modules/03857.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":3857,"record_offset":4468,"parent_record_offset":3944,"ancestor_record_offset":3064,"inlinee":7307,"source_line":640,"archive":"_pdb/A/modules/03857.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":3857,"record_offset":3576,"parent_record_offset":3544,"ancestor_record_offset":3064,"inlinee":7287,"source_line":402,"archive":"_pdb/A/modules/03857.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":3857,"record_offset":3736,"parent_record_offset":3576,"ancestor_record_offset":3064,"inlinee":7288,"source_line":422,"archive":"_pdb/A/modules/03857.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":3857,"record_offset":3544,"parent_record_offset":3520,"ancestor_record_offset":3064,"inlinee":7286,"source_line":434,"archive":"_pdb/A/modules/03857.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":3857,"record_offset":3296,"parent_record_offset":3264,"ancestor_record_offset":3064,"inlinee":6109,"source_line":60,"archive":"_pdb/A/modules/03857.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":3857,"record_offset":3760,"parent_record_offset":3736,"ancestor_record_offset":3064,"inlinee":7290,"source_line":192,"archive":"_pdb/A/modules/03857.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$9::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":3857,"record_offset":3352,"parent_record_offset":3320,"ancestor_record_offset":3064,"inlinee":6112,"source_line":333,"archive":"_pdb/A/modules/03857.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$9::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":3857,"record_offset":3320,"parent_record_offset":3296,"ancestor_record_offset":3064,"inlinee":6111,"source_line":363,"archive":"_pdb/A/modules/03857.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":3857,"record_offset":4032,"parent_record_offset":4004,"ancestor_record_offset":3064,"inlinee":7273,"source_line":79,"archive":"_pdb/A/modules/03857.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":3857,"record_offset":4280,"parent_record_offset":4032,"ancestor_record_offset":3064,"inlinee":4936,"source_line":17,"archive":"_pdb/A/modules/03857.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":3857,"record_offset":4252,"parent_record_offset":4032,"ancestor_record_offset":3064,"inlinee":4935,"source_line":11,"archive":"_pdb/A/modules/03857.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":3857,"record_offset":4084,"parent_record_offset":4032,"ancestor_record_offset":3064,"inlinee":7274,"source_line":24,"archive":"_pdb/A/modules/03857.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::iter_mut
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":3857,"record_offset":6132,"parent_record_offset":6108,"ancestor_record_offset":3064,"inlinee":6419,"source_line":801,"archive":"_pdb/A/modules/03857.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::reserve
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":3857,"record_offset":8816,"parent_record_offset":7924,"ancestor_record_offset":3064,"inlinee":6552,"source_line":1105,"archive":"_pdb/A/modules/03857.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::retain
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":3857,"record_offset":10100,"parent_record_offset":10076,"ancestor_record_offset":3064,"inlinee":28547,"source_line":919,"archive":"_pdb/A/modules/03857.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::rustc_entry
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\rustc_entry.rs
//     evidence={"module":3857,"record_offset":7924,"parent_record_offset":7900,"ancestor_record_offset":3064,"inlinee":6541,"source_line":34,"archive":"_pdb/A/modules/03857.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$63::next
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":3857,"record_offset":6500,"parent_record_offset":6472,"ancestor_record_offset":3064,"inlinee":8353,"source_line":4766,"archive":"_pdb/A/modules/03857.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":3857,"record_offset":7960,"parent_record_offset":7924,"ancestor_record_offset":3064,"inlinee":5617,"source_line":257,"archive":"_pdb/A/modules/03857.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3857,"record_offset":8536,"parent_record_offset":8512,"ancestor_record_offset":3064,"inlinee":5640,"source_line":513,"archive":"_pdb/A/modules/03857.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3857,"record_offset":8512,"parent_record_offset":8404,"ancestor_record_offset":3064,"inlinee":5639,"source_line":682,"archive":"_pdb/A/modules/03857.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3857,"record_offset":8452,"parent_record_offset":8428,"ancestor_record_offset":3064,"inlinee":5638,"source_line":355,"archive":"_pdb/A/modules/03857.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::next_n
//     owner=unresolved_namespace; count=6
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3857,"record_offset":6904,"parent_record_offset":6564,"ancestor_record_offset":3064,"inlinee":8358,"source_line":561,"archive":"_pdb/A/modules/03857.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::to_base_index
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3857,"record_offset":11444,"parent_record_offset":11420,"ancestor_record_offset":3064,"inlinee":11800,"source_line":428,"archive":"_pdb/A/modules/03857.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::write
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3857,"record_offset":9920,"parent_record_offset":8956,"ancestor_record_offset":3064,"inlinee":6569,"source_line":632,"archive":"_pdb/A/modules/03857.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3857,"record_offset":8776,"parent_record_offset":8012,"ancestor_record_offset":3064,"inlinee":5645,"source_line":175,"archive":"_pdb/A/modules/03857.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::new
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3857,"record_offset":6240,"parent_record_offset":6180,"ancestor_record_offset":3064,"inlinee":6425,"source_line":3760,"archive":"_pdb/A/modules/03857.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::next_impl
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3857,"record_offset":6564,"parent_record_offset":6528,"ancestor_record_offset":3064,"inlinee":8357,"source_line":3827,"archive":"_pdb/A/modules/03857.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3857,"record_offset":8428,"parent_record_offset":8404,"ancestor_record_offset":3064,"inlinee":5636,"source_line":997,"archive":"_pdb/A/modules/03857.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket_index
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3857,"record_offset":11420,"parent_record_offset":11000,"ancestor_record_offset":3064,"inlinee":11799,"source_line":968,"archive":"_pdb/A/modules/03857.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::erase
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3857,"record_offset":10972,"parent_record_offset":10100,"ancestor_record_offset":3064,"inlinee":28550,"source_line":1037,"archive":"_pdb/A/modules/03857.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::erase_no_drop
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3857,"record_offset":11000,"parent_record_offset":10972,"ancestor_record_offset":3064,"inlinee":11798,"source_line":1029,"archive":"_pdb/A/modules/03857.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3857,"record_offset":7988,"parent_record_offset":7924,"ancestor_record_offset":3064,"inlinee":5620,"source_line":1456,"archive":"_pdb/A/modules/03857.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::insert_no_grow
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3857,"record_offset":8956,"parent_record_offset":8932,"ancestor_record_offset":3064,"inlinee":6558,"source_line":1362,"archive":"_pdb/A/modules/03857.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::iter
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3857,"record_offset":6156,"parent_record_offset":6132,"ancestor_record_offset":3064,"inlinee":6420,"source_line":1596,"archive":"_pdb/A/modules/03857.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::reserve
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3857,"record_offset":8840,"parent_record_offset":8816,"ancestor_record_offset":3064,"inlinee":6553,"source_line":1175,"archive":"_pdb/A/modules/03857.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::bucket
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3857,"record_offset":9768,"parent_record_offset":8956,"ancestor_record_offset":3064,"inlinee":6567,"source_line":2501,"archive":"_pdb/A/modules/03857.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::data_end
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3857,"record_offset":6212,"parent_record_offset":6180,"ancestor_record_offset":3064,"inlinee":6422,"source_line":2597,"archive":"_pdb/A/modules/03857.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::erase
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3857,"record_offset":11036,"parent_record_offset":11000,"ancestor_record_offset":3064,"inlinee":11802,"source_line":3381,"archive":"_pdb/A/modules/03857.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3857,"record_offset":8012,"parent_record_offset":7988,"ancestor_record_offset":3064,"inlinee":5622,"source_line":2168,"archive":"_pdb/A/modules/03857.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_insert_slot
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3857,"record_offset":9036,"parent_record_offset":9000,"ancestor_record_offset":3064,"inlinee":6560,"source_line":2111,"archive":"_pdb/A/modules/03857.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_insert_slot_in_group
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3857,"record_offset":9216,"parent_record_offset":9036,"ancestor_record_offset":3064,"inlinee":6561,"source_line":1906,"archive":"_pdb/A/modules/03857.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::fix_insert_slot
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3857,"record_offset":9380,"parent_record_offset":9036,"ancestor_record_offset":3064,"inlinee":6562,"source_line":1869,"archive":"_pdb/A/modules/03857.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::is_bucket_full
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3857,"record_offset":9412,"parent_record_offset":9380,"ancestor_record_offset":3064,"inlinee":6563,"source_line":2816,"archive":"_pdb/A/modules/03857.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3857,"record_offset":6180,"parent_record_offset":6156,"ancestor_record_offset":3064,"inlinee":6421,"source_line":2297,"archive":"_pdb/A/modules/03857.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::prepare_insert_slot
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3857,"record_offset":9000,"parent_record_offset":8956,"ancestor_record_offset":3064,"inlinee":6559,"source_line":2068,"archive":"_pdb/A/modules/03857.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3857,"record_offset":8088,"parent_record_offset":8012,"ancestor_record_offset":3064,"inlinee":6542,"source_line":2610,"archive":"_pdb/A/modules/03857.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::set_ctrl
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3857,"record_offset":9696,"parent_record_offset":9672,"ancestor_record_offset":3064,"inlinee":6566,"source_line":2743,"archive":"_pdb/A/modules/03857.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::set_ctrl_h2
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3857,"record_offset":9672,"parent_record_offset":9000,"ancestor_record_offset":3064,"inlinee":6565,"source_line":2677,"archive":"_pdb/A/modules/03857.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::leading_zeros
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":3857,"record_offset":11320,"parent_record_offset":11036,"ancestor_record_offset":3064,"inlinee":11803,"source_line":86,"archive":"_pdb/A/modules/03857.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=5
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":3857,"record_offset":6648,"parent_record_offset":6612,"ancestor_record_offset":3064,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/03857.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=5
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":3857,"record_offset":6680,"parent_record_offset":6648,"ancestor_record_offset":3064,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/03857.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::remove_lowest_bit
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":3857,"record_offset":6740,"parent_record_offset":6612,"ancestor_record_offset":3064,"inlinee":5633,"source_line":37,"archive":"_pdb/A/modules/03857.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":3857,"record_offset":6612,"parent_record_offset":6564,"ancestor_record_offset":3064,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/03857.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::h2
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3857,"record_offset":8060,"parent_record_offset":8012,"ancestor_record_offset":3064,"inlinee":5624,"source_line":150,"archive":"_pdb/A/modules/03857.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$28::next
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3857,"record_offset":6528,"parent_record_offset":6500,"ancestor_record_offset":3064,"inlinee":8355,"source_line":4127,"archive":"_pdb/A/modules/03857.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3857,"record_offset":8404,"parent_record_offset":8012,"ancestor_record_offset":3064,"inlinee":5635,"source_line":1466,"archive":"_pdb/A/modules/03857.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::is_full
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3857,"record_offset":9436,"parent_record_offset":9412,"ancestor_record_offset":3064,"inlinee":6564,"source_line":115,"archive":"_pdb/A/modules/03857.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::offset_from
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3857,"record_offset":11468,"parent_record_offset":11444,"ancestor_record_offset":3064,"inlinee":11801,"source_line":69,"archive":"_pdb/A/modules/03857.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::special_is_empty
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3857,"record_offset":9892,"parent_record_offset":8956,"ancestor_record_offset":3064,"inlinee":6568,"source_line":127,"archive":"_pdb/A/modules/03857.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":3857,"record_offset":8116,"parent_record_offset":8012,"ancestor_record_offset":3064,"inlinee":5626,"source_line":52,"archive":"_pdb/A/modules/03857.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load_aligned
//     owner=unresolved_namespace; count=7
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":3857,"record_offset":6268,"parent_record_offset":6240,"ancestor_record_offset":3064,"inlinee":6426,"source_line":60,"archive":"_pdb/A/modules/03857.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_byte
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":3857,"record_offset":8200,"parent_record_offset":8012,"ancestor_record_offset":3064,"inlinee":5627,"source_line":79,"archive":"_pdb/A/modules/03857.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":3857,"record_offset":8684,"parent_record_offset":8012,"ancestor_record_offset":3064,"inlinee":5643,"source_line":97,"archive":"_pdb/A/modules/03857.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty_or_deleted
//     owner=unresolved_namespace; count=8
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":3857,"record_offset":6324,"parent_record_offset":6268,"ancestor_record_offset":3064,"inlinee":6427,"source_line":104,"archive":"_pdb/A/modules/03857.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::rustc_entry::RustcVacantEntry::insert
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\rustc_entry.rs
//     evidence={"module":3857,"record_offset":8932,"parent_record_offset":8908,"ancestor_record_offset":3064,"inlinee":6557,"source_line":574,"archive":"_pdb/A/modules/03857.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::rustc_entry::impl$0::rustc_entry::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\rustc_entry.rs
//     evidence={"module":3857,"record_offset":8624,"parent_record_offset":8404,"ancestor_record_offset":3064,"inlinee":6544,"source_line":36,"archive":"_pdb/A/modules/03857.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3857,"record_offset":4888,"parent_record_offset":4852,"ancestor_record_offset":3064,"inlinee":7259,"source_line":425,"archive":"_pdb/A/modules/03857.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::__getit
//     owner=unresolved_namespace; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":3857,"record_offset":5000,"parent_record_offset":4960,"ancestor_record_offset":3064,"inlinee":7260,"source_line":85,"archive":"_pdb/A/modules/03857.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3857,"record_offset":5164,"parent_record_offset":4960,"ancestor_record_offset":3064,"inlinee":7263,"source_line":429,"archive":"_pdb/A/modules/03857.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3857,"record_offset":5432,"parent_record_offset":5164,"ancestor_record_offset":3064,"inlinee":7311,"source_line":591,"archive":"_pdb/A/modules/03857.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::VacantEntry::insert
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":3857,"record_offset":8908,"parent_record_offset":8884,"ancestor_record_offset":3064,"inlinee":6555,"source_line":3092,"archive":"_pdb/A/modules/03857.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$42::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":3857,"record_offset":6472,"parent_record_offset":6440,"ancestor_record_offset":3064,"inlinee":8351,"source_line":2269,"archive":"_pdb/A/modules/03857.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::fast_local::Key::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":3857,"record_offset":5028,"parent_record_offset":5000,"ancestor_record_offset":3064,"inlinee":5546,"source_line":165,"archive":"_pdb/A/modules/03857.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::lazy::LazyKeyInner::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\mod.rs
//     evidence={"module":3857,"record_offset":5060,"parent_record_offset":5028,"ancestor_record_offset":3064,"inlinee":5548,"source_line":41,"archive":"_pdb/A/modules/03857.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":3857,"record_offset":4960,"parent_record_offset":4924,"ancestor_record_offset":3064,"inlinee":5542,"source_line":278,"archive":"_pdb/A/modules/03857.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":3857,"record_offset":4924,"parent_record_offset":4888,"ancestor_record_offset":3064,"inlinee":5541,"source_line":256,"archive":"_pdb/A/modules/03857.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::window_lights::window_lights::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/window_lights.rs
//     evidence={"module":3857,"record_offset":11540,"parent_record_offset":10100,"ancestor_record_offset":3064,"inlinee":60217,"source_line":62,"archive":"_pdb/A/modules/03857.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::window_lights::window_lights::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 62
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xEB39
// 首条位置: {"module":1278,"inlinee":60217}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::slice::impl$0::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":3857,"record_offset":11564,"parent_record_offset":11540,"ancestor_record_offset":11540,"inlinee":4963,"source_line":2524,"archive":"_pdb/A/modules/03857.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$21::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":3857,"record_offset":11728,"parent_record_offset":11704,"ancestor_record_offset":11540,"inlinee":19598,"source_line":1469,"archive":"_pdb/A/modules/03857.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":3857,"record_offset":11672,"parent_record_offset":11640,"ancestor_record_offset":11540,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/03857.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$10::slice_contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":3857,"record_offset":11588,"parent_record_offset":11564,"ancestor_record_offset":11540,"inlinee":4965,"source_line":215,"archive":"_pdb/A/modules/03857.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$10::slice_contains::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":3857,"record_offset":11704,"parent_record_offset":11612,"ancestor_record_offset":11540,"inlinee":4968,"source_line":216,"archive":"_pdb/A/modules/03857.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":3857,"record_offset":11612,"parent_record_offset":11588,"ancestor_record_offset":11540,"inlinee":4966,"source_line":279,"archive":"_pdb/A/modules/03857.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":3857,"record_offset":11640,"parent_record_offset":11612,"ancestor_record_offset":11540,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/03857.jsonl.gz"}
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
// 完整结构及原始类型记录: window_lights.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: window_lights.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: window_lights.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::window_lights::GlobalWindowLightsControl>,bevy_ecs::system::query::Query<ref$<country_core::resources::window_lights::WindowLight>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::resources::window_lights::GlobalWindowLightsControl>", "bevy_ecs::system::query::Query<ref$<country_core::resources::window_lights::WindowLight>,tuple$<> >", "bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>", "bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>", "bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::window_lights::GlobalWindowLightsControl>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<ref$<country_core::resources::window_lights::WindowLight>,tuple$<> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>

// ------------------------------------------------------------------
// 函数: system_decorator::window_lights::window_lights
// 信息 ID: cb1fd5049cebc85805b899f3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] eed2d4b151bc539b415df5c5: void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::window_lights::GlobalWindowLightsControl>,bevy_ecs::system::query::Query<ref$<country_core::resources::window_lights::WindowLight>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::window_lights::GlobalWindowLightsControl>,bevy_ecs::system::query::Query<ref$<country_core::resources::window_lights::WindowLight>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>),void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::window_lights::GlobalWindowLightsControl>,bevy_ecs::system::query::Query<ref$<country_core::resources::window_lights::WindowLight>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>)> > @ 0x17cc1fd -> 0x1c52c30 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::window_lights::GlobalWindowLightsControl>,bevy_ecs::system::query::Query<ref$<country_core::resources::window_lights::WindowLight>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>),void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::window_lights::GlobalWindowLightsControl>,bevy_ecs::system::query::Query<ref$<country_core::resources::window_lights::WindowLight>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>)> @ 0x17f492c -> 0x1c52c30 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::window_lights::window_lights::closure$0
// 信息 ID: d12b2e01fa5dc7515504b111
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_decorator::window_lights::window_lights", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::window_lights::window_lights; module=3857, inline_record=11540; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
