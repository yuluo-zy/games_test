// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/systems/color/src/color_menu.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: fc23d1183574c323d5d5611360cef624707f7479de9d3d8dd5ba46e39b72125e
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: color_menu.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 3756
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
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.29.3"}]
// use lerp; // namespace candidate
//   package identities: [{"name":"lerp","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.0"}]
// use nonmax; // namespace candidate
//   package identities: [{"name":"nonmax","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.5.5"}]
// use std; // namespace candidate
//   package identities: []
// use system_color; // namespace candidate
//   package identities: [{"name":"system-color","source":"local_or_path","version":"0.1.0"}]
// use utils; // namespace candidate
//   package identities: [{"name":"utils","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_color::color_menu::color_button_animations
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3756,"record_offset":52368,"rva":36272560,"va":5404981680}
// 编译实例 RVAs: 0x22979b0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] bevy_ecs::query::iter::QueryIterationCursor$LT$D$C$F$GT$::init
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t3517272","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404981680,"raw_hex":"e858ab3500","size":5,"soft_fail":false,"target_va":5408499104,"va":5404981827,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::easing_functions::EasingFunction::apply
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-23076032","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404981680,"raw_hex":"e840e39ffe","size":5,"soft_fail":false,"target_va":5381906160,"va":5404982187,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$58::deref_mut
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":3756,"record_offset":52752,"parent_record_offset":52368,"ancestor_record_offset":52368,"inlinee":9177,"source_line":434,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::query::iter::impl$1::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs
//     evidence={"module":3756,"record_offset":53004,"parent_record_offset":52368,"ancestor_record_offset":52368,"inlinee":11673,"source_line":884,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::query::Query::iter_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":3756,"record_offset":52536,"parent_record_offset":52368,"ancestor_record_offset":52368,"inlinee":17020,"source_line":701,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":3756,"record_offset":52720,"parent_record_offset":52368,"ancestor_record_offset":52368,"inlinee":6356,"source_line":1409,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::min
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":3756,"record_offset":55564,"parent_record_offset":52368,"ancestor_record_offset":52368,"inlinee":4754,"source_line":912,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":3756,"record_offset":55904,"parent_record_offset":52368,"ancestor_record_offset":52368,"inlinee":4696,"source_line":693,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":3756,"record_offset":55680,"parent_record_offset":52368,"ancestor_record_offset":52368,"inlinee":4698,"source_line":1198,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$28::mul_assign
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":3756,"record_offset":52948,"parent_record_offset":52368,"ancestor_record_offset":52368,"inlinee":19787,"source_line":1232,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_direct] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lerp","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lerp-0.4.0/src/lib.rs
//     evidence={"module":3756,"record_offset":52808,"parent_record_offset":52368,"ancestor_record_offset":52368,"inlinee":4713,"source_line":171,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3756,"record_offset":54732,"parent_record_offset":54708,"ancestor_record_offset":52368,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3756,"record_offset":54780,"parent_record_offset":54756,"ancestor_record_offset":52368,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3756,"record_offset":54756,"parent_record_offset":54732,"ancestor_record_offset":52368,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3756,"record_offset":54708,"parent_record_offset":54684,"ancestor_record_offset":52368,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3756,"record_offset":54684,"parent_record_offset":54660,"ancestor_record_offset":52368,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3756,"record_offset":53376,"parent_record_offset":53352,"ancestor_record_offset":52368,"inlinee":4613,"source_line":2705,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3756,"record_offset":53400,"parent_record_offset":53376,"ancestor_record_offset":52368,"inlinee":4352,"source_line":2680,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3756,"record_offset":54660,"parent_record_offset":54636,"ancestor_record_offset":52368,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::contains
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":3756,"record_offset":54272,"parent_record_offset":54248,"ancestor_record_offset":52368,"inlinee":5673,"source_line":621,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::entities
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":3756,"record_offset":54636,"parent_record_offset":53028,"ancestor_record_offset":52368,"inlinee":11682,"source_line":466,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::is_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":3756,"record_offset":54080,"parent_record_offset":53028,"ancestor_record_offset":52368,"inlinee":11679,"source_line":615,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::ArchetypeEntity::table_row
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":3756,"record_offset":54832,"parent_record_offset":53028,"ancestor_record_offset":52368,"inlinee":17012,"source_line":325,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetypes::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":3756,"record_offset":53996,"parent_record_offset":53028,"ancestor_record_offset":52368,"inlinee":11678,"source_line":895,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$57::set_changed
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":3756,"record_offset":52776,"parent_record_offset":52752,"ancestor_record_offset":52368,"inlinee":9179,"source_line":397,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::StorageSwitch::extract
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3756,"record_offset":54544,"parent_record_offset":54520,"ancestor_record_offset":52368,"inlinee":4876,"source_line":2472,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$26::matches_component_set
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3756,"record_offset":53572,"parent_record_offset":53548,"ancestor_record_offset":52368,"inlinee":5720,"source_line":1210,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::set_table
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3756,"record_offset":53464,"parent_record_offset":53436,"ancestor_record_offset":52368,"inlinee":13656,"source_line":1539,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$37::fetch
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3756,"record_offset":54520,"parent_record_offset":54496,"ancestor_record_offset":52368,"inlinee":13686,"source_line":1596,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$37::fetch::closure$0
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3756,"record_offset":54568,"parent_record_offset":54544,"ancestor_record_offset":52368,"inlinee":9533,"source_line":1602,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$41::set_archetype
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3756,"record_offset":54196,"parent_record_offset":54164,"ancestor_record_offset":52368,"inlinee":17008,"source_line":1799,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$41::set_archetype::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3756,"record_offset":54248,"parent_record_offset":54220,"ancestor_record_offset":52368,"inlinee":17010,"source_line":1805,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$41::set_table
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3756,"record_offset":53548,"parent_record_offset":53436,"ancestor_record_offset":52368,"inlinee":17014,"source_line":1815,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$41::set_table::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3756,"record_offset":53600,"parent_record_offset":53572,"ancestor_record_offset":52368,"inlinee":17016,"source_line":1816,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$65::fetch
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3756,"record_offset":54496,"parent_record_offset":53028,"ancestor_record_offset":52368,"inlinee":17024,"source_line":2083,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIter::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs
//     evidence={"module":3756,"record_offset":52644,"parent_record_offset":52560,"ancestor_record_offset":52368,"inlinee":13709,"source_line":39,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs
//     evidence={"module":3756,"record_offset":53028,"parent_record_offset":53004,"ancestor_record_offset":52368,"inlinee":11674,"source_line":2458,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$4::set_archetype
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/world_query.rs
//     evidence={"module":3756,"record_offset":54164,"parent_record_offset":53028,"ancestor_record_offset":52368,"inlinee":17025,"source_line":182,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$4::set_table
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/world_query.rs
//     evidence={"module":3756,"record_offset":53436,"parent_record_offset":53028,"ancestor_record_offset":52368,"inlinee":17022,"source_line":195,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseArray::contains
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":3756,"record_offset":53672,"parent_record_offset":53648,"ancestor_record_offset":52368,"inlinee":5675,"source_line":49,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::contains
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":3756,"record_offset":53648,"parent_record_offset":53624,"ancestor_record_offset":52368,"inlinee":5674,"source_line":388,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":3756,"record_offset":53512,"parent_record_offset":53488,"ancestor_record_offset":52368,"inlinee":4862,"source_line":395,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_column
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":3756,"record_offset":53488,"parent_record_offset":53464,"ancestor_record_offset":52368,"inlinee":4860,"source_line":475,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::has_column
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":3756,"record_offset":53624,"parent_record_offset":53600,"ancestor_record_offset":52368,"inlinee":17017,"source_line":496,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::is_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":3756,"record_offset":53352,"parent_record_offset":53028,"ancestor_record_offset":52368,"inlinee":11675,"source_line":615,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::TableRow::as_usize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":3756,"record_offset":54592,"parent_record_offset":54568,"ancestor_record_offset":52368,"inlinee":5472,"source_line":127,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":3756,"record_offset":53268,"parent_record_offset":53028,"ancestor_record_offset":52368,"inlinee":4854,"source_line":721,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::copy_unsafe
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":3756,"record_offset":52612,"parent_record_offset":52588,"ancestor_record_offset":52368,"inlinee":13707,"source_line":636,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::reborrow
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":3756,"record_offset":52560,"parent_record_offset":52536,"ancestor_record_offset":52368,"inlinee":17021,"source_line":601,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::reborrow_unsafe
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":3756,"record_offset":52588,"parent_record_offset":52560,"ancestor_record_offset":52368,"inlinee":13706,"source_line":618,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::archetypes
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":3756,"record_offset":52680,"parent_record_offset":52644,"ancestor_record_offset":52368,"inlinee":13723,"source_line":263,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ptr::ThinSlicePtr::get
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"bevy_ptr","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ptr-0.16.0/src/lib.rs
//     evidence={"module":3756,"record_offset":54988,"parent_record_offset":54960,"ancestor_record_offset":52368,"inlinee":4880,"source_line":526,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":3756,"record_offset":52916,"parent_record_offset":52808,"ancestor_record_offset":52368,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":3756,"record_offset":52860,"parent_record_offset":52808,"ancestor_record_offset":52368,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$417::mul_assign
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":3756,"record_offset":52972,"parent_record_offset":52948,"ancestor_record_offset":52368,"inlinee":13665,"source_line":886,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":3756,"record_offset":52832,"parent_record_offset":52808,"ancestor_record_offset":52368,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":3756,"record_offset":53756,"parent_record_offset":53696,"ancestor_record_offset":52368,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3756,"record_offset":53780,"parent_record_offset":53756,"ancestor_record_offset":52368,"inlinee":4512,"source_line":607,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3756,"record_offset":55012,"parent_record_offset":54988,"ancestor_record_offset":52368,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":3756,"record_offset":53228,"parent_record_offset":53204,"ancestor_record_offset":52368,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":3756,"record_offset":53152,"parent_record_offset":53116,"ancestor_record_offset":52368,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":3756,"record_offset":53292,"parent_record_offset":53268,"ancestor_record_offset":52368,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":3756,"record_offset":53316,"parent_record_offset":53292,"ancestor_record_offset":52368,"inlinee":4145,"source_line":226,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":3756,"record_offset":53204,"parent_record_offset":53180,"ancestor_record_offset":52368,"inlinee":4510,"source_line":99,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":3756,"record_offset":53116,"parent_record_offset":53028,"ancestor_record_offset":52368,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$186::next_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":3756,"record_offset":53180,"parent_record_offset":53116,"ancestor_record_offset":52368,"inlinee":4509,"source_line":440,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":3756,"record_offset":55932,"parent_record_offset":55904,"ancestor_record_offset":52368,"inlinee":4698,"source_line":1198,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":3756,"record_offset":56016,"parent_record_offset":55904,"ancestor_record_offset":52368,"inlinee":4702,"source_line":1283,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_color::color_menu::color_menu
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3756,"record_offset":512,"rva":36246064,"va":5404955184}
// 编译实例 RVAs: 0x2291230
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400885d0
//     owner=unresolved_namespace; count=2
//     target_aliases=12 candidates; complete list: color_menu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-35707386","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404955184,"raw_hex":"e80626dffd","size":5,"soft_fail":false,"target_va":5369267664,"va":5404975045,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=21
//     target_aliases=1 candidates; complete list: color_menu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-29880092","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404955184,"raw_hex":"e8e41038fe","size":5,"soft_fail":false,"target_va":5375081488,"va":5404961575,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=16
//     target_aliases=1 candidates; complete list: color_menu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-29880398","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404955184,"raw_hex":"e8b20f38fe","size":5,"soft_fail":false,"target_va":5375081504,"va":5404961897,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140833230
//     owner=unresolved_namespace; count=2
//     target_aliases=9 candidates; complete list: color_menu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-27657733","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404955184,"raw_hex":"e8fbf959fe","size":5,"soft_fail":false,"target_va":5377307184,"va":5404964912,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140895470
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: color_menu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-27272074","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404955184,"raw_hex":"e876dc5ffe","size":5,"soft_fail":false,"target_va":5377709168,"va":5404981237,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408c5670
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: color_menu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-27069643","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404955184,"raw_hex":"e835f362fe","size":5,"soft_fail":false,"target_va":5377906288,"va":5404975926,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1422bd120
//     owner=unresolved_namespace; count=12
//     target_aliases=2 candidates; complete list: color_menu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t174727","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404955184,"raw_hex":"e887aa0200","size":5,"soft_fail":false,"target_va":5405135136,"va":5404960404,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t6562681","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404955184,"raw_hex":"e879236400","size":5,"soft_fail":false,"target_va":5411540992,"va":5404978306,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=12
//     evidence={"assembly":"\tcallq\t-22120083","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404955184,"raw_hex":"e86d79aefe","size":5,"soft_fail":false,"target_va":5382840208,"va":5404960286,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=29
//     evidence={"assembly":"\tcallq\t-30603326","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404955184,"raw_hex":"e8c2072dfe","size":5,"soft_fail":false,"target_va":5374354992,"va":5404958313,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=5
//     evidence={"assembly":"\tcallq\t6562667","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404955184,"raw_hex":"e86b236400","size":5,"soft_fail":false,"target_va":5411540963,"va":5404978291,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t6563431","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404955184,"raw_hex":"e867266400","size":5,"soft_fail":false,"target_va":5411541776,"va":5404978340,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t6564040","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404955184,"raw_hex":"e8c8286400","size":5,"soft_fail":false,"target_va":5411542296,"va":5404978251,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t6564642","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404955184,"raw_hex":"e8222b6400","size":5,"soft_fail":false,"target_va":5411542880,"va":5404978233,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::input::action_set::DigitalActionState::just_pressed
//     owner=package_namespace_candidate; count=14
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-26550546","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404955184,"raw_hex":"e8eede6afe","size":5,"soft_fail":false,"target_va":5378407056,"va":5404957597,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::color_icon::ColorMenuBrightnessValue::to_f32
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-24697490","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404955184,"raw_hex":"e86e2587fe","size":5,"soft_fail":false,"target_va":5380269936,"va":5404967421,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::history::History::add
//     owner=package_namespace_candidate; count=13
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-25230183","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404955184,"raw_hex":"e899047ffe","size":5,"soft_fail":false,"target_va":5379739328,"va":5404969506,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::history::History::try_start_input_recording
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-25224000","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404955184,"raw_hex":"e8c01c7ffe","size":5,"soft_fail":false,"target_va":5379745040,"va":5404969035,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::inner_wall_state::FreehandHeightType::toggle
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-27057505","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404955184,"raw_hex":"e89f2263fe","size":5,"soft_fail":false,"target_va":5377906272,"va":5404963772,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_walls::PublicWalls::get
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-25180442","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404955184,"raw_hex":"e8e6c67ffe","size":5,"soft_fail":false,"target_va":5379792624,"va":5404973061,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_walls::PublicWalls::roof_entity
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-25179594","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404955184,"raw_hex":"e836ca7ffe","size":5,"soft_fail":false,"target_va":5379793584,"va":5404973173,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::resources::ui_state::UiWidgetContext::clicked
//     owner=package_namespace_candidate; count=11
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-26624387","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404955184,"raw_hex":"e87dbe69fe","size":5,"soft_fail":false,"target_va":5378336240,"va":5404960622,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::resources::ui_state::UiWidgetContext::is_active
//     owner=package_namespace_candidate; count=11
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-26624047","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404955184,"raw_hex":"e8d1bf69fe","size":5,"soft_fail":false,"target_va":5378336384,"va":5404960426,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::resources::widget_params::UiSounds::empty
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-26121611","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404955184,"raw_hex":"e8756a71fe","size":5,"soft_fail":false,"target_va":5378838448,"va":5404960054,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::resources::widget_params::UiSounds::on_click
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-26121602","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404955184,"raw_hex":"e87e6a71fe","size":5,"soft_fail":false,"target_va":5378838560,"va":5404960157,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::resources::widget_params::UiSounds::on_hover_start
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-26121637","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404955184,"raw_hex":"e85b6a71fe","size":5,"soft_fail":false,"target_va":5378838480,"va":5404960112,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::resources::widget_params::WidgetParams::new
//     owner=package_namespace_candidate; count=12
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-26122349","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404955184,"raw_hex":"e8936771fe","size":5,"soft_fail":false,"target_va":5378838032,"va":5404960376,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::resources::widget_params::WidgetParams::occlude_by_world
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-26130021","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404955184,"raw_hex":"e89b4971fe","size":5,"soft_fail":false,"target_va":5378838240,"va":5404968256,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::resources::widget_params::WidgetParams::sounds
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-26133134","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404955184,"raw_hex":"e8723d71fe","size":5,"soft_fail":false,"target_va":5378838144,"va":5404971273,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::angle_range::AngleRange::clamp
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3527944","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404955184,"raw_hex":"e808d53500","size":5,"soft_fail":false,"target_va":5408498896,"va":5404970947,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::cartesian_to_polar
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-23085426","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404955184,"raw_hex":"e88ebe9ffe","size":5,"soft_fail":false,"target_va":5381885472,"va":5404970893,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::polar_to_cartesian
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-23085423","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404955184,"raw_hex":"e891be9ffe","size":5,"soft_fail":false,"target_va":5381885552,"va":5404970970,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":3756,"record_offset":15144,"parent_record_offset":512,"ancestor_record_offset":512,"inlinee":7051,"source_line":644,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_direct] alloc::sync::Arc::new
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":3756,"record_offset":27752,"parent_record_offset":512,"ancestor_record_offset":512,"inlinee":4783,"source_line":386,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$13::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3756,"record_offset":39572,"parent_record_offset":512,"ancestor_record_offset":512,"inlinee":4358,"source_line":3357,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=37
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":3756,"record_offset":25364,"parent_record_offset":512,"ancestor_record_offset":512,"inlinee":4457,"source_line":434,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$58::deref_mut
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":3756,"record_offset":4444,"parent_record_offset":512,"ancestor_record_offset":512,"inlinee":9177,"source_line":434,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::writer::EventWriter::write
//     owner=registry_dependency_definition; count=33
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/writer.rs
//     evidence={"module":3756,"record_offset":1192,"parent_record_offset":512,"ancestor_record_offset":512,"inlinee":7657,"source_line":74,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::writer::EventWriter::write_default
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/writer.rs
//     evidence={"module":3756,"record_offset":26996,"parent_record_offset":512,"ancestor_record_offset":512,"inlinee":9992,"source_line":95,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::query::Query::get_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":3756,"record_offset":4884,"parent_record_offset":512,"ancestor_record_offset":512,"inlinee":13654,"source_line":1576,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::query::Query::iter_mut
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":3756,"record_offset":756,"parent_record_offset":512,"ancestor_record_offset":512,"inlinee":17020,"source_line":701,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":3756,"record_offset":31060,"parent_record_offset":512,"ancestor_record_offset":512,"inlinee":4974,"source_line":261,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":3756,"record_offset":16588,"parent_record_offset":512,"ancestor_record_offset":512,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":3756,"record_offset":1552,"parent_record_offset":512,"ancestor_record_offset":512,"inlinee":4732,"source_line":46,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_direct] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":3756,"record_offset":4716,"parent_record_offset":512,"ancestor_record_offset":512,"inlinee":4686,"source_line":848,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::and_then
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3756,"record_offset":34024,"parent_record_offset":512,"ancestor_record_offset":512,"inlinee":4518,"source_line":1447,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::as_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3756,"record_offset":4652,"parent_record_offset":512,"ancestor_record_offset":512,"inlinee":4130,"source_line":728,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::get_or_insert
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3756,"record_offset":4500,"parent_record_offset":512,"ancestor_record_offset":512,"inlinee":15140,"source_line":1642,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3756,"record_offset":29884,"parent_record_offset":512,"ancestor_record_offset":512,"inlinee":4512,"source_line":607,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3756,"record_offset":25420,"parent_record_offset":512,"ancestor_record_offset":512,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::map_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3756,"record_offset":37012,"parent_record_offset":512,"ancestor_record_offset":512,"inlinee":5577,"source_line":1173,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3756,"record_offset":4680,"parent_record_offset":512,"ancestor_record_offset":512,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_direct] core::option::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3756,"record_offset":14688,"parent_record_offset":512,"ancestor_record_offset":512,"inlinee":5835,"source_line":2186,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":3756,"record_offset":17700,"parent_record_offset":512,"ancestor_record_offset":512,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":3756,"record_offset":48808,"parent_record_offset":512,"ancestor_record_offset":512,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":3756,"record_offset":4800,"parent_record_offset":512,"ancestor_record_offset":512,"inlinee":4460,"source_line":1050,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::public_wall_state::PublicWallState::is_premade_shape
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_wall_state.rs
//     evidence={"module":3756,"record_offset":41628,"parent_record_offset":512,"ancestor_record_offset":512,"inlinee":24411,"source_line":113,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui::resources::widget_params::impl$62::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/resources/widget_params.rs
//     evidence={"module":3756,"record_offset":4564,"parent_record_offset":512,"ancestor_record_offset":512,"inlinee":19945,"source_line":337,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::distance
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":3756,"record_offset":30056,"parent_record_offset":512,"ancestor_record_offset":512,"inlinee":9564,"source_line":422,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":3756,"record_offset":30000,"parent_record_offset":512,"ancestor_record_offset":512,"inlinee":4702,"source_line":1283,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":3756,"record_offset":29912,"parent_record_offset":512,"ancestor_record_offset":512,"inlinee":9561,"source_line":1418,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_direct] system_color::color_menu::color_menu::closure$0
//     owner=private_source_ownership_candidate; count=11
//     packages=[{"name":"system-color","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/color/src/color_menu.rs
//     evidence={"module":3756,"record_offset":15228,"parent_record_offset":512,"ancestor_record_offset":512,"inlinee":121463,"source_line":255,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_direct] system_color::color_menu::color_menu::closure$1
//     owner=private_source_ownership_candidate; count=11
//     packages=[{"name":"system-color","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/color/src/color_menu.rs
//     evidence={"module":3756,"record_offset":15380,"parent_record_offset":512,"ancestor_record_offset":512,"inlinee":121464,"source_line":266,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_direct] system_color::color_menu::color_menu::closure$2
//     owner=private_source_ownership_candidate; count=11
//     packages=[{"name":"system-color","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/color/src/color_menu.rs
//     evidence={"module":3756,"record_offset":15580,"parent_record_offset":512,"ancestor_record_offset":512,"inlinee":121465,"source_line":292,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=21
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":3756,"record_offset":16908,"parent_record_offset":16884,"ancestor_record_offset":512,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=21
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":3756,"record_offset":16932,"parent_record_offset":16908,"ancestor_record_offset":512,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":3756,"record_offset":17876,"parent_record_offset":17852,"ancestor_record_offset":512,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":3756,"record_offset":27808,"parent_record_offset":27780,"ancestor_record_offset":512,"inlinee":4281,"source_line":347,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=21
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":3756,"record_offset":16884,"parent_record_offset":16856,"ancestor_record_offset":512,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":3756,"record_offset":17852,"parent_record_offset":17796,"ancestor_record_offset":512,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":3756,"record_offset":27780,"parent_record_offset":27752,"ancestor_record_offset":512,"inlinee":4280,"source_line":273,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":3756,"record_offset":15192,"parent_record_offset":15168,"ancestor_record_offset":512,"inlinee":7053,"source_line":654,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=39
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3756,"record_offset":1024,"parent_record_offset":1000,"ancestor_record_offset":512,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3756,"record_offset":16808,"parent_record_offset":16784,"ancestor_record_offset":512,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3756,"record_offset":17824,"parent_record_offset":17796,"ancestor_record_offset":512,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3756,"record_offset":17796,"parent_record_offset":17772,"ancestor_record_offset":512,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=39
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3756,"record_offset":1072,"parent_record_offset":1048,"ancestor_record_offset":512,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=39
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3756,"record_offset":1048,"parent_record_offset":1024,"ancestor_record_offset":512,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3756,"record_offset":16856,"parent_record_offset":16832,"ancestor_record_offset":512,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3756,"record_offset":16832,"parent_record_offset":16808,"ancestor_record_offset":512,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3756,"record_offset":17772,"parent_record_offset":17748,"ancestor_record_offset":512,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":3756,"record_offset":16756,"parent_record_offset":16732,"ancestor_record_offset":512,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":3756,"record_offset":16732,"parent_record_offset":16708,"ancestor_record_offset":512,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":3756,"record_offset":16684,"parent_record_offset":16660,"ancestor_record_offset":512,"inlinee":4247,"source_line":476,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":3756,"record_offset":16708,"parent_record_offset":16684,"ancestor_record_offset":512,"inlinee":4248,"source_line":500,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":3756,"record_offset":16660,"parent_record_offset":16636,"ancestor_record_offset":512,"inlinee":4245,"source_line":887,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\str.rs
//     evidence={"module":3756,"record_offset":16636,"parent_record_offset":16612,"ancestor_record_offset":512,"inlinee":4243,"source_line":210,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$47::from
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":3756,"record_offset":16612,"parent_record_offset":16588,"ancestor_record_offset":512,"inlinee":5924,"source_line":2956,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=29
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3756,"record_offset":1356,"parent_record_offset":1312,"ancestor_record_offset":512,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3756,"record_offset":1000,"parent_record_offset":976,"ancestor_record_offset":512,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3756,"record_offset":976,"parent_record_offset":944,"ancestor_record_offset":512,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3756,"record_offset":1952,"parent_record_offset":1928,"ancestor_record_offset":512,"inlinee":4613,"source_line":2705,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3756,"record_offset":1976,"parent_record_offset":1952,"ancestor_record_offset":512,"inlinee":4352,"source_line":2680,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=37
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3756,"record_offset":1312,"parent_record_offset":1268,"ancestor_record_offset":512,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3756,"record_offset":16784,"parent_record_offset":16756,"ancestor_record_offset":512,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3756,"record_offset":944,"parent_record_offset":912,"ancestor_record_offset":512,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::contains
//     owner=registry_dependency_definition; count=11
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":3756,"record_offset":5700,"parent_record_offset":5676,"ancestor_record_offset":512,"inlinee":5673,"source_line":621,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::entities
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":3756,"record_offset":3868,"parent_record_offset":1600,"ancestor_record_offset":512,"inlinee":11682,"source_line":466,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::is_empty
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":3756,"record_offset":3172,"parent_record_offset":1600,"ancestor_record_offset":512,"inlinee":11679,"source_line":615,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::table_id
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":3756,"record_offset":3256,"parent_record_offset":1600,"ancestor_record_offset":512,"inlinee":5495,"source_line":460,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::ArchetypeEntity::table_row
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":3756,"record_offset":4148,"parent_record_offset":1600,"ancestor_record_offset":512,"inlinee":17012,"source_line":325,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetypes::get
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":3756,"record_offset":3088,"parent_record_offset":1600,"ancestor_record_offset":512,"inlinee":11678,"source_line":895,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::impl$25::eq
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":3756,"record_offset":14008,"parent_record_offset":13676,"ancestor_record_offset":512,"inlinee":4846,"source_line":76,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=70
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":3756,"record_offset":1512,"parent_record_offset":1224,"ancestor_record_offset":512,"inlinee":4459,"source_line":397,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=32
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":3756,"record_offset":14844,"parent_record_offset":14820,"ancestor_record_offset":512,"inlinee":4457,"source_line":434,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$57::set_changed
//     owner=registry_dependency_definition; count=29
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":3756,"record_offset":4468,"parent_record_offset":4444,"ancestor_record_offset":512,"inlinee":9179,"source_line":397,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$58::deref_mut
//     owner=registry_dependency_definition; count=22
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":3756,"record_offset":15432,"parent_record_offset":15380,"ancestor_record_offset":512,"inlinee":9177,"source_line":434,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::Entities::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/entity/mod.rs
//     evidence={"module":3756,"record_offset":13676,"parent_record_offset":4920,"ancestor_record_offset":512,"inlinee":4840,"source_line":842,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::impl$0::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/entity/mod.rs
//     evidence={"module":3756,"record_offset":4616,"parent_record_offset":4592,"ancestor_record_offset":512,"inlinee":10138,"source_line":199,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::collections::Events::send
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/collections.rs
//     evidence={"module":3756,"record_offset":1224,"parent_record_offset":1192,"ancestor_record_offset":512,"inlinee":9991,"source_line":124,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::collections::Events::send_default
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/collections.rs
//     evidence={"module":3756,"record_offset":27076,"parent_record_offset":26996,"ancestor_record_offset":512,"inlinee":9990,"source_line":164,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::collections::Events::send_with_caller
//     owner=registry_dependency_definition; count=37
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/collections.rs
//     evidence={"module":3756,"record_offset":1268,"parent_record_offset":1224,"ancestor_record_offset":512,"inlinee":7659,"source_line":128,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::StorageSwitch::extract
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3756,"record_offset":2844,"parent_record_offset":2820,"ancestor_record_offset":512,"inlinee":4876,"source_line":2472,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$26::matches_component_set
//     owner=registry_dependency_definition; count=11
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3756,"record_offset":5648,"parent_record_offset":5616,"ancestor_record_offset":512,"inlinee":5720,"source_line":1210,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$26::set_archetype
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3756,"record_offset":5916,"parent_record_offset":5616,"ancestor_record_offset":512,"inlinee":4856,"source_line":1160,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$26::set_table
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3756,"record_offset":5940,"parent_record_offset":5916,"ancestor_record_offset":512,"inlinee":4857,"source_line":1175,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::set_archetype
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3756,"record_offset":3372,"parent_record_offset":3348,"ancestor_record_offset":512,"inlinee":13655,"source_line":1524,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::set_table
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3756,"record_offset":2040,"parent_record_offset":2012,"ancestor_record_offset":512,"inlinee":13656,"source_line":1539,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$37::fetch
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3756,"record_offset":2820,"parent_record_offset":2796,"ancestor_record_offset":512,"inlinee":13686,"source_line":1596,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$37::fetch::closure$0
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3756,"record_offset":2868,"parent_record_offset":2844,"ancestor_record_offset":512,"inlinee":9533,"source_line":1602,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$41::set_archetype
//     owner=registry_dependency_definition; count=11
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3756,"record_offset":5616,"parent_record_offset":4984,"ancestor_record_offset":512,"inlinee":17008,"source_line":1799,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$41::set_archetype::closure$0
//     owner=registry_dependency_definition; count=11
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3756,"record_offset":5676,"parent_record_offset":5648,"ancestor_record_offset":512,"inlinee":17010,"source_line":1805,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$61::fetch
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3756,"record_offset":2796,"parent_record_offset":1600,"ancestor_record_offset":512,"inlinee":17019,"source_line":2083,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$85::fetch
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3756,"record_offset":13448,"parent_record_offset":4920,"ancestor_record_offset":512,"inlinee":121461,"source_line":2083,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIter::new
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs
//     evidence={"module":3756,"record_offset":880,"parent_record_offset":788,"ancestor_record_offset":512,"inlinee":13709,"source_line":39,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::init
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs
//     evidence={"module":3756,"record_offset":912,"parent_record_offset":880,"ancestor_record_offset":512,"inlinee":9292,"source_line":2359,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::next
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs
//     evidence={"module":3756,"record_offset":1600,"parent_record_offset":1576,"ancestor_record_offset":512,"inlinee":11674,"source_line":2458,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::impl$1::next
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs
//     evidence={"module":3756,"record_offset":1576,"parent_record_offset":1552,"ancestor_record_offset":512,"inlinee":11673,"source_line":884,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$14::set_archetype
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/world_query.rs
//     evidence={"module":3756,"record_offset":4984,"parent_record_offset":4920,"ancestor_record_offset":512,"inlinee":121459,"source_line":182,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$2::set_archetype
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/world_query.rs
//     evidence={"module":3756,"record_offset":3348,"parent_record_offset":1600,"ancestor_record_offset":512,"inlinee":13725,"source_line":182,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$2::set_table
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/world_query.rs
//     evidence={"module":3756,"record_offset":2012,"parent_record_offset":1600,"ancestor_record_offset":512,"inlinee":17013,"source_line":195,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_array::BlobArray::get_sub_slice
//     owner=registry_dependency_definition; count=14
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_array.rs
//     evidence={"module":3756,"record_offset":2420,"parent_record_offset":2396,"ancestor_record_offset":512,"inlinee":4872,"source_line":143,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseArray::contains
//     owner=registry_dependency_definition; count=11
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":3756,"record_offset":5748,"parent_record_offset":5724,"ancestor_record_offset":512,"inlinee":5675,"source_line":49,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseArray::get
//     owner=registry_dependency_definition; count=14
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":3756,"record_offset":2128,"parent_record_offset":2100,"ancestor_record_offset":512,"inlinee":4864,"source_line":58,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::contains
//     owner=registry_dependency_definition; count=11
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":3756,"record_offset":5724,"parent_record_offset":5700,"ancestor_record_offset":512,"inlinee":5674,"source_line":388,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
//     owner=registry_dependency_definition; count=15
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":3756,"record_offset":2100,"parent_record_offset":2072,"ancestor_record_offset":512,"inlinee":4862,"source_line":395,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$16::get::closure$0
//     owner=registry_dependency_definition; count=14
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":3756,"record_offset":2264,"parent_record_offset":2240,"ancestor_record_offset":512,"inlinee":4866,"source_line":396,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::entities
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":3756,"record_offset":2516,"parent_record_offset":1600,"ancestor_record_offset":512,"inlinee":11681,"source_line":211,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_column
//     owner=registry_dependency_definition; count=15
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":3756,"record_offset":2072,"parent_record_offset":2040,"ancestor_record_offset":512,"inlinee":4860,"source_line":475,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_data_slice_for
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":3756,"record_offset":5964,"parent_record_offset":5940,"ancestor_record_offset":512,"inlinee":4859,"source_line":363,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::is_empty
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":3756,"record_offset":1928,"parent_record_offset":1600,"ancestor_record_offset":512,"inlinee":11675,"source_line":615,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::TableRow::as_usize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":3756,"record_offset":2892,"parent_record_offset":2868,"ancestor_record_offset":512,"inlinee":5472,"source_line":127,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":3756,"record_offset":1844,"parent_record_offset":1600,"ancestor_record_offset":512,"inlinee":4854,"source_line":721,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::column::ThinColumn::get_changed_ticks_slice
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/column.rs
//     evidence={"module":3756,"record_offset":2452,"parent_record_offset":2040,"ancestor_record_offset":512,"inlinee":13683,"source_line":319,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::column::ThinColumn::get_data_slice
//     owner=registry_dependency_definition; count=14
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/column.rs
//     evidence={"module":3756,"record_offset":2396,"parent_record_offset":2040,"ancestor_record_offset":512,"inlinee":4870,"source_line":303,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::impl$4::get_data_slice_for::closure$0
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":3756,"record_offset":6304,"parent_record_offset":6280,"ancestor_record_offset":512,"inlinee":4868,"source_line":368,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::thin_array_ptr::ThinArrayPtr::as_slice
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/thin_array_ptr.rs
//     evidence={"module":3756,"record_offset":2476,"parent_record_offset":2452,"ancestor_record_offset":512,"inlinee":13684,"source_line":300,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::copy_unsafe
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":3756,"record_offset":848,"parent_record_offset":824,"ancestor_record_offset":512,"inlinee":13707,"source_line":636,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":3756,"record_offset":34080,"parent_record_offset":34056,"ancestor_record_offset":512,"inlinee":4837,"source_line":1385,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::get_inner
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":3756,"record_offset":4920,"parent_record_offset":4884,"ancestor_record_offset":512,"inlinee":4838,"source_line":1591,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::reborrow
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":3756,"record_offset":788,"parent_record_offset":756,"ancestor_record_offset":512,"inlinee":17021,"source_line":601,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::reborrow_unsafe
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":3756,"record_offset":824,"parent_record_offset":788,"ancestor_record_offset":512,"inlinee":13706,"source_line":618,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::impl$4::into_iter
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":3756,"record_offset":45128,"parent_record_offset":45100,"ancestor_record_offset":512,"inlinee":25723,"source_line":2574,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ptr::ThinSlicePtr::get
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bevy_ptr","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ptr-0.16.0/src/lib.rs
//     evidence={"module":3756,"record_offset":4276,"parent_record_offset":4248,"ancestor_record_offset":512,"inlinee":4880,"source_line":526,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] core::bool::impl$0::then
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\bool.rs
//     evidence={"module":3756,"record_offset":14068,"parent_record_offset":14040,"ancestor_record_offset":512,"inlinee":4781,"source_line":61,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$21::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":3756,"record_offset":44632,"parent_record_offset":44604,"ancestor_record_offset":512,"inlinee":4913,"source_line":1740,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$24::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":3756,"record_offset":31272,"parent_record_offset":31244,"ancestor_record_offset":512,"inlinee":5188,"source_line":1740,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$24::ne
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":3756,"record_offset":13976,"parent_record_offset":13952,"ancestor_record_offset":512,"inlinee":4844,"source_line":1742,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":3756,"record_offset":4764,"parent_record_offset":4740,"ancestor_record_offset":512,"inlinee":4690,"source_line":1824,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":3756,"record_offset":4592,"parent_record_offset":4564,"ancestor_record_offset":512,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":3756,"record_offset":17008,"parent_record_offset":16984,"ancestor_record_offset":512,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":3756,"record_offset":4740,"parent_record_offset":4716,"ancestor_record_offset":512,"inlinee":4688,"source_line":763,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::impl$2::ne
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":3756,"record_offset":13952,"parent_record_offset":13676,"ancestor_record_offset":512,"inlinee":4842,"source_line":200,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":3756,"record_offset":30024,"parent_record_offset":30000,"ancestor_record_offset":512,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$241::div
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":3756,"record_offset":15288,"parent_record_offset":15264,"ancestor_record_offset":512,"inlinee":9344,"source_line":502,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":3756,"record_offset":15344,"parent_record_offset":15320,"ancestor_record_offset":512,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=25
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":3756,"record_offset":2176,"parent_record_offset":2152,"ancestor_record_offset":512,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3756,"record_offset":2152,"parent_record_offset":2128,"ancestor_record_offset":512,"inlinee":4518,"source_line":1447,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3756,"record_offset":2200,"parent_record_offset":2176,"ancestor_record_offset":512,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::get_or_insert_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3756,"record_offset":4528,"parent_record_offset":4500,"ancestor_record_offset":512,"inlinee":7077,"source_line":1691,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=22
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3756,"record_offset":5860,"parent_record_offset":5836,"ancestor_record_offset":512,"inlinee":4512,"source_line":607,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=23
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3756,"record_offset":2240,"parent_record_offset":2100,"ancestor_record_offset":512,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3756,"record_offset":15168,"parent_record_offset":15144,"ancestor_record_offset":512,"inlinee":6345,"source_line":1218,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$14::eq
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3756,"record_offset":15612,"parent_record_offset":15580,"ancestor_record_offset":512,"inlinee":5835,"source_line":2186,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$39::branch
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3756,"record_offset":4412,"parent_record_offset":1552,"ancestor_record_offset":512,"inlinee":4927,"source_line":2486,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":3756,"record_offset":16984,"parent_record_offset":16756,"ancestor_record_offset":512,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=32
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":3756,"record_offset":17724,"parent_record_offset":17700,"ancestor_record_offset":512,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3756,"record_offset":1140,"parent_record_offset":1116,"ancestor_record_offset":512,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":3756,"record_offset":1804,"parent_record_offset":1780,"ancestor_record_offset":512,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":3756,"record_offset":1728,"parent_record_offset":1692,"ancestor_record_offset":512,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":3756,"record_offset":27908,"parent_record_offset":27880,"ancestor_record_offset":512,"inlinee":4282,"source_line":1740,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=29
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":3756,"record_offset":1476,"parent_record_offset":1312,"ancestor_record_offset":512,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":3756,"record_offset":14592,"parent_record_offset":4920,"ancestor_record_offset":512,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=28
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":3756,"record_offset":1868,"parent_record_offset":1844,"ancestor_record_offset":512,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get_unchecked
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":3756,"record_offset":2712,"parent_record_offset":1600,"ancestor_record_offset":512,"inlinee":5263,"source_line":656,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_noubcheck
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":3756,"record_offset":2344,"parent_record_offset":2320,"ancestor_record_offset":512,"inlinee":4164,"source_line":87,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":3756,"record_offset":39604,"parent_record_offset":39572,"ancestor_record_offset":512,"inlinee":4360,"source_line":15,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=30
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":3756,"record_offset":1892,"parent_record_offset":1868,"ancestor_record_offset":512,"inlinee":4145,"source_line":226,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":3756,"record_offset":2320,"parent_record_offset":2288,"ancestor_record_offset":512,"inlinee":4162,"source_line":242,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":3756,"record_offset":39636,"parent_record_offset":39604,"ancestor_record_offset":512,"inlinee":4640,"source_line":272,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":3756,"record_offset":1116,"parent_record_offset":944,"ancestor_record_offset":512,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":3756,"record_offset":1780,"parent_record_offset":1756,"ancestor_record_offset":512,"inlinee":4510,"source_line":99,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":3756,"record_offset":1692,"parent_record_offset":1600,"ancestor_record_offset":512,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$186::next_unchecked
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":3756,"record_offset":1756,"parent_record_offset":1692,"ancestor_record_offset":512,"inlinee":4509,"source_line":440,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] core::tuple::impl$10::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\tuple.rs
//     evidence={"module":3756,"record_offset":44604,"parent_record_offset":44568,"ancestor_record_offset":512,"inlinee":4662,"source_line":30,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::color_id::impl$14::eq
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/color_id.rs
//     evidence={"module":3756,"record_offset":31148,"parent_record_offset":31120,"ancestor_record_offset":512,"inlinee":14666,"source_line":26,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::roofs::roof_shape::impl$22::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/roofs/roof_shape.rs
//     evidence={"module":3756,"record_offset":31088,"parent_record_offset":31060,"ancestor_record_offset":512,"inlinee":121467,"source_line":70,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::roofs::roof_shape::impl$28::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/roofs/roof_shape.rs
//     evidence={"module":3756,"record_offset":31212,"parent_record_offset":31184,"ancestor_record_offset":512,"inlinee":121469,"source_line":116,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::roofs::roof_shape::impl$35::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/roofs/roof_shape.rs
//     evidence={"module":3756,"record_offset":38248,"parent_record_offset":38220,"ancestor_record_offset":512,"inlinee":25203,"source_line":123,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator::impl$77::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":3756,"record_offset":32376,"parent_record_offset":32348,"ancestor_record_offset":512,"inlinee":14668,"source_line":323,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::inner_wall_state::impl$16::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_wall_state.rs
//     evidence={"module":3756,"record_offset":37480,"parent_record_offset":37452,"ancestor_record_offset":512,"inlinee":14656,"source_line":27,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::inner_wall_state::impl$23::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_wall_state.rs
//     evidence={"module":3756,"record_offset":35596,"parent_record_offset":35568,"ancestor_record_offset":512,"inlinee":14652,"source_line":49,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::inner_wall_state::impl$32::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_wall_state.rs
//     evidence={"module":3756,"record_offset":37712,"parent_record_offset":37684,"ancestor_record_offset":512,"inlinee":14658,"source_line":143,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::ui::resources::widget_params::impl$46::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/resources/widget_params.rs
//     evidence={"module":3756,"record_offset":44700,"parent_record_offset":44672,"ancestor_record_offset":512,"inlinee":121472,"source_line":205,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::contains
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":3756,"record_offset":14040,"parent_record_offset":4920,"ancestor_record_offset":512,"inlinee":4848,"source_line":392,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::contains_unchecked
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":3756,"record_offset":14116,"parent_record_offset":14092,"ancestor_record_offset":512,"inlinee":4851,"source_line":408,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::get_unchecked
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":3756,"record_offset":14140,"parent_record_offset":14116,"ancestor_record_offset":512,"inlinee":4852,"source_line":171,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::impl$2::contains::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":3756,"record_offset":14092,"parent_record_offset":14068,"ancestor_record_offset":512,"inlinee":4850,"source_line":395,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":3756,"record_offset":30136,"parent_record_offset":30080,"ancestor_record_offset":512,"inlinee":4749,"source_line":166,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=12
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":3756,"record_offset":15320,"parent_record_offset":15228,"ancestor_record_offset":512,"inlinee":9561,"source_line":1418,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$8::div
//     owner=registry_dependency_definition; count=11
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":3756,"record_offset":15264,"parent_record_offset":15228,"ancestor_record_offset":512,"inlinee":19932,"source_line":1063,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=14
//     packages=[{"name":"nonmax","version":"0.5.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":3756,"record_offset":2288,"parent_record_offset":2264,"ancestor_record_offset":512,"inlinee":4171,"source_line":153,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":3756,"record_offset":30168,"parent_record_offset":30136,"ancestor_record_offset":512,"inlinee":4750,"source_line":369,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] system_color::color_menu::color_menu::closure$4
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-color","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/color/src/color_menu.rs
//     evidence={"module":3756,"record_offset":34056,"parent_record_offset":34024,"ancestor_record_offset":512,"inlinee":121470,"source_line":791,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_color::color_menu::color_menu::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=12
// 源码行: 255
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1DA77
// 首条位置: {"module":3756,"inlinee":121463}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=11
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":3756,"record_offset":15320,"parent_record_offset":15228,"ancestor_record_offset":15228,"inlinee":9561,"source_line":1418,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$8::div
//     owner=registry_dependency_definition; count=11
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":3756,"record_offset":15264,"parent_record_offset":15228,"ancestor_record_offset":15228,"inlinee":19932,"source_line":1063,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$241::div
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":3756,"record_offset":15288,"parent_record_offset":15264,"ancestor_record_offset":15228,"inlinee":9344,"source_line":502,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":3756,"record_offset":15344,"parent_record_offset":15320,"ancestor_record_offset":15228,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_color::color_menu::color_menu::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=12
// 源码行: 266
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1DA78
// 首条位置: {"module":3756,"inlinee":121464}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$58::deref_mut
//     owner=registry_dependency_definition; count=22
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":3756,"record_offset":15432,"parent_record_offset":15380,"ancestor_record_offset":15380,"inlinee":9177,"source_line":434,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3756,"record_offset":15488,"parent_record_offset":15380,"ancestor_record_offset":15380,"inlinee":4512,"source_line":607,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$57::set_changed
//     owner=registry_dependency_definition; count=22
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":3756,"record_offset":15456,"parent_record_offset":15432,"ancestor_record_offset":15380,"inlinee":9179,"source_line":397,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_color::color_menu::color_menu::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=12
// 源码行: 292
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1DA79
// 首条位置: {"module":3756,"inlinee":121465}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::impl$14::eq
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3756,"record_offset":15612,"parent_record_offset":15580,"ancestor_record_offset":15580,"inlinee":5835,"source_line":2186,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_color::color_menu::color_menu::closure$4
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 791
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1DA7E
// 首条位置: {"module":3756,"inlinee":121470}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] bevy_ecs::system::query::Query::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":3756,"record_offset":34080,"parent_record_offset":34056,"ancestor_record_offset":34056,"inlinee":4837,"source_line":1385,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3756,"record_offset":34352,"parent_record_offset":34324,"ancestor_record_offset":34056,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3756,"record_offset":34408,"parent_record_offset":34380,"ancestor_record_offset":34056,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3756,"record_offset":34380,"parent_record_offset":34352,"ancestor_record_offset":34056,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3756,"record_offset":34324,"parent_record_offset":34296,"ancestor_record_offset":34056,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3756,"record_offset":34296,"parent_record_offset":34184,"ancestor_record_offset":34056,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3756,"record_offset":34776,"parent_record_offset":34748,"ancestor_record_offset":34056,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::impl$25::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":3756,"record_offset":34520,"parent_record_offset":34152,"ancestor_record_offset":34056,"inlinee":4846,"source_line":76,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::Entities::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/entity/mod.rs
//     evidence={"module":3756,"record_offset":34152,"parent_record_offset":34108,"ancestor_record_offset":34056,"inlinee":4840,"source_line":842,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$26::set_archetype
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3756,"record_offset":35044,"parent_record_offset":34108,"ancestor_record_offset":34056,"inlinee":4856,"source_line":1160,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$26::set_table
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3756,"record_offset":35072,"parent_record_offset":35044,"ancestor_record_offset":34056,"inlinee":4857,"source_line":1175,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_array::BlobArray::get_sub_slice
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_array.rs
//     evidence={"module":3756,"record_offset":35496,"parent_record_offset":35468,"ancestor_record_offset":34056,"inlinee":4872,"source_line":143,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseArray::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":3756,"record_offset":35188,"parent_record_offset":35160,"ancestor_record_offset":34056,"inlinee":4864,"source_line":58,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":3756,"record_offset":35160,"parent_record_offset":35132,"ancestor_record_offset":34056,"inlinee":4862,"source_line":395,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$16::get::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":3756,"record_offset":35340,"parent_record_offset":35312,"ancestor_record_offset":34056,"inlinee":4866,"source_line":396,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_column
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":3756,"record_offset":35132,"parent_record_offset":35100,"ancestor_record_offset":34056,"inlinee":4860,"source_line":475,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_data_slice_for
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":3756,"record_offset":35100,"parent_record_offset":35072,"ancestor_record_offset":34056,"inlinee":4859,"source_line":363,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":3756,"record_offset":34748,"parent_record_offset":34108,"ancestor_record_offset":34056,"inlinee":4854,"source_line":721,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::column::ThinColumn::get_data_slice
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/column.rs
//     evidence={"module":3756,"record_offset":35468,"parent_record_offset":35444,"ancestor_record_offset":34056,"inlinee":4870,"source_line":303,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::impl$4::get_data_slice_for::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":3756,"record_offset":35444,"parent_record_offset":35416,"ancestor_record_offset":34056,"inlinee":4868,"source_line":368,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::get_inner
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":3756,"record_offset":34108,"parent_record_offset":34080,"ancestor_record_offset":34056,"inlinee":4838,"source_line":1591,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] core::bool::impl$0::then
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\bool.rs
//     evidence={"module":3756,"record_offset":34580,"parent_record_offset":34552,"ancestor_record_offset":34056,"inlinee":4781,"source_line":61,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$24::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":3756,"record_offset":34488,"parent_record_offset":34460,"ancestor_record_offset":34056,"inlinee":4844,"source_line":1742,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::impl$2::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":3756,"record_offset":34460,"parent_record_offset":34152,"ancestor_record_offset":34056,"inlinee":4842,"source_line":200,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":3756,"record_offset":35244,"parent_record_offset":35216,"ancestor_record_offset":34056,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3756,"record_offset":35216,"parent_record_offset":35188,"ancestor_record_offset":34056,"inlinee":4518,"source_line":1447,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3756,"record_offset":35268,"parent_record_offset":35244,"ancestor_record_offset":34056,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3756,"record_offset":35312,"parent_record_offset":35160,"ancestor_record_offset":34056,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3756,"record_offset":34696,"parent_record_offset":34660,"ancestor_record_offset":34056,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":3756,"record_offset":34184,"parent_record_offset":34152,"ancestor_record_offset":34056,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_noubcheck
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":3756,"record_offset":34260,"parent_record_offset":34228,"ancestor_record_offset":34056,"inlinee":4164,"source_line":87,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":3756,"record_offset":34228,"parent_record_offset":34184,"ancestor_record_offset":34056,"inlinee":4145,"source_line":226,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::contains
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":3756,"record_offset":34552,"parent_record_offset":34108,"ancestor_record_offset":34056,"inlinee":4848,"source_line":392,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::contains_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":3756,"record_offset":34632,"parent_record_offset":34608,"ancestor_record_offset":34056,"inlinee":4851,"source_line":408,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::get_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":3756,"record_offset":34660,"parent_record_offset":34632,"ancestor_record_offset":34056,"inlinee":4852,"source_line":171,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::impl$2::contains::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":3756,"record_offset":34608,"parent_record_offset":34580,"ancestor_record_offset":34056,"inlinee":4850,"source_line":395,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nonmax","version":"0.5.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":3756,"record_offset":35368,"parent_record_offset":35340,"ancestor_record_offset":34056,"inlinee":4171,"source_line":153,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_color::color_menu::delete_color_icon_entities
// 记录形式: compiled_procedure; instances=1
// 源码行: 1145
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3756,"record_offset":56192,"rva":36274160,"va":5404983280}
// 编译实例 RVAs: 0x2297ff0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] bevy_ecs::system::commands::EntityCommands::despawn
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-22559172","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404983280,"raw_hex":"e83cc6a7fe","size":5,"soft_fail":false,"target_va":5382424432,"va":5404983599,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::query::iter::impl$1::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs
//     evidence={"module":3756,"record_offset":56740,"parent_record_offset":56192,"ancestor_record_offset":56192,"inlinee":11673,"source_line":884,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::commands::Commands::entity
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs
//     evidence={"module":3756,"record_offset":59288,"parent_record_offset":56192,"ancestor_record_offset":56192,"inlinee":13699,"source_line":407,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::query::Query::iter
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":3756,"record_offset":56336,"parent_record_offset":56192,"ancestor_record_offset":56192,"inlinee":13704,"source_line":670,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3756,"record_offset":56572,"parent_record_offset":56548,"ancestor_record_offset":56192,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3756,"record_offset":56620,"parent_record_offset":56596,"ancestor_record_offset":56192,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3756,"record_offset":56596,"parent_record_offset":56572,"ancestor_record_offset":56192,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3756,"record_offset":56548,"parent_record_offset":56524,"ancestor_record_offset":56192,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3756,"record_offset":56524,"parent_record_offset":56492,"ancestor_record_offset":56192,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3756,"record_offset":57108,"parent_record_offset":57084,"ancestor_record_offset":56192,"inlinee":4613,"source_line":2705,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3756,"record_offset":57132,"parent_record_offset":57108,"ancestor_record_offset":56192,"inlinee":4352,"source_line":2680,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3756,"record_offset":56492,"parent_record_offset":56468,"ancestor_record_offset":56192,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::entities
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":3756,"record_offset":58972,"parent_record_offset":56768,"ancestor_record_offset":56192,"inlinee":11682,"source_line":466,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::is_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":3756,"record_offset":58252,"parent_record_offset":56768,"ancestor_record_offset":56192,"inlinee":11679,"source_line":615,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::table_id
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":3756,"record_offset":58336,"parent_record_offset":56768,"ancestor_record_offset":56192,"inlinee":5495,"source_line":460,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::ArchetypeEntity::table_row
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":3756,"record_offset":59252,"parent_record_offset":56768,"ancestor_record_offset":56192,"inlinee":17012,"source_line":325,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetypes::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":3756,"record_offset":58168,"parent_record_offset":56768,"ancestor_record_offset":56192,"inlinee":11678,"source_line":895,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::StorageSwitch::extract
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3756,"record_offset":57924,"parent_record_offset":57900,"ancestor_record_offset":56192,"inlinee":4876,"source_line":2472,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$26::set_archetype
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3756,"record_offset":58452,"parent_record_offset":58428,"ancestor_record_offset":56192,"inlinee":4856,"source_line":1160,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$26::set_table
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3756,"record_offset":57192,"parent_record_offset":57168,"ancestor_record_offset":56192,"inlinee":4857,"source_line":1175,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$27::fetch
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3756,"record_offset":57900,"parent_record_offset":57876,"ancestor_record_offset":56192,"inlinee":4874,"source_line":1229,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$27::fetch::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3756,"record_offset":57948,"parent_record_offset":57924,"ancestor_record_offset":56192,"inlinee":4878,"source_line":1235,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$63::fetch
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3756,"record_offset":57876,"parent_record_offset":56768,"ancestor_record_offset":56192,"inlinee":13690,"source_line":2083,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIter::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs
//     evidence={"module":3756,"record_offset":56444,"parent_record_offset":56360,"ancestor_record_offset":56192,"inlinee":13709,"source_line":39,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::init
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs
//     evidence={"module":3756,"record_offset":56468,"parent_record_offset":56444,"ancestor_record_offset":56192,"inlinee":9292,"source_line":2359,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs
//     evidence={"module":3756,"record_offset":56768,"parent_record_offset":56740,"ancestor_record_offset":56192,"inlinee":11674,"source_line":2458,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$3::set_archetype
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/world_query.rs
//     evidence={"module":3756,"record_offset":58428,"parent_record_offset":56768,"ancestor_record_offset":56192,"inlinee":11680,"source_line":182,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$3::set_table
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/world_query.rs
//     evidence={"module":3756,"record_offset":57168,"parent_record_offset":56768,"ancestor_record_offset":56192,"inlinee":11676,"source_line":195,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_array::BlobArray::get_sub_slice
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_array.rs
//     evidence={"module":3756,"record_offset":57628,"parent_record_offset":57604,"ancestor_record_offset":56192,"inlinee":4872,"source_line":143,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseArray::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":3756,"record_offset":57308,"parent_record_offset":57280,"ancestor_record_offset":56192,"inlinee":4864,"source_line":58,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":3756,"record_offset":57280,"parent_record_offset":57252,"ancestor_record_offset":56192,"inlinee":4862,"source_line":395,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$16::get::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":3756,"record_offset":57488,"parent_record_offset":57464,"ancestor_record_offset":56192,"inlinee":4866,"source_line":396,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::entities
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":3756,"record_offset":57680,"parent_record_offset":56768,"ancestor_record_offset":56192,"inlinee":11681,"source_line":211,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_column
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":3756,"record_offset":57252,"parent_record_offset":57220,"ancestor_record_offset":56192,"inlinee":4860,"source_line":475,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_data_slice_for
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":3756,"record_offset":57220,"parent_record_offset":57192,"ancestor_record_offset":56192,"inlinee":4859,"source_line":363,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::is_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":3756,"record_offset":57084,"parent_record_offset":56768,"ancestor_record_offset":56192,"inlinee":11675,"source_line":615,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::TableRow::as_usize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":3756,"record_offset":57972,"parent_record_offset":57948,"ancestor_record_offset":56192,"inlinee":5472,"source_line":127,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":3756,"record_offset":57000,"parent_record_offset":56768,"ancestor_record_offset":56192,"inlinee":4854,"source_line":721,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::column::ThinColumn::get_data_slice
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/column.rs
//     evidence={"module":3756,"record_offset":57604,"parent_record_offset":57580,"ancestor_record_offset":56192,"inlinee":4870,"source_line":303,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::impl$4::get_data_slice_for::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":3756,"record_offset":57580,"parent_record_offset":57556,"ancestor_record_offset":56192,"inlinee":4868,"source_line":368,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::as_readonly
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":3756,"record_offset":56360,"parent_record_offset":56336,"ancestor_record_offset":56192,"inlinee":13705,"source_line":542,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::copy_unsafe
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":3756,"record_offset":56412,"parent_record_offset":56388,"ancestor_record_offset":56192,"inlinee":13707,"source_line":636,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::reborrow_unsafe
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":3756,"record_offset":56388,"parent_record_offset":56360,"ancestor_record_offset":56192,"inlinee":13706,"source_line":618,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":3756,"record_offset":57400,"parent_record_offset":57336,"ancestor_record_offset":56192,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3756,"record_offset":57424,"parent_record_offset":57400,"ancestor_record_offset":56192,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3756,"record_offset":57464,"parent_record_offset":57280,"ancestor_record_offset":56192,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3756,"record_offset":56688,"parent_record_offset":56664,"ancestor_record_offset":56192,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":3756,"record_offset":56960,"parent_record_offset":56936,"ancestor_record_offset":56192,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":3756,"record_offset":56884,"parent_record_offset":56848,"ancestor_record_offset":56192,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":3756,"record_offset":57024,"parent_record_offset":57000,"ancestor_record_offset":56192,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":3756,"record_offset":59168,"parent_record_offset":56768,"ancestor_record_offset":56192,"inlinee":5263,"source_line":656,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_noubcheck
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":3756,"record_offset":59216,"parent_record_offset":59192,"ancestor_record_offset":56192,"inlinee":4164,"source_line":87,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":3756,"record_offset":57048,"parent_record_offset":57024,"ancestor_record_offset":56192,"inlinee":4145,"source_line":226,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":3756,"record_offset":59192,"parent_record_offset":59168,"ancestor_record_offset":56192,"inlinee":4162,"source_line":242,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":3756,"record_offset":56664,"parent_record_offset":56492,"ancestor_record_offset":56192,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":3756,"record_offset":56936,"parent_record_offset":56912,"ancestor_record_offset":56192,"inlinee":4510,"source_line":99,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":3756,"record_offset":56848,"parent_record_offset":56768,"ancestor_record_offset":56192,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$186::next_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":3756,"record_offset":56912,"parent_record_offset":56848,"ancestor_record_offset":56192,"inlinee":4509,"source_line":440,"archive":"_pdb/B/modules/03756.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"nonmax","version":"0.5.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":3756,"record_offset":57512,"parent_record_offset":57488,"ancestor_record_offset":56192,"inlinee":4171,"source_line":153,"archive":"_pdb/B/modules/03756.jsonl.gz"}
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
// 完整结构及原始类型记录: color_menu.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: color_menu.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: color_menu.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 3; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::color_icon::CustomizationIcon>,enum2$<core::option::Option<ref$<country_core::resources::color_icon::ColorIcon> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::WallStyleIcon> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::FlagConnectionIcon> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::FreehandHeightTypeToggleIcon> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::FlatRoofRimTypeToggle> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::ColorMenuBrightnessIcon> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::ChimneySmokeToggle> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::FlatRoofStyleIcon> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::PlasterPeelIcon> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::StairRailingIcon> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::MagicColorIcon> > >,ref$<country_core::components::Ui2dExtent>,ref_mut$<country_core::components::Ui2dImage> >,tuple$<> >,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<system_color::radial_menu::ColorRadialMenu> >,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::resources::glade_color_customization::GladeColorCustomization>,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::widget_params::UiNavState>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<tuple$<usize,usize> > > >,tuple$<bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::resources::color_id::WallColorIds>,bevy_ecs::change_detection::Res<country_core::resources::color_id::HalftimberColorIds>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState> >,tuple$<bevy_ecs::event::writer::EventWriter<country_core::resources::color_id::SetWallColorIdCmd>,bevy_ecs::event::writer::EventWriter<system_color::set_roof_customization::SetRoofColorIdCmd>,bevy_ecs::event::writer::EventWriter<system_color::set_decorator_color_id::SetDecoratorColorIdCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::color_id::SetGateColorIdCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::color_id::SetHalftimberColorIdCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::walls::decorator::SetFlagConnectionCmd>,bevy_ecs::event::writer::EventWriter<system_color::set_decorator_color_id::SetDecoratorBrightnessCmd>,bevy_ecs::event::writer::EventWriter<country_core::systems::wall::set_wall_style::SetFreehandHeightTypeCmd>,bevy_ecs::event::writer::EventWriter<system_color::set_window_light::SetWindowLightCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::SetFlatRoofRimStyleCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::glade_border::NotInDemoHint>,bevy_ecs::event::writer::EventWriter<country_core::systems::wall::set_wall_style::SetWallStyleCmd>,bevy_ecs::event::writer::EventWriter<system_color::show_color_menu::HideColorMenuCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::events::LightLanternCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::events::LightWindowCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::history::FinishInputRecordingCmd> >,bevy_ecs::event::writer::EventWriter<country_core::resources::walls::decorator::SetChimneySmokeCmd>,bevy_ecs::event::writer::EventWriter<enum2$<country_core::systems::ui::resources::widget_params::PlayUiSound> >,bevy_ecs::event::writer::EventWriter<system_color::set_roof_customization::SetFlatRoofStyleCmd>,bevy_ecs::event::writer::EventWriter<country_core::systems::wall::set_wall_style::SetPlasterPeelAmountCmd>,bevy_ecs::event::writer::EventWriter<system_color::set_decorator_color_id::SetLadderColorIdCmd>,bevy_ecs::event::writer::EventWriter<system_color::set_decorator_color_id::SetStairRailingCmd>,bevy_ecs::event::writer::EventWriter<system_color::set_decorator_color_id::SetMagicColorIdCmd>)
// 候选参数类型: ["bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::color_icon::CustomizationIcon>,enum2$<core::option::Option<ref$<country_core::resources::color_icon::ColorIcon> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::WallStyleIcon> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::FlagConnectionIcon> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::FreehandHeightTypeToggleIcon> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::FlatRoofRimTypeToggle> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::ColorMenuBrightnessIcon> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::ChimneySmokeToggle> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::FlatRoofStyleIcon> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::PlasterPeelIcon> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::StairRailingIcon> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::MagicColorIcon> > >,ref$<country_core::components::Ui2dExtent>,ref_mut$<country_core::components::Ui2dImage> >,tuple$<> >", "bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<system_color::radial_menu::ColorRadialMenu> >,tuple$<> >", "bevy_ecs::change_detection::ResMut<country_core::resources::glade_color_customization::GladeColorCustomization>", "bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::widget_params::UiNavState>", "bevy_ecs::system::system_param::Local<enum2$<core::option::Option<tuple$<usize,usize> > > >", "tuple$<bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::resources::color_id::WallColorIds>,bevy_ecs::change_detection::Res<country_core::resources::color_id::HalftimberColorIds>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates> >", "tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState> >", "tuple$<bevy_ecs::event::writer::EventWriter<country_core::resources::color_id::SetWallColorIdCmd>,bevy_ecs::event::writer::EventWriter<system_color::set_roof_customization::SetRoofColorIdCmd>,bevy_ecs::event::writer::EventWriter<system_color::set_decorator_color_id::SetDecoratorColorIdCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::color_id::SetGateColorIdCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::color_id::SetHalftimberColorIdCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::walls::decorator::SetFlagConnectionCmd>,bevy_ecs::event::writer::EventWriter<system_color::set_decorator_color_id::SetDecoratorBrightnessCmd>,bevy_ecs::event::writer::EventWriter<country_core::systems::wall::set_wall_style::SetFreehandHeightTypeCmd>,bevy_ecs::event::writer::EventWriter<system_color::set_window_light::SetWindowLightCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::SetFlatRoofRimStyleCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::glade_border::NotInDemoHint>,bevy_ecs::event::writer::EventWriter<country_core::systems::wall::set_wall_style::SetWallStyleCmd>,bevy_ecs::event::writer::EventWriter<system_color::show_color_menu::HideColorMenuCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::events::LightLanternCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::events::LightWindowCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::history::FinishInputRecordingCmd> >", "bevy_ecs::event::writer::EventWriter<country_core::resources::walls::decorator::SetChimneySmokeCmd>", "bevy_ecs::event::writer::EventWriter<enum2$<country_core::systems::ui::resources::widget_params::PlayUiSound> >", "bevy_ecs::event::writer::EventWriter<system_color::set_roof_customization::SetFlatRoofStyleCmd>", "bevy_ecs::event::writer::EventWriter<country_core::systems::wall::set_wall_style::SetPlasterPeelAmountCmd>", "bevy_ecs::event::writer::EventWriter<system_color::set_decorator_color_id::SetLadderColorIdCmd>", "bevy_ecs::event::writer::EventWriter<system_color::set_decorator_color_id::SetStairRailingCmd>", "bevy_ecs::event::writer::EventWriter<system_color::set_decorator_color_id::SetMagicColorIdCmd>"]
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::color_icon::CustomizationIcon>,enum2$<core::option::Option<ref$<country_core::resources::color_icon::ColorIcon> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::WallStyleIcon> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::FlagConnectionIcon> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::FreehandHeightTypeToggleIcon> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::FlatRoofRimTypeToggle> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::ColorMenuBrightnessIcon> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::ChimneySmokeToggle> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::FlatRoofStyleIcon> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::PlasterPeelIcon> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::StairRailingIcon> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::MagicColorIcon> > >,ref$<country_core::components::Ui2dExtent>,ref_mut$<country_core::components::Ui2dImage> >,tuple$<> >
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<system_color::radial_menu::ColorRadialMenu> >,tuple$<> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::glade_color_customization::GladeColorCustomization>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::widget_params::UiNavState>
// [候选系统接口] system_local: bevy_ecs::system::system_param::Local<enum2$<core::option::Option<tuple$<usize,usize> > > >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::color_id::WallColorIds>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::color_id::HalftimberColorIds>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::history::History>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::resources::color_id::SetWallColorIdCmd>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<system_color::set_roof_customization::SetRoofColorIdCmd>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<system_color::set_decorator_color_id::SetDecoratorColorIdCmd>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::resources::color_id::SetGateColorIdCmd>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::resources::color_id::SetHalftimberColorIdCmd>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::resources::walls::decorator::SetFlagConnectionCmd>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<system_color::set_decorator_color_id::SetDecoratorBrightnessCmd>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::systems::wall::set_wall_style::SetFreehandHeightTypeCmd>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<system_color::set_window_light::SetWindowLightCmd>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::SetFlatRoofRimStyleCmd>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::resources::glade_border::NotInDemoHint>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::systems::wall::set_wall_style::SetWallStyleCmd>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<system_color::show_color_menu::HideColorMenuCmd>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::resources::events::LightLanternCmd>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::resources::events::LightWindowCmd>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::resources::history::FinishInputRecordingCmd>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::resources::walls::decorator::SetChimneySmokeCmd>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<enum2$<country_core::systems::ui::resources::widget_params::PlayUiSound> >
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<system_color::set_roof_customization::SetFlatRoofStyleCmd>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::systems::wall::set_wall_style::SetPlasterPeelAmountCmd>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<system_color::set_decorator_color_id::SetLadderColorIdCmd>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<system_color::set_decorator_color_id::SetStairRailingCmd>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<system_color::set_decorator_color_id::SetMagicColorIdCmd>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::color_icon::CustomizationIcon>,ref_mut$<country_core::components::Ui2dExtent>,ref_mut$<country_core::components::UiDrawOrder>,enum2$<core::option::Option<ref$<country_core::resources::color_icon::CustomizationIconMarkedForDeletion> > > >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)
// 候选参数类型: ["bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::color_icon::CustomizationIcon>,ref_mut$<country_core::components::Ui2dExtent>,ref_mut$<country_core::components::UiDrawOrder>,enum2$<core::option::Option<ref$<country_core::resources::color_icon::CustomizationIconMarkedForDeletion> > > >,tuple$<> >", "bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>"]
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::color_icon::CustomizationIcon>,ref_mut$<country_core::components::Ui2dExtent>,ref_mut$<country_core::components::UiDrawOrder>,enum2$<core::option::Option<ref$<country_core::resources::color_icon::CustomizationIconMarkedForDeletion> > > >,tuple$<> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::color_icon::CustomizationIcon>,ref$<country_core::resources::color_icon::CustomizationIconMarkedForDeletion> >,tuple$<> >,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::color_icon::CustomizationIcon>,ref$<country_core::resources::color_icon::CustomizationIconMarkedForDeletion> >,tuple$<> >", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::color_icon::CustomizationIcon>,ref$<country_core::resources::color_icon::CustomizationIconMarkedForDeletion> >,tuple$<> >
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands

