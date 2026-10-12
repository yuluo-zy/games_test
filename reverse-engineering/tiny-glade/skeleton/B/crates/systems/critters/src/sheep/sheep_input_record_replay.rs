// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/systems/critters/src/sheep/sheep_input_record_replay.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: 1944fe7e02a6ffc32d180f91dd433beb75ffdb03f09e6692f3e2ce3499962a8b
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: sheep_input_record_replay.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 2725, 2739, 3884
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
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.29.3"}]
// use nonmax; // namespace candidate
//   package identities: [{"name":"nonmax","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.5.5"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_critters::sheep::sheep_input_record_replay::sheep_input_record_replay
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3884,"record_offset":264,"rva":36812576,"va":5405521696}
// 编译实例 RVAs: 0x231b720
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$glam..f32..sse2..quat..Quat$u20$as$u20$glam..euler..FromEuler$GT$::from_euler_angles
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-22873779","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405521696,"raw_hex":"e84df9a2fe","size":5,"soft_fail":false,"target_va":5382648896,"va":5405522670,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: sheep_input_record_replay.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-30441394","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405521696,"raw_hex":"e84e802ffe","size":5,"soft_fail":false,"target_va":5375081504,"va":5405522893,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140949970
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: sheep_input_record_replay.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-27074116","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405521696,"raw_hex":"e8bce162fe","size":5,"soft_fail":false,"target_va":5378447728,"va":5405521839,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141dbdec0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: sheep_input_record_replay.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5627285","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405521696,"raw_hex":"e86b22aaff","size":5,"soft_fail":false,"target_va":5399895744,"va":5405523024,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141dbf100
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: sheep_input_record_replay.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5622431","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405521696,"raw_hex":"e86135aaff","size":5,"soft_fail":false,"target_va":5399900416,"va":5405522842,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t6018646","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405521696,"raw_hex":"e856d65b00","size":5,"soft_fail":false,"target_va":5411541776,"va":5405523125,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::input::input_stream::InputStream::last_frame_mut
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-27076391","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405521696,"raw_hex":"e8d9d862fe","size":5,"soft_fail":false,"target_va":5378446464,"va":5405522850,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":3884,"record_offset":460,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":4457,"source_line":434,"archive":"_pdb/B/modules/03884.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$58::deref_mut
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":3884,"record_offset":4756,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":9177,"source_line":434,"archive":"_pdb/B/modules/03884.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::query::Query::iter
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":3884,"record_offset":4896,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":13704,"source_line":670,"archive":"_pdb/B/modules/03884.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::query::Query::iter_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":3884,"record_offset":520,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":17020,"source_line":701,"archive":"_pdb/B/modules/03884.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":3884,"record_offset":1504,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":6061,"source_line":106,"archive":"_pdb/B/modules/03884.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":3884,"record_offset":5344,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":4669,"source_line":1967,"archive":"_pdb/B/modules/03884.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::zip
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":3884,"record_offset":928,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":4522,"source_line":586,"archive":"_pdb/B/modules/03884.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3884,"record_offset":5400,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/03884.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":3884,"record_offset":5436,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/03884.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::quat::Quat::from_euler
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/quat.rs
//     evidence={"module":3884,"record_offset":4812,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":28507,"source_line":184,"archive":"_pdb/B/modules/03884.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":3884,"record_offset":5624,"parent_record_offset":5600,"ancestor_record_offset":264,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/03884.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":3884,"record_offset":5600,"parent_record_offset":5508,"ancestor_record_offset":264,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/03884.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3884,"record_offset":760,"parent_record_offset":736,"ancestor_record_offset":264,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/03884.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3884,"record_offset":5536,"parent_record_offset":5508,"ancestor_record_offset":264,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/03884.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3884,"record_offset":5508,"parent_record_offset":5484,"ancestor_record_offset":264,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/03884.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3884,"record_offset":808,"parent_record_offset":784,"ancestor_record_offset":264,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/03884.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3884,"record_offset":784,"parent_record_offset":760,"ancestor_record_offset":264,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/03884.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3884,"record_offset":5484,"parent_record_offset":5460,"ancestor_record_offset":264,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/03884.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3884,"record_offset":736,"parent_record_offset":712,"ancestor_record_offset":264,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/03884.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3884,"record_offset":712,"parent_record_offset":680,"ancestor_record_offset":264,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/03884.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3884,"record_offset":2064,"parent_record_offset":2040,"ancestor_record_offset":264,"inlinee":4613,"source_line":2705,"archive":"_pdb/B/modules/03884.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3884,"record_offset":2088,"parent_record_offset":2064,"ancestor_record_offset":264,"inlinee":4352,"source_line":2680,"archive":"_pdb/B/modules/03884.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$15::from_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3884,"record_offset":5368,"parent_record_offset":5344,"ancestor_record_offset":264,"inlinee":4671,"source_line":3423,"archive":"_pdb/B/modules/03884.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$17::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3884,"record_offset":952,"parent_record_offset":928,"ancestor_record_offset":264,"inlinee":8707,"source_line":3472,"archive":"_pdb/B/modules/03884.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3884,"record_offset":680,"parent_record_offset":652,"ancestor_record_offset":264,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/03884.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::entities
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":3884,"record_offset":3876,"parent_record_offset":1700,"ancestor_record_offset":264,"inlinee":11682,"source_line":466,"archive":"_pdb/B/modules/03884.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::is_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":3884,"record_offset":3152,"parent_record_offset":1700,"ancestor_record_offset":264,"inlinee":11679,"source_line":615,"archive":"_pdb/B/modules/03884.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::table_id
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":3884,"record_offset":3236,"parent_record_offset":1700,"ancestor_record_offset":264,"inlinee":5495,"source_line":460,"archive":"_pdb/B/modules/03884.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::ArchetypeEntity::table_row
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":3884,"record_offset":4072,"parent_record_offset":1700,"ancestor_record_offset":264,"inlinee":17012,"source_line":325,"archive":"_pdb/B/modules/03884.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetypes::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":3884,"record_offset":3068,"parent_record_offset":1700,"ancestor_record_offset":264,"inlinee":11678,"source_line":895,"archive":"_pdb/B/modules/03884.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":3884,"record_offset":488,"parent_record_offset":460,"ancestor_record_offset":264,"inlinee":4459,"source_line":397,"archive":"_pdb/B/modules/03884.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$57::set_changed
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":3884,"record_offset":4780,"parent_record_offset":4756,"ancestor_record_offset":264,"inlinee":9179,"source_line":397,"archive":"_pdb/B/modules/03884.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::StorageSwitch::extract
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3884,"record_offset":2668,"parent_record_offset":2644,"ancestor_record_offset":264,"inlinee":4876,"source_line":2472,"archive":"_pdb/B/modules/03884.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::set_archetype
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3884,"record_offset":3328,"parent_record_offset":1700,"ancestor_record_offset":264,"inlinee":13655,"source_line":1524,"archive":"_pdb/B/modules/03884.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::set_table
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3884,"record_offset":2124,"parent_record_offset":1700,"ancestor_record_offset":264,"inlinee":13656,"source_line":1539,"archive":"_pdb/B/modules/03884.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$37::fetch
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3884,"record_offset":2644,"parent_record_offset":1700,"ancestor_record_offset":264,"inlinee":13686,"source_line":1596,"archive":"_pdb/B/modules/03884.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$37::fetch::closure$0
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3884,"record_offset":2692,"parent_record_offset":2668,"ancestor_record_offset":264,"inlinee":9533,"source_line":1602,"archive":"_pdb/B/modules/03884.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIter::new
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs
//     evidence={"module":3884,"record_offset":628,"parent_record_offset":544,"ancestor_record_offset":264,"inlinee":13709,"source_line":39,"archive":"_pdb/B/modules/03884.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::init
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs
//     evidence={"module":3884,"record_offset":652,"parent_record_offset":628,"ancestor_record_offset":264,"inlinee":9292,"source_line":2359,"archive":"_pdb/B/modules/03884.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs
//     evidence={"module":3884,"record_offset":1700,"parent_record_offset":1668,"ancestor_record_offset":264,"inlinee":11674,"source_line":2458,"archive":"_pdb/B/modules/03884.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::impl$1::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs
//     evidence={"module":3884,"record_offset":1668,"parent_record_offset":1632,"ancestor_record_offset":264,"inlinee":11673,"source_line":884,"archive":"_pdb/B/modules/03884.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_array::BlobArray::get_sub_slice
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_array.rs
//     evidence={"module":3884,"record_offset":2496,"parent_record_offset":2472,"ancestor_record_offset":264,"inlinee":4872,"source_line":143,"archive":"_pdb/B/modules/03884.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseArray::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":3884,"record_offset":2204,"parent_record_offset":2180,"ancestor_record_offset":264,"inlinee":4864,"source_line":58,"archive":"_pdb/B/modules/03884.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":3884,"record_offset":2180,"parent_record_offset":2156,"ancestor_record_offset":264,"inlinee":4862,"source_line":395,"archive":"_pdb/B/modules/03884.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$16::get::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":3884,"record_offset":2340,"parent_record_offset":2316,"ancestor_record_offset":264,"inlinee":4866,"source_line":396,"archive":"_pdb/B/modules/03884.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_column
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":3884,"record_offset":2156,"parent_record_offset":2124,"ancestor_record_offset":264,"inlinee":4860,"source_line":475,"archive":"_pdb/B/modules/03884.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::is_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":3884,"record_offset":2040,"parent_record_offset":1700,"ancestor_record_offset":264,"inlinee":11675,"source_line":615,"archive":"_pdb/B/modules/03884.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::TableRow::as_usize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":3884,"record_offset":2728,"parent_record_offset":2692,"ancestor_record_offset":264,"inlinee":5472,"source_line":127,"archive":"_pdb/B/modules/03884.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":3884,"record_offset":1956,"parent_record_offset":1700,"ancestor_record_offset":264,"inlinee":4854,"source_line":721,"archive":"_pdb/B/modules/03884.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::column::ThinColumn::get_added_ticks_slice
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/column.rs
//     evidence={"module":3884,"record_offset":2528,"parent_record_offset":2124,"ancestor_record_offset":264,"inlinee":33100,"source_line":311,"archive":"_pdb/B/modules/03884.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::column::ThinColumn::get_changed_ticks_slice
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/column.rs
//     evidence={"module":3884,"record_offset":2584,"parent_record_offset":2124,"ancestor_record_offset":264,"inlinee":13683,"source_line":319,"archive":"_pdb/B/modules/03884.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::column::ThinColumn::get_data_slice
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/column.rs
//     evidence={"module":3884,"record_offset":2472,"parent_record_offset":2124,"ancestor_record_offset":264,"inlinee":4870,"source_line":303,"archive":"_pdb/B/modules/03884.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::thin_array_ptr::ThinArrayPtr::as_slice
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/thin_array_ptr.rs
//     evidence={"module":3884,"record_offset":2552,"parent_record_offset":2528,"ancestor_record_offset":264,"inlinee":13684,"source_line":300,"archive":"_pdb/B/modules/03884.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::as_readonly
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":3884,"record_offset":4920,"parent_record_offset":4896,"ancestor_record_offset":264,"inlinee":13705,"source_line":542,"archive":"_pdb/B/modules/03884.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::copy_unsafe
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":3884,"record_offset":596,"parent_record_offset":572,"ancestor_record_offset":264,"inlinee":13707,"source_line":636,"archive":"_pdb/B/modules/03884.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::reborrow
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":3884,"record_offset":544,"parent_record_offset":520,"ancestor_record_offset":264,"inlinee":17021,"source_line":601,"archive":"_pdb/B/modules/03884.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::reborrow_unsafe
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":3884,"record_offset":572,"parent_record_offset":544,"ancestor_record_offset":264,"inlinee":13706,"source_line":618,"archive":"_pdb/B/modules/03884.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::impl$4::into_iter
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":3884,"record_offset":5696,"parent_record_offset":5672,"ancestor_record_offset":264,"inlinee":25723,"source_line":2574,"archive":"_pdb/B/modules/03884.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::archetypes
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":3884,"record_offset":5036,"parent_record_offset":5004,"ancestor_record_offset":264,"inlinee":13723,"source_line":263,"archive":"_pdb/B/modules/03884.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ptr::ThinSlicePtr::get
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bevy_ptr","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ptr-0.16.0/src/lib.rs
//     evidence={"module":3884,"record_offset":2792,"parent_record_offset":2692,"ancestor_record_offset":264,"inlinee":4880,"source_line":526,"archive":"_pdb/B/modules/03884.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$1::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":3884,"record_offset":1536,"parent_record_offset":1504,"ancestor_record_offset":264,"inlinee":4538,"source_line":85,"archive":"_pdb/B/modules/03884.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$3::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":3884,"record_offset":1568,"parent_record_offset":1536,"ancestor_record_offset":264,"inlinee":4925,"source_line":166,"archive":"_pdb/B/modules/03884.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":3884,"record_offset":5568,"parent_record_offset":5536,"ancestor_record_offset":264,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/03884.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":3884,"record_offset":2252,"parent_record_offset":2228,"ancestor_record_offset":264,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/03884.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3884,"record_offset":2228,"parent_record_offset":2204,"ancestor_record_offset":264,"inlinee":4518,"source_line":1447,"archive":"_pdb/B/modules/03884.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3884,"record_offset":2276,"parent_record_offset":2252,"ancestor_record_offset":264,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/03884.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3884,"record_offset":2316,"parent_record_offset":2180,"ancestor_record_offset":264,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/03884.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$39::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3884,"record_offset":4484,"parent_record_offset":1632,"ancestor_record_offset":264,"inlinee":4927,"source_line":2486,"archive":"_pdb/B/modules/03884.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":3884,"record_offset":5460,"parent_record_offset":5436,"ancestor_record_offset":264,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/03884.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3884,"record_offset":876,"parent_record_offset":852,"ancestor_record_offset":264,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/03884.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":3884,"record_offset":1916,"parent_record_offset":1892,"ancestor_record_offset":264,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/03884.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":3884,"record_offset":1840,"parent_record_offset":1804,"ancestor_record_offset":264,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/03884.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":3884,"record_offset":1980,"parent_record_offset":1956,"ancestor_record_offset":264,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/03884.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_noubcheck
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":3884,"record_offset":2420,"parent_record_offset":2396,"ancestor_record_offset":264,"inlinee":4164,"source_line":87,"archive":"_pdb/B/modules/03884.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":3884,"record_offset":2004,"parent_record_offset":1980,"ancestor_record_offset":264,"inlinee":4145,"source_line":226,"archive":"_pdb/B/modules/03884.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":3884,"record_offset":2396,"parent_record_offset":2364,"ancestor_record_offset":264,"inlinee":4162,"source_line":242,"archive":"_pdb/B/modules/03884.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":3884,"record_offset":852,"parent_record_offset":680,"ancestor_record_offset":264,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/03884.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":3884,"record_offset":1892,"parent_record_offset":1868,"ancestor_record_offset":264,"inlinee":4510,"source_line":99,"archive":"_pdb/B/modules/03884.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":3884,"record_offset":1804,"parent_record_offset":1700,"ancestor_record_offset":264,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/03884.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$186::next_unchecked
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":3884,"record_offset":1868,"parent_record_offset":1804,"ancestor_record_offset":264,"inlinee":4509,"source_line":440,"archive":"_pdb/B/modules/03884.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"nonmax","version":"0.5.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":3884,"record_offset":2364,"parent_record_offset":2340,"ancestor_record_offset":264,"inlinee":4171,"source_line":153,"archive":"_pdb/B/modules/03884.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_critters::sheep::sheep_input_record_replay::sheep_input_record_replay::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 29
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x184FC
// 首条位置: {"module":2725,"inlinee":99580}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_critters::sheep::sheep_input_record_replay::sheep_input_record_replay::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 41
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x184FD
// 首条位置: {"module":2725,"inlinee":99581}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::sse2::quat::Quat::to_euler
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/quat.rs
//     evidence={"module":2739,"record_offset":8364,"parent_record_offset":8340,"ancestor_record_offset":8340,"inlinee":29987,"source_line":426,"archive":"_pdb/B/modules/02739.jsonl.gz"}
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
// 完整结构及原始类型记录: sheep_input_record_replay.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: sheep_input_record_replay.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: sheep_input_record_replay.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::query::Query<ref_mut$<utils::components::transform::Transform>,bevy_ecs::query::filter::With<country_core::resources::critters::sheep::Sheep> >,bevy_ecs::change_detection::ResMut<country_core::input::input_stream::InputRecordReplayState>,bevy_ecs::change_detection::Res<country_core::input::panic_repro_state::PanicReproState>,bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>)
// 候选参数类型: ["bevy_ecs::system::query::Query<ref_mut$<utils::components::transform::Transform>,bevy_ecs::query::filter::With<country_core::resources::critters::sheep::Sheep> >", "bevy_ecs::change_detection::ResMut<country_core::input::input_stream::InputRecordReplayState>", "bevy_ecs::change_detection::Res<country_core::input::panic_repro_state::PanicReproState>", "bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>"]
// [候选系统接口] query_with_filter: bevy_ecs::query::filter::With<country_core::resources::critters::sheep::Sheep>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<ref_mut$<utils::components::transform::Transform>,bevy_ecs::query::filter::With<country_core::resources::critters::sheep::Sheep> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::input::input_stream::InputRecordReplayState>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::input::panic_repro_state::PanicReproState>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>

