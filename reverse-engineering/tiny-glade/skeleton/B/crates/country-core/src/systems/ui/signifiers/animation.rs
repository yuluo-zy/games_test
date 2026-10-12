// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/country-core/src/systems/ui/signifiers/animation.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: 670b3c664eb21abbf4a2caa843a9a08b2ec224423fc5cba95091f8e5cde70174
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: animation.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 551
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
// use nonmax; // namespace candidate
//   package identities: [{"name":"nonmax","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.5.5"}]
// use puffin; // namespace candidate
//   package identities: [{"name":"puffin","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.1"}]
// use std; // namespace candidate
//   package identities: []
// use utils; // namespace candidate
//   package identities: [{"name":"utils","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::systems::ui::signifiers::animation::signifier_animation_t
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":551,"record_offset":15268,"rva":10916672,"va":5379625792}
// 编译实例 RVAs: 0xa69340
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1428f87d0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: animation.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t32042272","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379625792,"raw_hex":"e820ede801","size":5,"soft_fail":false,"target_va":5411669968,"va":5379627691,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t31913830","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379625792,"raw_hex":"e866f7e601","size":5,"soft_fail":false,"target_va":5411541584,"va":5379627749,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-8219","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379625792,"raw_hex":"e8e5dfffff","size":5,"soft_fail":false,"target_va":5379617664,"va":5379625878,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t31917407","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379625792,"raw_hex":"e85f05e701","size":5,"soft_fail":false,"target_va":5411543360,"va":5379625948,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t1759130","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379625792,"raw_hex":"e89ad71a00","size":5,"soft_fail":false,"target_va":5381385536,"va":5379626401,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t1764316","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379625792,"raw_hex":"e8dceb1a00","size":5,"soft_fail":false,"target_va":5381390640,"va":5379626319,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::end_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t1758839","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379625792,"raw_hex":"e877d61a00","size":5,"soft_fail":false,"target_va":5381386480,"va":5379627636,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t1758850","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379625792,"raw_hex":"e882d61a00","size":5,"soft_fail":false,"target_va":5381384704,"va":5379625849,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::LocalKey$LT$T$GT$::with
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t9197261","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379625792,"raw_hex":"e8cd568c00","size":5,"soft_fail":false,"target_va":5388825296,"va":5379628030,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t31909362","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379625792,"raw_hex":"e8f2e5e601","size":5,"soft_fail":false,"target_va":5411537104,"va":5379627737,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::easing_functions::EasingFunction::apply
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2278634","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379625792,"raw_hex":"e8eac42200","size":5,"soft_fail":false,"target_va":5381906160,"va":5379627521,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$58::deref_mut
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":551,"record_offset":23736,"parent_record_offset":15268,"ancestor_record_offset":15268,"inlinee":9177,"source_line":434,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::query::iter::impl$1::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs
//     evidence={"module":551,"record_offset":18268,"parent_record_offset":15268,"ancestor_record_offset":15268,"inlinee":11673,"source_line":884,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::query::Query::iter_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":551,"record_offset":17860,"parent_record_offset":15268,"ancestor_record_offset":15268,"inlinee":17020,"source_line":701,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":551,"record_offset":23848,"parent_record_offset":15268,"ancestor_record_offset":15268,"inlinee":6356,"source_line":1409,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::as_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":551,"record_offset":23936,"parent_record_offset":15268,"ancestor_record_offset":15268,"inlinee":4130,"source_line":728,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":551,"record_offset":23704,"parent_record_offset":15268,"ancestor_record_offset":15268,"inlinee":4512,"source_line":607,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":551,"record_offset":24020,"parent_record_offset":15268,"ancestor_record_offset":15268,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":551,"record_offset":16892,"parent_record_offset":15268,"ancestor_record_offset":15268,"inlinee":4416,"source_line":589,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":551,"record_offset":15460,"parent_record_offset":15268,"ancestor_record_offset":15268,"inlinee":4368,"source_line":626,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":551,"record_offset":15916,"parent_record_offset":15268,"ancestor_record_offset":15268,"inlinee":4385,"source_line":659,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":551,"record_offset":18100,"parent_record_offset":18076,"ancestor_record_offset":15268,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":551,"record_offset":18148,"parent_record_offset":18124,"ancestor_record_offset":15268,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":551,"record_offset":18124,"parent_record_offset":18100,"ancestor_record_offset":15268,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":551,"record_offset":18076,"parent_record_offset":18052,"ancestor_record_offset":15268,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":551,"record_offset":18052,"parent_record_offset":18020,"ancestor_record_offset":15268,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":551,"record_offset":18644,"parent_record_offset":18620,"ancestor_record_offset":15268,"inlinee":4613,"source_line":2705,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":551,"record_offset":18668,"parent_record_offset":18644,"ancestor_record_offset":15268,"inlinee":4352,"source_line":2680,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":551,"record_offset":18020,"parent_record_offset":17992,"ancestor_record_offset":15268,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::contains
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":551,"record_offset":20720,"parent_record_offset":20696,"ancestor_record_offset":15268,"inlinee":5673,"source_line":621,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::entities
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":551,"record_offset":22876,"parent_record_offset":18292,"ancestor_record_offset":15268,"inlinee":11682,"source_line":466,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::is_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":551,"record_offset":20316,"parent_record_offset":18292,"ancestor_record_offset":15268,"inlinee":11679,"source_line":615,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::table_id
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":551,"record_offset":20400,"parent_record_offset":18292,"ancestor_record_offset":15268,"inlinee":5495,"source_line":460,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::ArchetypeEntity::table_row
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":551,"record_offset":23072,"parent_record_offset":18292,"ancestor_record_offset":15268,"inlinee":17012,"source_line":325,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetypes::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":551,"record_offset":20232,"parent_record_offset":18292,"ancestor_record_offset":15268,"inlinee":11678,"source_line":895,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$57::set_changed
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":551,"record_offset":23760,"parent_record_offset":23736,"ancestor_record_offset":15268,"inlinee":9179,"source_line":397,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::StorageSwitch::extract
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":551,"record_offset":22348,"parent_record_offset":22324,"ancestor_record_offset":15268,"inlinee":4876,"source_line":2472,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$26::matches_component_set
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":551,"record_offset":18848,"parent_record_offset":18816,"ancestor_record_offset":15268,"inlinee":5720,"source_line":1210,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$26::set_archetype
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":551,"record_offset":20936,"parent_record_offset":20636,"ancestor_record_offset":15268,"inlinee":4856,"source_line":1160,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$26::set_table
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":551,"record_offset":19112,"parent_record_offset":18816,"ancestor_record_offset":15268,"inlinee":4857,"source_line":1175,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::matches_component_set
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":551,"record_offset":19460,"parent_record_offset":19428,"ancestor_record_offset":15268,"inlinee":10339,"source_line":1577,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::set_archetype
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":551,"record_offset":20524,"parent_record_offset":20492,"ancestor_record_offset":15268,"inlinee":13655,"source_line":1524,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::set_table
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":551,"record_offset":18732,"parent_record_offset":18704,"ancestor_record_offset":15268,"inlinee":13656,"source_line":1539,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$37::fetch
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":551,"record_offset":22324,"parent_record_offset":22300,"ancestor_record_offset":15268,"inlinee":13686,"source_line":1596,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$37::fetch::closure$0
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":551,"record_offset":22372,"parent_record_offset":22348,"ancestor_record_offset":15268,"inlinee":9533,"source_line":1602,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$41::set_archetype
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":551,"record_offset":20636,"parent_record_offset":20492,"ancestor_record_offset":15268,"inlinee":17008,"source_line":1799,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$41::set_archetype::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":551,"record_offset":20696,"parent_record_offset":20668,"ancestor_record_offset":15268,"inlinee":17010,"source_line":1805,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$41::set_table
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":551,"record_offset":18816,"parent_record_offset":18704,"ancestor_record_offset":15268,"inlinee":17014,"source_line":1815,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$41::set_table::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":551,"record_offset":18876,"parent_record_offset":18848,"ancestor_record_offset":15268,"inlinee":17016,"source_line":1816,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$42::fetch
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":551,"record_offset":22532,"parent_record_offset":22300,"ancestor_record_offset":15268,"inlinee":26664,"source_line":1867,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$42::fetch::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":551,"record_offset":22636,"parent_record_offset":22612,"ancestor_record_offset":15268,"inlinee":26666,"source_line":1875,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$63::fetch
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":551,"record_offset":22300,"parent_record_offset":18292,"ancestor_record_offset":15268,"inlinee":13690,"source_line":2083,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIter::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs
//     evidence={"module":551,"record_offset":17968,"parent_record_offset":17884,"ancestor_record_offset":15268,"inlinee":13709,"source_line":39,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::init
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs
//     evidence={"module":551,"record_offset":17992,"parent_record_offset":17968,"ancestor_record_offset":15268,"inlinee":9292,"source_line":2359,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs
//     evidence={"module":551,"record_offset":18292,"parent_record_offset":18268,"ancestor_record_offset":15268,"inlinee":11674,"source_line":2458,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$3::set_archetype
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/world_query.rs
//     evidence={"module":551,"record_offset":20492,"parent_record_offset":18292,"ancestor_record_offset":15268,"inlinee":11680,"source_line":182,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$3::set_table
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/world_query.rs
//     evidence={"module":551,"record_offset":18704,"parent_record_offset":18292,"ancestor_record_offset":15268,"inlinee":11676,"source_line":195,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_array::BlobArray::get_sub_slice
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_array.rs
//     evidence={"module":551,"record_offset":19376,"parent_record_offset":19352,"ancestor_record_offset":15268,"inlinee":4872,"source_line":143,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseArray::contains
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":551,"record_offset":18948,"parent_record_offset":18924,"ancestor_record_offset":15268,"inlinee":5675,"source_line":49,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseArray::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":551,"record_offset":21060,"parent_record_offset":21036,"ancestor_record_offset":15268,"inlinee":4864,"source_line":58,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::contains
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":551,"record_offset":18924,"parent_record_offset":18900,"ancestor_record_offset":15268,"inlinee":5674,"source_line":388,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":551,"record_offset":18780,"parent_record_offset":18756,"ancestor_record_offset":15268,"inlinee":4862,"source_line":395,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$16::get::closure$0
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":551,"record_offset":19236,"parent_record_offset":19212,"ancestor_record_offset":15268,"inlinee":4866,"source_line":396,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_column
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":551,"record_offset":18756,"parent_record_offset":18732,"ancestor_record_offset":15268,"inlinee":4860,"source_line":475,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_data_slice_for
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":551,"record_offset":19136,"parent_record_offset":19112,"ancestor_record_offset":15268,"inlinee":4859,"source_line":363,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::has_column
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":551,"record_offset":18900,"parent_record_offset":18876,"ancestor_record_offset":15268,"inlinee":17017,"source_line":496,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::is_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":551,"record_offset":18620,"parent_record_offset":18292,"ancestor_record_offset":15268,"inlinee":11675,"source_line":615,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::TableRow::as_usize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":551,"record_offset":22400,"parent_record_offset":22372,"ancestor_record_offset":15268,"inlinee":5472,"source_line":127,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":551,"record_offset":18536,"parent_record_offset":18292,"ancestor_record_offset":15268,"inlinee":4854,"source_line":721,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::column::ThinColumn::get_changed_ticks_slice
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/column.rs
//     evidence={"module":551,"record_offset":20012,"parent_record_offset":19724,"ancestor_record_offset":15268,"inlinee":13683,"source_line":319,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::column::ThinColumn::get_data_slice
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/column.rs
//     evidence={"module":551,"record_offset":19352,"parent_record_offset":19328,"ancestor_record_offset":15268,"inlinee":4870,"source_line":303,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::impl$4::get_data_slice_for::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":551,"record_offset":19328,"parent_record_offset":19304,"ancestor_record_offset":15268,"inlinee":4868,"source_line":368,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::thin_array_ptr::ThinArrayPtr::as_slice
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/thin_array_ptr.rs
//     evidence={"module":551,"record_offset":20036,"parent_record_offset":20012,"ancestor_record_offset":15268,"inlinee":13684,"source_line":300,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::copy_unsafe
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":551,"record_offset":17936,"parent_record_offset":17912,"ancestor_record_offset":15268,"inlinee":13707,"source_line":636,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::reborrow
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":551,"record_offset":17884,"parent_record_offset":17860,"ancestor_record_offset":15268,"inlinee":17021,"source_line":601,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::reborrow_unsafe
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":551,"record_offset":17912,"parent_record_offset":17884,"ancestor_record_offset":15268,"inlinee":13706,"source_line":618,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ptr::ThinSlicePtr::get
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"bevy_ptr","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ptr-0.16.0/src/lib.rs
//     evidence={"module":551,"record_offset":22464,"parent_record_offset":22372,"ancestor_record_offset":15268,"inlinee":4880,"source_line":526,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::bool::impl$0::then
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\bool.rs
//     evidence={"module":551,"record_offset":22556,"parent_record_offset":22532,"ancestor_record_offset":15268,"inlinee":4781,"source_line":61,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":551,"record_offset":17212,"parent_record_offset":17184,"ancestor_record_offset":15268,"inlinee":4432,"source_line":1817,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":551,"record_offset":17264,"parent_record_offset":17240,"ancestor_record_offset":15268,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":551,"record_offset":17240,"parent_record_offset":17212,"ancestor_record_offset":15268,"inlinee":4434,"source_line":428,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":551,"record_offset":17160,"parent_record_offset":17132,"ancestor_record_offset":15268,"inlinee":4429,"source_line":1089,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":551,"record_offset":17184,"parent_record_offset":17160,"ancestor_record_offset":15268,"inlinee":4430,"source_line":1121,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":551,"record_offset":17356,"parent_record_offset":17184,"ancestor_record_offset":15268,"inlinee":4439,"source_line":2194,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$44::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":551,"record_offset":17468,"parent_record_offset":17444,"ancestor_record_offset":15268,"inlinee":4443,"source_line":1808,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":551,"record_offset":17288,"parent_record_offset":17264,"ancestor_record_offset":15268,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\mod.rs
//     evidence={"module":551,"record_offset":15616,"parent_record_offset":15584,"ancestor_record_offset":15268,"inlinee":4377,"source_line":1044,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":551,"record_offset":17028,"parent_record_offset":16988,"ancestor_record_offset":15268,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":551,"record_offset":21928,"parent_record_offset":21904,"ancestor_record_offset":15268,"inlinee":4518,"source_line":1447,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":551,"record_offset":21168,"parent_record_offset":21144,"ancestor_record_offset":15268,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":551,"record_offset":19056,"parent_record_offset":19032,"ancestor_record_offset":15268,"inlinee":4512,"source_line":607,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":551,"record_offset":19212,"parent_record_offset":19188,"ancestor_record_offset":15268,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":551,"record_offset":15864,"parent_record_offset":15836,"ancestor_record_offset":15268,"inlinee":4384,"source_line":933,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":551,"record_offset":17816,"parent_record_offset":16988,"ancestor_record_offset":15268,"inlinee":4795,"source_line":296,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":551,"record_offset":17420,"parent_record_offset":17132,"ancestor_record_offset":15268,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":551,"record_offset":18216,"parent_record_offset":18192,"ancestor_record_offset":15268,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":551,"record_offset":18496,"parent_record_offset":18472,"ancestor_record_offset":15268,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":551,"record_offset":16328,"parent_record_offset":16304,"ancestor_record_offset":15268,"inlinee":4410,"source_line":533,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":551,"record_offset":16304,"parent_record_offset":16280,"ancestor_record_offset":15268,"inlinee":4409,"source_line":686,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":551,"record_offset":16512,"parent_record_offset":16484,"ancestor_record_offset":15268,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":551,"record_offset":17312,"parent_record_offset":17288,"ancestor_record_offset":15268,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":551,"record_offset":18560,"parent_record_offset":18536,"ancestor_record_offset":15268,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_noubcheck
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":551,"record_offset":19904,"parent_record_offset":19880,"ancestor_record_offset":15268,"inlinee":4164,"source_line":87,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":551,"record_offset":18584,"parent_record_offset":18560,"ancestor_record_offset":15268,"inlinee":4145,"source_line":226,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":551,"record_offset":19880,"parent_record_offset":19848,"ancestor_record_offset":15268,"inlinee":4162,"source_line":242,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":551,"record_offset":18192,"parent_record_offset":18020,"ancestor_record_offset":15268,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":551,"record_offset":16256,"parent_record_offset":16232,"ancestor_record_offset":15268,"inlinee":4407,"source_line":80,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":551,"record_offset":18472,"parent_record_offset":18448,"ancestor_record_offset":15268,"inlinee":4510,"source_line":99,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":551,"record_offset":16280,"parent_record_offset":16256,"ancestor_record_offset":15268,"inlinee":4408,"source_line":118,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":551,"record_offset":16584,"parent_record_offset":16556,"ancestor_record_offset":15268,"inlinee":4402,"source_line":279,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":551,"record_offset":18384,"parent_record_offset":18292,"ancestor_record_offset":15268,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$183::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":551,"record_offset":16232,"parent_record_offset":16116,"ancestor_record_offset":15268,"inlinee":4405,"source_line":392,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$186::next_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":551,"record_offset":18448,"parent_record_offset":18384,"ancestor_record_offset":15268,"inlinee":4509,"source_line":440,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":551,"record_offset":15584,"parent_record_offset":15556,"ancestor_record_offset":15268,"inlinee":4375,"source_line":366,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":551,"record_offset":15944,"parent_record_offset":15916,"ancestor_record_offset":15268,"inlinee":4386,"source_line":1488,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":551,"record_offset":16084,"parent_record_offset":16052,"ancestor_record_offset":15268,"inlinee":4396,"source_line":128,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":551,"record_offset":16052,"parent_record_offset":16016,"ancestor_record_offset":15268,"inlinee":4394,"source_line":218,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":551,"record_offset":15992,"parent_record_offset":15968,"ancestor_record_offset":15268,"inlinee":4390,"source_line":315,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":551,"record_offset":16016,"parent_record_offset":15992,"ancestor_record_offset":15268,"inlinee":4392,"source_line":709,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":551,"record_offset":15968,"parent_record_offset":15944,"ancestor_record_offset":15268,"inlinee":4388,"source_line":798,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":551,"record_offset":16556,"parent_record_offset":16016,"ancestor_record_offset":15268,"inlinee":4400,"source_line":659,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":551,"record_offset":15556,"parent_record_offset":15524,"ancestor_record_offset":15268,"inlinee":4373,"source_line":433,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":551,"record_offset":15524,"parent_record_offset":15496,"ancestor_record_offset":15268,"inlinee":4372,"source_line":463,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":551,"record_offset":15716,"parent_record_offset":15684,"ancestor_record_offset":15268,"inlinee":4380,"source_line":501,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":551,"record_offset":15812,"parent_record_offset":15716,"ancestor_record_offset":15268,"inlinee":4381,"source_line":521,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":551,"record_offset":15684,"parent_record_offset":15660,"ancestor_record_offset":15268,"inlinee":4379,"source_line":533,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":551,"record_offset":15496,"parent_record_offset":15460,"ancestor_record_offset":15268,"inlinee":4370,"source_line":59,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":551,"record_offset":15836,"parent_record_offset":15812,"ancestor_record_offset":15268,"inlinee":4383,"source_line":189,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":551,"record_offset":16116,"parent_record_offset":16084,"ancestor_record_offset":15268,"inlinee":4398,"source_line":79,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":551,"record_offset":16400,"parent_record_offset":16116,"ancestor_record_offset":15268,"inlinee":4412,"source_line":17,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":551,"record_offset":16372,"parent_record_offset":16116,"ancestor_record_offset":15268,"inlinee":4411,"source_line":11,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":551,"record_offset":16176,"parent_record_offset":16116,"ancestor_record_offset":15268,"inlinee":4403,"source_line":24,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"nonmax","version":"0.5.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":551,"record_offset":19260,"parent_record_offset":19236,"ancestor_record_offset":15268,"inlinee":4171,"source_line":153,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":551,"record_offset":16924,"parent_record_offset":16892,"ancestor_record_offset":15268,"inlinee":4418,"source_line":425,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":551,"record_offset":24084,"parent_record_offset":24052,"ancestor_record_offset":15268,"inlinee":4966,"source_line":601,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":551,"record_offset":24584,"parent_record_offset":24324,"ancestor_record_offset":15268,"inlinee":4968,"source_line":602,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":551,"record_offset":17056,"parent_record_offset":17028,"ancestor_record_offset":15268,"inlinee":4423,"source_line":90,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":551,"record_offset":17132,"parent_record_offset":16988,"ancestor_record_offset":15268,"inlinee":4427,"source_line":429,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":551,"record_offset":17392,"parent_record_offset":17132,"ancestor_record_offset":15268,"inlinee":4441,"source_line":591,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":551,"record_offset":17084,"parent_record_offset":17056,"ancestor_record_offset":15268,"inlinee":4425,"source_line":51,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":551,"record_offset":16988,"parent_record_offset":16956,"ancestor_record_offset":15268,"inlinee":4421,"source_line":305,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":551,"record_offset":16956,"parent_record_offset":16924,"ancestor_record_offset":15268,"inlinee":4420,"source_line":270,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::ui::signifiers::animation::signifier_animation
// 记录形式: compiled_procedure; instances=1
// 源码行: 44
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":551,"record_offset":25176,"rva":10918960,"va":5379628080}
// 编译实例 RVAs: 0xa69c30
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1428f87d0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: animation.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t32040065","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379628080,"raw_hex":"e881e4e801","size":5,"soft_fail":false,"target_va":5411669968,"va":5379629898,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t31911623","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379628080,"raw_hex":"e8c7eee601","size":5,"soft_fail":false,"target_va":5411541584,"va":5379629956,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t31912320","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379628080,"raw_hex":"e880f1e601","size":5,"soft_fail":false,"target_va":5411542296,"va":5379629971,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-10491","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379628080,"raw_hex":"e805d7ffff","size":5,"soft_fail":false,"target_va":5379617664,"va":5379628150,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t31915135","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379628080,"raw_hex":"e87ffce601","size":5,"soft_fail":false,"target_va":5411543360,"va":5379628220,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t1756858","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379628080,"raw_hex":"e8bace1a00","size":5,"soft_fail":false,"target_va":5381385536,"va":5379628673,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t1762044","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379628080,"raw_hex":"e8fce21a00","size":5,"soft_fail":false,"target_va":5381390640,"va":5379628591,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::end_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t1756616","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379628080,"raw_hex":"e8c8cd1a00","size":5,"soft_fail":false,"target_va":5381386480,"va":5379629859,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t1756578","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379628080,"raw_hex":"e8a2cd1a00","size":5,"soft_fail":false,"target_va":5381384704,"va":5379628121,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::LocalKey$LT$T$GT$::with
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t9195245","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379628080,"raw_hex":"e8ed4e8c00","size":5,"soft_fail":false,"target_va":5388825296,"va":5379630046,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t31907155","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379628080,"raw_hex":"e853dde601","size":5,"soft_fail":false,"target_va":5411537104,"va":5379629944,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$58::deref_mut
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":551,"record_offset":33332,"parent_record_offset":25176,"ancestor_record_offset":25176,"inlinee":9177,"source_line":434,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::query::iter::impl$1::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs
//     evidence={"module":551,"record_offset":28184,"parent_record_offset":25176,"ancestor_record_offset":25176,"inlinee":11673,"source_line":884,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::query::Query::iter_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":551,"record_offset":27776,"parent_record_offset":25176,"ancestor_record_offset":25176,"inlinee":17020,"source_line":701,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":551,"record_offset":33652,"parent_record_offset":25176,"ancestor_record_offset":25176,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui::resources::ui_state::UiState::origin
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/resources/ui_state.rs
//     evidence={"module":551,"record_offset":33388,"parent_record_offset":25176,"ancestor_record_offset":25176,"inlinee":28904,"source_line":212,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":551,"record_offset":26792,"parent_record_offset":25176,"ancestor_record_offset":25176,"inlinee":4416,"source_line":589,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":551,"record_offset":25360,"parent_record_offset":25176,"ancestor_record_offset":25176,"inlinee":4368,"source_line":626,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":551,"record_offset":25816,"parent_record_offset":25176,"ancestor_record_offset":25176,"inlinee":4385,"source_line":659,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":551,"record_offset":28016,"parent_record_offset":27992,"ancestor_record_offset":25176,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":551,"record_offset":28064,"parent_record_offset":28040,"ancestor_record_offset":25176,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":551,"record_offset":28040,"parent_record_offset":28016,"ancestor_record_offset":25176,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":551,"record_offset":27992,"parent_record_offset":27968,"ancestor_record_offset":25176,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":551,"record_offset":27968,"parent_record_offset":27936,"ancestor_record_offset":25176,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":551,"record_offset":28560,"parent_record_offset":28536,"ancestor_record_offset":25176,"inlinee":4613,"source_line":2705,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":551,"record_offset":28584,"parent_record_offset":28560,"ancestor_record_offset":25176,"inlinee":4352,"source_line":2680,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$13::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":551,"record_offset":33492,"parent_record_offset":33452,"ancestor_record_offset":25176,"inlinee":4358,"source_line":3357,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":551,"record_offset":27936,"parent_record_offset":27908,"ancestor_record_offset":25176,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::contains
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":551,"record_offset":31460,"parent_record_offset":31432,"ancestor_record_offset":25176,"inlinee":5673,"source_line":621,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::entities
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":551,"record_offset":32552,"parent_record_offset":28208,"ancestor_record_offset":25176,"inlinee":11682,"source_line":466,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::is_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":551,"record_offset":30504,"parent_record_offset":28208,"ancestor_record_offset":25176,"inlinee":11679,"source_line":615,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::table_id
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":551,"record_offset":30588,"parent_record_offset":28208,"ancestor_record_offset":25176,"inlinee":5495,"source_line":460,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::ArchetypeEntity::table_row
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":551,"record_offset":32748,"parent_record_offset":28208,"ancestor_record_offset":25176,"inlinee":17012,"source_line":325,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetypes::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":551,"record_offset":30420,"parent_record_offset":28208,"ancestor_record_offset":25176,"inlinee":11678,"source_line":895,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$57::set_changed
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":551,"record_offset":33356,"parent_record_offset":33332,"ancestor_record_offset":25176,"inlinee":9179,"source_line":397,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::StorageSwitch::extract
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":551,"record_offset":32852,"parent_record_offset":32828,"ancestor_record_offset":25176,"inlinee":4876,"source_line":2472,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$26::matches_component_set
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":551,"record_offset":29284,"parent_record_offset":29244,"ancestor_record_offset":25176,"inlinee":5720,"source_line":1210,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$26::set_archetype
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":551,"record_offset":30708,"parent_record_offset":30680,"ancestor_record_offset":25176,"inlinee":4856,"source_line":1160,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$26::set_table
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":551,"record_offset":28656,"parent_record_offset":28620,"ancestor_record_offset":25176,"inlinee":4857,"source_line":1175,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$27::fetch
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":551,"record_offset":32828,"parent_record_offset":32804,"ancestor_record_offset":25176,"inlinee":4874,"source_line":1229,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$27::fetch::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":551,"record_offset":32876,"parent_record_offset":32852,"ancestor_record_offset":25176,"inlinee":4878,"source_line":1235,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::set_archetype
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":551,"record_offset":31232,"parent_record_offset":30680,"ancestor_record_offset":25176,"inlinee":13655,"source_line":1524,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::set_table
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":551,"record_offset":29148,"parent_record_offset":28620,"ancestor_record_offset":25176,"inlinee":13656,"source_line":1539,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$37::fetch::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":551,"record_offset":33140,"parent_record_offset":33116,"ancestor_record_offset":25176,"inlinee":9533,"source_line":1602,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$41::set_archetype
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":551,"record_offset":31360,"parent_record_offset":30680,"ancestor_record_offset":25176,"inlinee":17008,"source_line":1799,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$41::set_archetype::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":551,"record_offset":31432,"parent_record_offset":31396,"ancestor_record_offset":25176,"inlinee":17010,"source_line":1805,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$41::set_table
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":551,"record_offset":29244,"parent_record_offset":28620,"ancestor_record_offset":25176,"inlinee":17014,"source_line":1815,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$41::set_table::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":551,"record_offset":29328,"parent_record_offset":29284,"ancestor_record_offset":25176,"inlinee":17016,"source_line":1816,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$65::fetch
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":551,"record_offset":32804,"parent_record_offset":28208,"ancestor_record_offset":25176,"inlinee":17024,"source_line":2083,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIter::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs
//     evidence={"module":551,"record_offset":27884,"parent_record_offset":27800,"ancestor_record_offset":25176,"inlinee":13709,"source_line":39,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::init
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs
//     evidence={"module":551,"record_offset":27908,"parent_record_offset":27884,"ancestor_record_offset":25176,"inlinee":9292,"source_line":2359,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs
//     evidence={"module":551,"record_offset":28208,"parent_record_offset":28184,"ancestor_record_offset":25176,"inlinee":11674,"source_line":2458,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$4::set_archetype
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/world_query.rs
//     evidence={"module":551,"record_offset":30680,"parent_record_offset":28208,"ancestor_record_offset":25176,"inlinee":17025,"source_line":182,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$4::set_table
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/world_query.rs
//     evidence={"module":551,"record_offset":28620,"parent_record_offset":28208,"ancestor_record_offset":25176,"inlinee":17022,"source_line":195,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_array::BlobArray::get_sub_slice
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_array.rs
//     evidence={"module":551,"record_offset":29100,"parent_record_offset":29076,"ancestor_record_offset":25176,"inlinee":4872,"source_line":143,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseArray::contains
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":551,"record_offset":29432,"parent_record_offset":29396,"ancestor_record_offset":25176,"inlinee":5675,"source_line":49,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseArray::get
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":551,"record_offset":28772,"parent_record_offset":28744,"ancestor_record_offset":25176,"inlinee":4864,"source_line":58,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::contains
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":551,"record_offset":29396,"parent_record_offset":29360,"ancestor_record_offset":25176,"inlinee":5674,"source_line":388,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":551,"record_offset":28744,"parent_record_offset":28716,"ancestor_record_offset":25176,"inlinee":4862,"source_line":395,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$16::get::closure$0
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":551,"record_offset":28960,"parent_record_offset":28936,"ancestor_record_offset":25176,"inlinee":4866,"source_line":396,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_column
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":551,"record_offset":28716,"parent_record_offset":28684,"ancestor_record_offset":25176,"inlinee":4860,"source_line":475,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_data_slice_for
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":551,"record_offset":28684,"parent_record_offset":28656,"ancestor_record_offset":25176,"inlinee":4859,"source_line":363,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::has_column
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":551,"record_offset":29360,"parent_record_offset":29328,"ancestor_record_offset":25176,"inlinee":17017,"source_line":496,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::is_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":551,"record_offset":28536,"parent_record_offset":28208,"ancestor_record_offset":25176,"inlinee":11675,"source_line":615,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::TableRow::as_usize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":551,"record_offset":32900,"parent_record_offset":32876,"ancestor_record_offset":25176,"inlinee":5472,"source_line":127,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":551,"record_offset":28452,"parent_record_offset":28208,"ancestor_record_offset":25176,"inlinee":4854,"source_line":721,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::column::ThinColumn::get_data_slice
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/column.rs
//     evidence={"module":551,"record_offset":29076,"parent_record_offset":29052,"ancestor_record_offset":25176,"inlinee":4870,"source_line":303,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::impl$4::get_data_slice_for::closure$0
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":551,"record_offset":29052,"parent_record_offset":29028,"ancestor_record_offset":25176,"inlinee":4868,"source_line":368,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::copy_unsafe
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":551,"record_offset":27852,"parent_record_offset":27828,"ancestor_record_offset":25176,"inlinee":13707,"source_line":636,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::reborrow
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":551,"record_offset":27800,"parent_record_offset":27776,"ancestor_record_offset":25176,"inlinee":17021,"source_line":601,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::reborrow_unsafe
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":551,"record_offset":27828,"parent_record_offset":27800,"ancestor_record_offset":25176,"inlinee":13706,"source_line":618,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ptr::ThinSlicePtr::get
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ptr","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ptr-0.16.0/src/lib.rs
//     evidence={"module":551,"record_offset":33052,"parent_record_offset":33028,"ancestor_record_offset":25176,"inlinee":4880,"source_line":526,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::bool::impl$0::then
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\bool.rs
//     evidence={"module":551,"record_offset":33288,"parent_record_offset":32968,"ancestor_record_offset":25176,"inlinee":4781,"source_line":61,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":551,"record_offset":27128,"parent_record_offset":27100,"ancestor_record_offset":25176,"inlinee":4432,"source_line":1817,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":551,"record_offset":27180,"parent_record_offset":27156,"ancestor_record_offset":25176,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":551,"record_offset":27156,"parent_record_offset":27128,"ancestor_record_offset":25176,"inlinee":4434,"source_line":428,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":551,"record_offset":27076,"parent_record_offset":27048,"ancestor_record_offset":25176,"inlinee":4429,"source_line":1089,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":551,"record_offset":27100,"parent_record_offset":27076,"ancestor_record_offset":25176,"inlinee":4430,"source_line":1121,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":551,"record_offset":27272,"parent_record_offset":27100,"ancestor_record_offset":25176,"inlinee":4439,"source_line":2194,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$44::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":551,"record_offset":27384,"parent_record_offset":27360,"ancestor_record_offset":25176,"inlinee":4443,"source_line":1808,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":551,"record_offset":27204,"parent_record_offset":27180,"ancestor_record_offset":25176,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\mod.rs
//     evidence={"module":551,"record_offset":25516,"parent_record_offset":25484,"ancestor_record_offset":25176,"inlinee":4377,"source_line":1044,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":551,"record_offset":26944,"parent_record_offset":26900,"ancestor_record_offset":25176,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":551,"record_offset":28892,"parent_record_offset":28864,"ancestor_record_offset":25176,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":551,"record_offset":29572,"parent_record_offset":29548,"ancestor_record_offset":25176,"inlinee":4512,"source_line":607,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":551,"record_offset":28936,"parent_record_offset":28744,"ancestor_record_offset":25176,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":551,"record_offset":25764,"parent_record_offset":25736,"ancestor_record_offset":25176,"inlinee":4384,"source_line":933,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":551,"record_offset":27732,"parent_record_offset":26900,"ancestor_record_offset":25176,"inlinee":4795,"source_line":296,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":551,"record_offset":27336,"parent_record_offset":27048,"ancestor_record_offset":25176,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":551,"record_offset":28132,"parent_record_offset":28108,"ancestor_record_offset":25176,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":551,"record_offset":28412,"parent_record_offset":28388,"ancestor_record_offset":25176,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":551,"record_offset":26228,"parent_record_offset":26204,"ancestor_record_offset":25176,"inlinee":4410,"source_line":533,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":551,"record_offset":26204,"parent_record_offset":26180,"ancestor_record_offset":25176,"inlinee":4409,"source_line":686,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":551,"record_offset":26412,"parent_record_offset":26384,"ancestor_record_offset":25176,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":551,"record_offset":27228,"parent_record_offset":27204,"ancestor_record_offset":25176,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":551,"record_offset":28476,"parent_record_offset":28452,"ancestor_record_offset":25176,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":551,"record_offset":33520,"parent_record_offset":33492,"ancestor_record_offset":25176,"inlinee":4360,"source_line":15,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":551,"record_offset":28500,"parent_record_offset":28476,"ancestor_record_offset":25176,"inlinee":4145,"source_line":226,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":551,"record_offset":33548,"parent_record_offset":33520,"ancestor_record_offset":25176,"inlinee":4640,"source_line":272,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":551,"record_offset":28108,"parent_record_offset":27936,"ancestor_record_offset":25176,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":551,"record_offset":26156,"parent_record_offset":26132,"ancestor_record_offset":25176,"inlinee":4407,"source_line":80,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":551,"record_offset":28388,"parent_record_offset":28364,"ancestor_record_offset":25176,"inlinee":4510,"source_line":99,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":551,"record_offset":26180,"parent_record_offset":26156,"ancestor_record_offset":25176,"inlinee":4408,"source_line":118,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":551,"record_offset":26484,"parent_record_offset":26456,"ancestor_record_offset":25176,"inlinee":4402,"source_line":279,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":551,"record_offset":28300,"parent_record_offset":28208,"ancestor_record_offset":25176,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$183::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":551,"record_offset":26132,"parent_record_offset":26016,"ancestor_record_offset":25176,"inlinee":4405,"source_line":392,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$186::next_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":551,"record_offset":28364,"parent_record_offset":28300,"ancestor_record_offset":25176,"inlinee":4509,"source_line":440,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":551,"record_offset":25484,"parent_record_offset":25456,"ancestor_record_offset":25176,"inlinee":4375,"source_line":366,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":551,"record_offset":25844,"parent_record_offset":25816,"ancestor_record_offset":25176,"inlinee":4386,"source_line":1488,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":551,"record_offset":25984,"parent_record_offset":25952,"ancestor_record_offset":25176,"inlinee":4396,"source_line":128,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":551,"record_offset":25952,"parent_record_offset":25916,"ancestor_record_offset":25176,"inlinee":4394,"source_line":218,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":551,"record_offset":25892,"parent_record_offset":25868,"ancestor_record_offset":25176,"inlinee":4390,"source_line":315,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":551,"record_offset":25916,"parent_record_offset":25892,"ancestor_record_offset":25176,"inlinee":4392,"source_line":709,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":551,"record_offset":25868,"parent_record_offset":25844,"ancestor_record_offset":25176,"inlinee":4388,"source_line":798,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":551,"record_offset":26456,"parent_record_offset":25916,"ancestor_record_offset":25176,"inlinee":4400,"source_line":659,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":551,"record_offset":25456,"parent_record_offset":25424,"ancestor_record_offset":25176,"inlinee":4373,"source_line":433,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":551,"record_offset":25424,"parent_record_offset":25396,"ancestor_record_offset":25176,"inlinee":4372,"source_line":463,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":551,"record_offset":25616,"parent_record_offset":25584,"ancestor_record_offset":25176,"inlinee":4380,"source_line":501,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":551,"record_offset":25712,"parent_record_offset":25616,"ancestor_record_offset":25176,"inlinee":4381,"source_line":521,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":551,"record_offset":25584,"parent_record_offset":25560,"ancestor_record_offset":25176,"inlinee":4379,"source_line":533,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":551,"record_offset":25396,"parent_record_offset":25360,"ancestor_record_offset":25176,"inlinee":4370,"source_line":59,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":551,"record_offset":25736,"parent_record_offset":25712,"ancestor_record_offset":25176,"inlinee":4383,"source_line":189,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":551,"record_offset":26016,"parent_record_offset":25984,"ancestor_record_offset":25176,"inlinee":4398,"source_line":79,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":551,"record_offset":26300,"parent_record_offset":26016,"ancestor_record_offset":25176,"inlinee":4412,"source_line":17,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":551,"record_offset":26272,"parent_record_offset":26016,"ancestor_record_offset":25176,"inlinee":4411,"source_line":11,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":551,"record_offset":26076,"parent_record_offset":26016,"ancestor_record_offset":25176,"inlinee":4403,"source_line":24,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::ui::resources::ui_state::impl$13::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/resources/ui_state.rs
//     evidence={"module":551,"record_offset":33424,"parent_record_offset":33388,"ancestor_record_offset":25176,"inlinee":26163,"source_line":11,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::ui::resources::widget_params::WidgetVolumeShape::get_origin
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/resources/widget_params.rs
//     evidence={"module":551,"record_offset":33452,"parent_record_offset":33388,"ancestor_record_offset":25176,"inlinee":28901,"source_line":312,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"nonmax","version":"0.5.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":551,"record_offset":28984,"parent_record_offset":28960,"ancestor_record_offset":25176,"inlinee":4171,"source_line":153,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":551,"record_offset":26828,"parent_record_offset":26792,"ancestor_record_offset":25176,"inlinee":4418,"source_line":425,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":551,"record_offset":33716,"parent_record_offset":33684,"ancestor_record_offset":25176,"inlinee":4966,"source_line":601,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":551,"record_offset":34216,"parent_record_offset":33956,"ancestor_record_offset":25176,"inlinee":4968,"source_line":602,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":551,"record_offset":26972,"parent_record_offset":26944,"ancestor_record_offset":25176,"inlinee":4423,"source_line":90,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":551,"record_offset":27048,"parent_record_offset":26900,"ancestor_record_offset":25176,"inlinee":4427,"source_line":429,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":551,"record_offset":27308,"parent_record_offset":27048,"ancestor_record_offset":25176,"inlinee":4441,"source_line":591,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":551,"record_offset":27000,"parent_record_offset":26972,"ancestor_record_offset":25176,"inlinee":4425,"source_line":51,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":551,"record_offset":26900,"parent_record_offset":26864,"ancestor_record_offset":25176,"inlinee":4421,"source_line":305,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":551,"record_offset":26864,"parent_record_offset":26828,"ancestor_record_offset":25176,"inlinee":4420,"source_line":270,"archive":"_pdb/B/modules/00551.jsonl.gz"}
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
// 完整结构及原始类型记录: animation.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: animation.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: animation.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 2; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::systems::ui::signifiers::SignifierAnimation>,enum2$<core::option::Option<ref$<country_core::systems::ui::signifiers::SignifierMarkedForDeletion> > >,enum2$<core::option::Option<ref_mut$<country_core::systems::ui::signifiers::SignifierMinimumLifespan> > > >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)
// 候选参数类型: ["bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::systems::ui::signifiers::SignifierAnimation>,enum2$<core::option::Option<ref$<country_core::systems::ui::signifiers::SignifierMarkedForDeletion> > >,enum2$<core::option::Option<ref_mut$<country_core::systems::ui::signifiers::SignifierMinimumLifespan> > > >,tuple$<> >", "bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>"]
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::systems::ui::signifiers::SignifierAnimation>,enum2$<core::option::Option<ref$<country_core::systems::ui::signifiers::SignifierMarkedForDeletion> > >,enum2$<core::option::Option<ref_mut$<country_core::systems::ui::signifiers::SignifierMinimumLifespan> > > >,tuple$<> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::query::Query<tuple$<ref$<country_core::systems::ui::signifiers::SignifierAnimation>,ref_mut$<utils::components::transform::Transform>,enum2$<core::option::Option<ref$<country_core::systems::ui::signifiers::SignifierId> > >,enum2$<core::option::Option<ref$<country_core::systems::ui::signifiers::SignifierParentToVolumeOrigin> > > >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>)
// 候选参数类型: ["bevy_ecs::system::query::Query<tuple$<ref$<country_core::systems::ui::signifiers::SignifierAnimation>,ref_mut$<utils::components::transform::Transform>,enum2$<core::option::Option<ref$<country_core::systems::ui::signifiers::SignifierId> > >,enum2$<core::option::Option<ref$<country_core::systems::ui::signifiers::SignifierParentToVolumeOrigin> > > >,tuple$<> >", "bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>"]
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<ref$<country_core::systems::ui::signifiers::SignifierAnimation>,ref_mut$<utils::components::transform::Transform>,enum2$<core::option::Option<ref$<country_core::systems::ui::signifiers::SignifierId> > >,enum2$<core::option::Option<ref$<country_core::systems::ui::signifiers::SignifierParentToVolumeOrigin> > > >,tuple$<> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>