// ------------------------------------------------------------------
// 函数: system_color::color_menu::color_button_animations
// 信息 ID: 903a6457af3e5500ad7eefb8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 700d28d21d1bc8ddd520f542: void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::color_icon::CustomizationIcon>,ref_mut$<country_core::components::Ui2dExtent>,ref_mut$<country_core::components::UiDrawOrder>,enum2$<core::option::Option<ref$<country_core::resources::color_icon::CustomizationIconMarkedForDeletion> > > >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::color_icon::CustomizationIcon>,ref_mut$<country_core::components::Ui2dExtent>,ref_mut$<country_core::components::UiDrawOrder>,enum2$<core::option::Option<ref$<country_core::resources::color_icon::CustomizationIconMarkedForDeletion> > > >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::color_icon::CustomizationIcon>,ref_mut$<country_core::components::Ui2dExtent>,ref_mut$<country_core::components::UiDrawOrder>,enum2$<core::option::Option<ref$<country_core::resources::color_icon::CustomizationIconMarkedForDeletion> > > >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)> > > @ 0x1c8c675 -> 0x22979b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::schedule_system::impl$1::run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::color_icon::CustomizationIcon>,ref_mut$<country_core::components::Ui2dExtent>,ref_mut$<country_core::components::UiDrawOrder>,enum2$<core::option::Option<ref$<country_core::resources::color_icon::CustomizationIconMarkedForDeletion> > > >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::color_icon::CustomizationIcon>,ref_mut$<country_core::components::Ui2dExtent>,ref_mut$<country_core::components::UiDrawOrder>,enum2$<core::option::Option<ref$<country_core::resources::color_icon::CustomizationIconMarkedForDeletion> > > >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)> > @ 0x1ca3c1c -> 0x22979b0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_color::color_menu::color_menu
// 信息 ID: 9953a05c671c18827d05bacf
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 20c7a9ee7541568263f003a2: void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::color_icon::CustomizationIcon>,enum2$<core::option::Option<ref$<country_core::resources::color_icon::ColorIcon> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::WallStyleIcon> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::FlagConnectionIcon> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::FreehandHeightTypeToggleIcon> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::FlatRoofRimTypeToggle> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::ColorMenuBrightnessIcon> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::ChimneySmokeToggle> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::FlatRoofStyleIcon> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::PlasterPeelIcon> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::StairRailingIcon> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::MagicColorIcon> > >,ref$<country_core::components::Ui2dExtent>,ref_mut$<country_core::components::Ui2dImage> >,tuple$<> >,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<system_color::radial_menu::ColorRadialMenu> >,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::resources::glade_color_customization::GladeColorCustomization>,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::widget_params::UiNavState>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<tuple$<usize,usize> > > >,tuple$<bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::resources::color_id::WallColorIds>,bevy_ecs::change_detection::Res<country_core::resources::color_id::HalftimberColorIds>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState> >,tuple$<bevy_ecs::event::writer::EventWriter<country_core::resources::color_id::SetWallColorIdCmd>,bevy_ecs::event::writer::EventWriter<system_color::set_roof_customization::SetRoofColorIdCmd>,bevy_ecs::event::writer::EventWriter<system_color::set_decorator_color_id::SetDecoratorColorIdCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::color_id::SetGateColorIdCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::color_id::SetHalftimberColorIdCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::walls::decorator::SetFlagConnectionCmd>,bevy_ecs::event::writer::EventWriter<system_color::set_decorator_color_id::SetDecoratorBrightnessCmd>,bevy_ecs::event::writer::EventWriter<country_core::systems::wall::set_wall_style::SetFreehandHeightTypeCmd>,bevy_ecs::event::writer::EventWriter<system_color::set_window_light::SetWindowLightCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::SetFlatRoofRimStyleCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::glade_border::NotInDemoHint>,bevy_ecs::event::writer::EventWriter<country_core::systems::wall::set_wall_style::SetWallStyleCmd>,bevy_ecs::event::writer::EventWriter<system_color::show_color_menu::HideColorMenuCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::events::LightLanternCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::events::LightWindowCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::history::FinishInputRecordingCmd> >,bevy_ecs::event::writer::EventWriter<country_core::resources::walls::decorator::SetChimneySmokeCmd>,bevy_ecs::event::writer::EventWriter<enum2$<country_core::systems::ui::resources::widget_params::PlayUiSound> >,bevy_ecs::event::writer::EventWriter<system_color::set_roof_customization::SetFlatRoofStyleCmd>,bevy_ecs::event::writer::EventWriter<country_core::systems::wall::set_wall_style::SetPlasterPeelAmountCmd>,bevy_ecs::event::writer::EventWriter<system_color::set_decorator_color_id::SetLadderColorIdCmd>,bevy_ecs::event::writer::EventWriter<system_color::set_decorator_color_id::SetStairRailingCmd>,bevy_ecs::event::writer::EventWriter<system_color::set_decorator_color_id::SetMagicColorIdCmd>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::color_icon::CustomizationIcon>,enum2$<core::option::Option<ref$<country_core::resources::color_icon::ColorIcon> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::WallStyleIcon> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::FlagConnectionIcon> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::FreehandHeightTypeToggleIcon> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::FlatRoofRimTypeToggle> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::ColorMenuBrightnessIcon> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::ChimneySmokeToggle> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::FlatRoofStyleIcon> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::PlasterPeelIcon> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::StairRailingIcon> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::MagicColorIcon> > >,ref$<country_core::components::Ui2dExtent>,ref_mut$<country_core::components::Ui2dImage> >,tuple$<> >,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<system_color::radial_menu::ColorRadialMenu> >,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::resources::glade_color_customization::GladeColorCustomization>,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::widget_params::UiNavState>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<tuple$<usize,usize> > > >,tuple$<bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::resources::color_id::WallColorIds>,bevy_ecs::change_detection::Res<country_core::resources::color_id::HalftimberColorIds>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState> >,tuple$<bevy_ecs::event::writer::EventWriter<country_core::resources::color_id::SetWallColorIdCmd>,bevy_ecs::event::writer::EventWriter<system_color::set_roof_customization::SetRoofColorIdCmd>,bevy_ecs::event::writer::EventWriter<system_color::set_decorator_color_id::SetDecoratorColorIdCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::color_id::SetGateColorIdCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::color_id::SetHalftimberColorIdCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::walls::decorator::SetFlagConnectionCmd>,bevy_ecs::event::writer::EventWriter<system_color::set_decorator_color_id::SetDecoratorBrightnessCmd>,bevy_ecs::event::writer::EventWriter<country_core::systems::wall::set_wall_style::SetFreehandHeightTypeCmd>,bevy_ecs::event::writer::EventWriter<system_color::set_window_light::SetWindowLightCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::SetFlatRoofRimStyleCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::glade_border::NotInDemoHint>,bevy_ecs::event::writer::EventWriter<country_core::systems::wall::set_wall_style::SetWallStyleCmd>,bevy_ecs::event::writer::EventWriter<system_color::show_color_menu::HideColorMenuCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::events::LightLanternCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::events::LightWindowCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::history::FinishInputRecordingCmd> >,bevy_ecs::event::writer::EventWriter<country_core::resources::walls::decorator::SetChimneySmokeCmd>,bevy_ecs::event::writer::EventWriter<enum2$<country_core::systems::ui::resources::widget_params::PlayUiSound> >,bevy_ecs::event::writer::EventWriter<system_color::set_roof_customization::SetFlatRoofStyleCmd>,bevy_ecs::event::writer::EventWriter<country_core::systems::wall::set_wall_style::SetPlasterPeelAmountCmd>,bevy_ecs::event::writer::EventWriter<system_color::set_decorator_color_id::SetLadderColorIdCmd>,bevy_ecs::event::writer::EventWriter<system_color::set_decorator_color_id::SetStairRailingCmd>,bevy_ecs::event::writer::EventWriter<system_color::set_decorator_color_id::SetMagicColorIdCmd>),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::color_icon::CustomizationIcon>,enum2$<core::option::Option<ref$<country_core::resources::color_icon::ColorIcon> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::WallStyleIcon> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::FlagConnectionIcon> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::FreehandHeightTypeToggleIcon> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::FlatRoofRimTypeToggle> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::ColorMenuBrightnessIcon> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::ChimneySmokeToggle> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::FlatRoofStyleIcon> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::PlasterPeelIcon> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::StairRailingIcon> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::MagicColorIcon> > >,ref$<country_core::components::Ui2dExtent>,ref_mut$<country_core::components::Ui2dImage> >,tuple$<> >,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<system_color::radial_menu::ColorRadialMenu> >,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::resources::glade_color_customization::GladeColorCustomization>,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::widget_params::UiNavState>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<tuple$<usize,usize> > > >,tuple$<bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::resources::color_id::WallColorIds>,bevy_ecs::change_detection::Res<country_core::resources::color_id::HalftimberColorIds>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState> >,tuple$<bevy_ecs::event::writer::EventWriter<country_core::resources::color_id::SetWallColorIdCmd>,bevy_ecs::event::writer::EventWriter<system_color::set_roof_customization::SetRoofColorIdCmd>,bevy_ecs::event::writer::EventWriter<system_color::set_decorator_color_id::SetDecoratorColorIdCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::color_id::SetGateColorIdCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::color_id::SetHalftimberColorIdCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::walls::decorator::SetFlagConnectionCmd>,bevy_ecs::event::writer::EventWriter<system_color::set_decorator_color_id::SetDecoratorBrightnessCmd>,bevy_ecs::event::writer::EventWriter<country_core::systems::wall::set_wall_style::SetFreehandHeightTypeCmd>,bevy_ecs::event::writer::EventWriter<system_color::set_window_light::SetWindowLightCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::SetFlatRoofRimStyleCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::glade_border::NotInDemoHint>,bevy_ecs::event::writer::EventWriter<country_core::systems::wall::set_wall_style::SetWallStyleCmd>,bevy_ecs::event::writer::EventWriter<system_color::show_color_menu::HideColorMenuCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::events::LightLanternCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::events::LightWindowCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::history::FinishInputRecordingCmd> >,bevy_ecs::event::writer::EventWriter<country_core::resources::walls::decorator::SetChimneySmokeCmd>,bevy_ecs::event::writer::EventWriter<enum2$<country_core::systems::ui::resources::widget_params::PlayUiSound> >,bevy_ecs::event::writer::EventWriter<system_color::set_roof_customization::SetFlatRoofStyleCmd>,bevy_ecs::event::writer::EventWriter<country_core::systems::wall::set_wall_style::SetPlasterPeelAmountCmd>,bevy_ecs::event::writer::EventWriter<system_color::set_decorator_color_id::SetLadderColorIdCmd>,bevy_ecs::event::writer::EventWriter<system_color::set_decorator_color_id::SetStairRailingCmd>,bevy_ecs::event::writer::EventWriter<system_color::set_decorator_color_id::SetMagicColorIdCmd>)> > > @ 0x1c8f29b -> 0x2291230 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::schedule_system::impl$1::run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::color_icon::CustomizationIcon>,enum2$<core::option::Option<ref$<country_core::resources::color_icon::ColorIcon> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::WallStyleIcon> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::FlagConnectionIcon> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::FreehandHeightTypeToggleIcon> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::FlatRoofRimTypeToggle> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::ColorMenuBrightnessIcon> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::ChimneySmokeToggle> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::FlatRoofStyleIcon> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::PlasterPeelIcon> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::StairRailingIcon> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::MagicColorIcon> > >,ref$<country_core::components::Ui2dExtent>,ref_mut$<country_core::components::Ui2dImage> >,tuple$<> >,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<system_color::radial_menu::ColorRadialMenu> >,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::resources::glade_color_customization::GladeColorCustomization>,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::widget_params::UiNavState>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<tuple$<usize,usize> > > >,tuple$<bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::resources::color_id::WallColorIds>,bevy_ecs::change_detection::Res<country_core::resources::color_id::HalftimberColorIds>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState> >,tuple$<bevy_ecs::event::writer::EventWriter<country_core::resources::color_id::SetWallColorIdCmd>,bevy_ecs::event::writer::EventWriter<system_color::set_roof_customization::SetRoofColorIdCmd>,bevy_ecs::event::writer::EventWriter<system_color::set_decorator_color_id::SetDecoratorColorIdCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::color_id::SetGateColorIdCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::color_id::SetHalftimberColorIdCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::walls::decorator::SetFlagConnectionCmd>,bevy_ecs::event::writer::EventWriter<system_color::set_decorator_color_id::SetDecoratorBrightnessCmd>,bevy_ecs::event::writer::EventWriter<country_core::systems::wall::set_wall_style::SetFreehandHeightTypeCmd>,bevy_ecs::event::writer::EventWriter<system_color::set_window_light::SetWindowLightCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::SetFlatRoofRimStyleCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::glade_border::NotInDemoHint>,bevy_ecs::event::writer::EventWriter<country_core::systems::wall::set_wall_style::SetWallStyleCmd>,bevy_ecs::event::writer::EventWriter<system_color::show_color_menu::HideColorMenuCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::events::LightLanternCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::events::LightWindowCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::history::FinishInputRecordingCmd> >,bevy_ecs::event::writer::EventWriter<country_core::resources::walls::decorator::SetChimneySmokeCmd>,bevy_ecs::event::writer::EventWriter<enum2$<country_core::systems::ui::resources::widget_params::PlayUiSound> >,bevy_ecs::event::writer::EventWriter<system_color::set_roof_customization::SetFlatRoofStyleCmd>,bevy_ecs::event::writer::EventWriter<country_core::systems::wall::set_wall_style::SetPlasterPeelAmountCmd>,bevy_ecs::event::writer::EventWriter<system_color::set_decorator_color_id::SetLadderColorIdCmd>,bevy_ecs::event::writer::EventWriter<system_color::set_decorator_color_id::SetStairRailingCmd>,bevy_ecs::event::writer::EventWriter<system_color::set_decorator_color_id::SetMagicColorIdCmd>),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::color_icon::CustomizationIcon>,enum2$<core::option::Option<ref$<country_core::resources::color_icon::ColorIcon> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::WallStyleIcon> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::FlagConnectionIcon> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::FreehandHeightTypeToggleIcon> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::FlatRoofRimTypeToggle> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::ColorMenuBrightnessIcon> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::ChimneySmokeToggle> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::FlatRoofStyleIcon> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::PlasterPeelIcon> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::StairRailingIcon> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::MagicColorIcon> > >,ref$<country_core::components::Ui2dExtent>,ref_mut$<country_core::components::Ui2dImage> >,tuple$<> >,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<system_color::radial_menu::ColorRadialMenu> >,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::resources::glade_color_customization::GladeColorCustomization>,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::widget_params::UiNavState>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<tuple$<usize,usize> > > >,tuple$<bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::resources::color_id::WallColorIds>,bevy_ecs::change_detection::Res<country_core::resources::color_id::HalftimberColorIds>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState> >,tuple$<bevy_ecs::event::writer::EventWriter<country_core::resources::color_id::SetWallColorIdCmd>,bevy_ecs::event::writer::EventWriter<system_color::set_roof_customization::SetRoofColorIdCmd>,bevy_ecs::event::writer::EventWriter<system_color::set_decorator_color_id::SetDecoratorColorIdCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::color_id::SetGateColorIdCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::color_id::SetHalftimberColorIdCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::walls::decorator::SetFlagConnectionCmd>,bevy_ecs::event::writer::EventWriter<system_color::set_decorator_color_id::SetDecoratorBrightnessCmd>,bevy_ecs::event::writer::EventWriter<country_core::systems::wall::set_wall_style::SetFreehandHeightTypeCmd>,bevy_ecs::event::writer::EventWriter<system_color::set_window_light::SetWindowLightCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::SetFlatRoofRimStyleCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::glade_border::NotInDemoHint>,bevy_ecs::event::writer::EventWriter<country_core::systems::wall::set_wall_style::SetWallStyleCmd>,bevy_ecs::event::writer::EventWriter<system_color::show_color_menu::HideColorMenuCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::events::LightLanternCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::events::LightWindowCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::history::FinishInputRecordingCmd> >,bevy_ecs::event::writer::EventWriter<country_core::resources::walls::decorator::SetChimneySmokeCmd>,bevy_ecs::event::writer::EventWriter<enum2$<country_core::systems::ui::resources::widget_params::PlayUiSound> >,bevy_ecs::event::writer::EventWriter<system_color::set_roof_customization::SetFlatRoofStyleCmd>,bevy_ecs::event::writer::EventWriter<country_core::systems::wall::set_wall_style::SetPlasterPeelAmountCmd>,bevy_ecs::event::writer::EventWriter<system_color::set_decorator_color_id::SetLadderColorIdCmd>,bevy_ecs::event::writer::EventWriter<system_color::set_decorator_color_id::SetStairRailingCmd>,bevy_ecs::event::writer::EventWriter<system_color::set_decorator_color_id::SetMagicColorIdCmd>)> > @ 0x1ca5300 -> 0x2291230 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_color::color_menu::color_menu::closure$0
// 信息 ID: 7bfd984855376d6bd9b59975
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_color::color_menu::color_menu", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 11 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_color::color_menu::color_menu; module=3756, inline_record=15228; 不是独立调用边。
// [PDB 内联宿主] system_color::color_menu::color_menu; module=3756, inline_record=15728; 不是独立调用边。
// [PDB 内联宿主] system_color::color_menu::color_menu; module=3756, inline_record=16168; 不是独立调用边。
// [PDB 内联宿主] system_color::color_menu::color_menu; module=3756, inline_record=18012; 不是独立调用边。
// [PDB 内联宿主] system_color::color_menu::color_menu; module=3756, inline_record=18512; 不是独立调用边。
// [PDB 内联宿主] system_color::color_menu::color_menu; module=3756, inline_record=19012; 不是独立调用边。
// [PDB 内联宿主] system_color::color_menu::color_menu; module=3756, inline_record=20872; 不是独立调用边。
// [PDB 内联宿主] system_color::color_menu::color_menu; module=3756, inline_record=22624; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_color::color_menu::color_menu::closure$1
// 信息 ID: faa9b3008188f07fb1af7c74
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "system_color::color_menu::color_menu", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 11 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_color::color_menu::color_menu; module=3756, inline_record=15380; 不是独立调用边。
// [PDB 内联宿主] system_color::color_menu::color_menu; module=3756, inline_record=15880; 不是独立调用边。
// [PDB 内联宿主] system_color::color_menu::color_menu; module=3756, inline_record=16320; 不是独立调用边。
// [PDB 内联宿主] system_color::color_menu::color_menu; module=3756, inline_record=18164; 不是独立调用边。
// [PDB 内联宿主] system_color::color_menu::color_menu; module=3756, inline_record=18664; 不是独立调用边。
// [PDB 内联宿主] system_color::color_menu::color_menu; module=3756, inline_record=19164; 不是独立调用边。
// [PDB 内联宿主] system_color::color_menu::color_menu; module=3756, inline_record=21024; 不是独立调用边。
// [PDB 内联宿主] system_color::color_menu::color_menu; module=3756, inline_record=22776; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_color::color_menu::color_menu::closure$2
// 信息 ID: 56edc696a8229be6f07b8aba
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "system_color::color_menu::color_menu", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 11 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_color::color_menu::color_menu; module=3756, inline_record=15580; 不是独立调用边。
// [PDB 内联宿主] system_color::color_menu::color_menu; module=3756, inline_record=16520; 不是独立调用边。
// [PDB 内联宿主] system_color::color_menu::color_menu; module=3756, inline_record=18364; 不是独立调用边。
// [PDB 内联宿主] system_color::color_menu::color_menu; module=3756, inline_record=18864; 不是独立调用边。
// [PDB 内联宿主] system_color::color_menu::color_menu; module=3756, inline_record=19364; 不是独立调用边。
// [PDB 内联宿主] system_color::color_menu::color_menu; module=3756, inline_record=21224; 不是独立调用边。
// [PDB 内联宿主] system_color::color_menu::color_menu; module=3756, inline_record=22976; 不是独立调用边。
// [PDB 内联宿主] system_color::color_menu::color_menu; module=3756, inline_record=24728; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_color::color_menu::color_menu::closure$4
// 信息 ID: 9ec69401a8af9f7d0ee720dc
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 4, "lexical_owner": "system_color::color_menu::color_menu", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_color::color_menu::color_menu; module=3756, inline_record=34056; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_color::color_menu::delete_color_icon_entities
// 信息 ID: 4100136a8d2b96e0d0126cf9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] b72ca01a2f46e826efb23874: void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::color_icon::CustomizationIcon>,ref$<country_core::resources::color_icon::CustomizationIconMarkedForDeletion> >,tuple$<> >,bevy_ecs::system::commands::Commands)
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::color_icon::CustomizationIcon>,ref$<country_core::resources::color_icon::CustomizationIconMarkedForDeletion> >,tuple$<> >,bevy_ecs::system::commands::Commands),void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::color_icon::CustomizationIcon>,ref$<country_core::resources::color_icon::CustomizationIconMarkedForDeletion> >,tuple$<> >,bevy_ecs::system::commands::Commands)> > > @ 0x1c8c9cd -> 0x2297ff0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::color_icon::CustomizationIcon>,ref$<country_core::resources::color_icon::CustomizationIconMarkedForDeletion> >,tuple$<> >,bevy_ecs::system::commands::Commands),void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::color_icon::CustomizationIcon>,ref$<country_core::resources::color_icon::CustomizationIconMarkedForDeletion> >,tuple$<> >,bevy_ecs::system::commands::Commands)> > > @ 0x1c9291e -> 0x2297ff0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::schedule_system::impl$1::run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::color_icon::CustomizationIcon>,ref$<country_core::resources::color_icon::CustomizationIconMarkedForDeletion> >,tuple$<> >,bevy_ecs::system::commands::Commands),void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::color_icon::CustomizationIcon>,ref$<country_core::resources::color_icon::CustomizationIconMarkedForDeletion> >,tuple$<> >,bevy_ecs::system::commands::Commands)> > @ 0x1ca3d47 -> 0x2297ff0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