// ------------------------------------------------------------------
// 函数: system_critters::sheep::sheep_input_record_replay::sheep_input_record_replay
// 信息 ID: d5a56e315ce38e86b581a7b0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 1fcca224b2574abbeaac05ae: void (*)(bevy_ecs::system::query::Query<ref_mut$<utils::components::transform::Transform>,bevy_ecs::query::filter::With<country_core::resources::critters::sheep::Sheep> >,bevy_ecs::change_detection::ResMut<country_core::input::input_stream::InputRecordReplayState>,bevy_ecs::change_detection::Res<country_core::input::panic_repro_state::PanicReproState>,bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<ref_mut$<utils::components::transform::Transform>,bevy_ecs::query::filter::With<country_core::resources::critters::sheep::Sheep> >,bevy_ecs::change_detection::ResMut<country_core::input::input_stream::InputRecordReplayState>,bevy_ecs::change_detection::Res<country_core::input::panic_repro_state::PanicReproState>,bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>),void (*)(bevy_ecs::system::query::Query<ref_mut$<utils::components::transform::Transform>,bevy_ecs::query::filter::With<country_core::resources::critters::sheep::Sheep> >,bevy_ecs::change_detection::ResMut<country_core::input::input_stream::InputRecordReplayState>,bevy_ecs::change_detection::Res<country_core::input::panic_repro_state::PanicReproState>,bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>)> > > @ 0x1d9c554 -> 0x231b720 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::schedule_system::impl$1::run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<ref_mut$<utils::components::transform::Transform>,bevy_ecs::query::filter::With<country_core::resources::critters::sheep::Sheep> >,bevy_ecs::change_detection::ResMut<country_core::input::input_stream::InputRecordReplayState>,bevy_ecs::change_detection::Res<country_core::input::panic_repro_state::PanicReproState>,bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>),void (*)(bevy_ecs::system::query::Query<ref_mut$<utils::components::transform::Transform>,bevy_ecs::query::filter::With<country_core::resources::critters::sheep::Sheep> >,bevy_ecs::change_detection::ResMut<country_core::input::input_stream::InputRecordReplayState>,bevy_ecs::change_detection::Res<country_core::input::panic_repro_state::PanicReproState>,bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>)> > @ 0x1dae31d -> 0x231b720 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_critters::sheep::sheep_input_record_replay::sheep_input_record_replay::closure$0
// 信息 ID: 8e3dcbddc05ad2356f997a81
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_critters::sheep::sheep_input_record_replay::sheep_input_record_replay", "captures": "unknown"}
// [闭包环境类型引用] system_critters::sheep::sheep_input_record_replay::sheep_input_record_replay::closure_env$0 <- core::iter::adapters::map::impl$2::next<glam::f32::vec3::Vec3,bevy_ecs::query::iter::QueryIter<ref$<utils::components::transform::Transform>,bevy_ecs::query::filter::With<country_core::resources::critters::sheep::Sheep> >,system_critters::sheep::sheep_input_record_replay::sheep_input_record_replay::closure_env$0>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_critters::sheep::sheep_input_record_replay::sheep_input_record_replay::closure_env$0 <- alloc::vec::spec_from_iter::impl$0::from_iter<glam::f32::vec3::Vec3,core::iter::adapters::map::Map<bevy_ecs::query::iter::QueryIter<ref$<utils::components::transform::Transform>,bevy_ecs::query::filter::With<country_core::resources::critters::sheep::Sheep> >,system_critters::sheep::sheep_input_record_replay::sheep_input_record_replay::closure_env$0> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 2 条。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::next<glam::f32::vec3::Vec3,bevy_ecs::query::iter::QueryIter<ref$<utils::components::transform::Transform>,bevy_ecs::query::filter::With<country_core::resources::critters::sheep::Sheep> >,system_critters::sheep::sheep_input_record_replay::sheep_input_record_replay::closure_env$0>; module=2739, inline_record=25780; 不是独立调用边。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<glam::f32::vec3::Vec3,core::iter::adapters::map::Map<bevy_ecs::query::iter::QueryIter<ref$<utils::components::transform::Transform>,bevy_ecs::query::filter::With<country_core::resources::critters::sheep::Sheep> >,system_critters::sheep::sheep_input_record_replay::sheep_input_record_replay::closure_env$0> >; module=2739, inline_record=64156; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_critters::sheep::sheep_input_record_replay::sheep_input_record_replay::closure$1
// 信息 ID: 00efae59e0bb4791d2104e0c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "system_critters::sheep::sheep_input_record_replay::sheep_input_record_replay", "captures": "unknown"}
// [闭包环境类型引用] system_critters::sheep::sheep_input_record_replay::sheep_input_record_replay::closure_env$1 <- core::iter::adapters::map::impl$2::next<f32,bevy_ecs::query::iter::QueryIter<ref$<utils::components::transform::Transform>,bevy_ecs::query::filter::With<country_core::resources::critters::sheep::Sheep> >,system_critters::sheep::sheep_input_record_replay::sheep_input_record_replay::closure_env$1>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_critters::sheep::sheep_input_record_replay::sheep_input_record_replay::closure_env$1 <- alloc::vec::spec_from_iter::impl$0::from_iter<f32,core::iter::adapters::map::Map<bevy_ecs::query::iter::QueryIter<ref$<utils::components::transform::Transform>,bevy_ecs::query::filter::With<country_core::resources::critters::sheep::Sheep> >,system_critters::sheep::sheep_input_record_replay::sheep_input_record_replay::closure_env$1> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 2 条。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::next<f32,bevy_ecs::query::iter::QueryIter<ref$<utils::components::transform::Transform>,bevy_ecs::query::filter::With<country_core::resources::critters::sheep::Sheep> >,system_critters::sheep::sheep_input_record_replay::sheep_input_record_replay::closure_env$1>; module=2739, inline_record=8340; 不是独立调用边。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<f32,core::iter::adapters::map::Map<bevy_ecs::query::iter::QueryIter<ref$<utils::components::transform::Transform>,bevy_ecs::query::filter::With<country_core::resources::critters::sheep::Sheep> >,system_critters::sheep::sheep_input_record_replay::sheep_input_record_replay::closure_env$1> >; module=2739, inline_record=42196; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