// ------------------------------------------------------------------
// 函数: country_core::systems::ui::signifiers::animation::signifier_animation
// 信息 ID: cf1f517b8bab85574403daa9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 707f5472713a8ad2baa199d0: void (*)(bevy_ecs::system::query::Query<tuple$<ref$<country_core::systems::ui::signifiers::SignifierAnimation>,ref_mut$<utils::components::transform::Transform>,enum2$<core::option::Option<ref$<country_core::systems::ui::signifiers::SignifierId> > >,enum2$<core::option::Option<ref$<country_core::systems::ui::signifiers::SignifierParentToVolumeOrigin> > > >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref$<country_core::systems::ui::signifiers::SignifierAnimation>,ref_mut$<utils::components::transform::Transform>,enum2$<core::option::Option<ref$<country_core::systems::ui::signifiers::SignifierId> > >,enum2$<core::option::Option<ref$<country_core::systems::ui::signifiers::SignifierParentToVolumeOrigin> > > >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>),void (*)(bevy_ecs::system::query::Query<tuple$<ref$<country_core::systems::ui::signifiers::SignifierAnimation>,ref_mut$<utils::components::transform::Transform>,enum2$<core::option::Option<ref$<country_core::systems::ui::signifiers::SignifierId> > >,enum2$<core::option::Option<ref$<country_core::systems::ui::signifiers::SignifierParentToVolumeOrigin> > > >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>)> > > @ 0x14c3bc5 -> 0xa69c30 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::schedule_system::impl$1::run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref$<country_core::systems::ui::signifiers::SignifierAnimation>,ref_mut$<utils::components::transform::Transform>,enum2$<core::option::Option<ref$<country_core::systems::ui::signifiers::SignifierId> > >,enum2$<core::option::Option<ref$<country_core::systems::ui::signifiers::SignifierParentToVolumeOrigin> > > >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>),void (*)(bevy_ecs::system::query::Query<tuple$<ref$<country_core::systems::ui::signifiers::SignifierAnimation>,ref_mut$<utils::components::transform::Transform>,enum2$<core::option::Option<ref$<country_core::systems::ui::signifiers::SignifierId> > >,enum2$<core::option::Option<ref$<country_core::systems::ui::signifiers::SignifierParentToVolumeOrigin> > > >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>)> > @ 0x17ca87c -> 0xa69c30 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::ui::signifiers::animation::signifier_animation_t
// 信息 ID: ce8c676668cfba878ecf5902
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 3e279b7af7ce4475a3647486: void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::systems::ui::signifiers::SignifierAnimation>,enum2$<core::option::Option<ref$<country_core::systems::ui::signifiers::SignifierMarkedForDeletion> > >,enum2$<core::option::Option<ref_mut$<country_core::systems::ui::signifiers::SignifierMinimumLifespan> > > >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::systems::ui::signifiers::SignifierAnimation>,enum2$<core::option::Option<ref$<country_core::systems::ui::signifiers::SignifierMarkedForDeletion> > >,enum2$<core::option::Option<ref_mut$<country_core::systems::ui::signifiers::SignifierMinimumLifespan> > > >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::systems::ui::signifiers::SignifierAnimation>,enum2$<core::option::Option<ref$<country_core::systems::ui::signifiers::SignifierMarkedForDeletion> > >,enum2$<core::option::Option<ref_mut$<country_core::systems::ui::signifiers::SignifierMinimumLifespan> > > >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)> > > @ 0x14fa245 -> 0xa69340 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::schedule_system::impl$1::run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::systems::ui::signifiers::SignifierAnimation>,enum2$<core::option::Option<ref$<country_core::systems::ui::signifiers::SignifierMarkedForDeletion> > >,enum2$<core::option::Option<ref_mut$<country_core::systems::ui::signifiers::SignifierMinimumLifespan> > > >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::systems::ui::signifiers::SignifierAnimation>,enum2$<core::option::Option<ref$<country_core::systems::ui::signifiers::SignifierMarkedForDeletion> > >,enum2$<core::option::Option<ref_mut$<country_core::systems::ui::signifiers::SignifierMinimumLifespan> > > >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)> > @ 0x17ca6fc -> 0xa69340 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
