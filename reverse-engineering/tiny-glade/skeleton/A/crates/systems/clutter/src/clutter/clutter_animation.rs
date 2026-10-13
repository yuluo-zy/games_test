// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/systems/clutter/src/clutter/clutter_animation.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 501e1ab2d5df096ebbc4765ef33b7011a4da497b4624b5f10e11fd521faebbd4
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: clutter_animation.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 3506
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use bevy_ecs; // namespace candidate
//   package identities: []
// use bevy_ptr; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"PDB_registry_path_only","version":"0.27.0"}]
// use lerp; // namespace candidate
//   package identities: [{"name":"lerp","source":"PDB_registry_path_only","version":"0.4.0"}]
// use nonmax; // namespace candidate
//   package identities: [{"name":"nonmax","source":"PDB_registry_path_only","version":"0.5.5"}]
// use std; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_clutter::clutter::clutter_animation::clutter_animation
// 记录形式: compiled_procedure; instances=1
// 源码行: 13
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3506,"record_offset":252,"rva":28097952,"va":5396807072}
// 编译实例 RVAs: 0x1acbda0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142144d10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: clutter_animation.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t6785378","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396807072,"raw_hex":"e862896700","size":5,"soft_fail":false,"target_va":5403594000,"va":5396808617,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144e60
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: clutter_animation.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t6785725","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396807072,"raw_hex":"e8bd8a6700","size":5,"soft_fail":false,"target_va":5403594336,"va":5396808606,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-25501157","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396807072,"raw_hex":"e81be27afe","size":5,"soft_fail":false,"target_va":5371306912,"va":5396808064,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::entity::Entities::contains
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-19563121","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396807072,"raw_hex":"e88f7dd5fe","size":5,"soft_fail":false,"target_va":5377245296,"va":5396808412,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::system::commands::Commands::entity::panic_no_entity
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t6163175","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396807072,"raw_hex":"e8e70a5e00","size":5,"soft_fail":false,"target_va":5402972080,"va":5396808900,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::system::commands::EntityCommands::despawn
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-19502950","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396807072,"raw_hex":"e89a68d6fe","size":5,"soft_fail":false,"target_va":5377305552,"va":5396808497,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::easing_functions::EasingFunction::apply
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t-19870190","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396807072,"raw_hex":"e812ced0fe","size":5,"soft_fail":false,"target_va":5376938064,"va":5396808249,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$50::deref_mut
//     owner=git_dependency_definition; count=5
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":3506,"record_offset":2472,"parent_record_offset":252,"ancestor_record_offset":252,"inlinee":7127,"source_line":326,"archive":"_pdb/A/modules/03506.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::EventWriter::send
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":3506,"record_offset":2528,"parent_record_offset":252,"ancestor_record_offset":252,"inlinee":7450,"source_line":624,"archive":"_pdb/A/modules/03506.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::query::iter::impl$1::next
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs
//     evidence={"module":3506,"record_offset":644,"parent_record_offset":252,"ancestor_record_offset":252,"inlinee":14613,"source_line":881,"archive":"_pdb/A/modules/03506.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::commands::Commands::entity
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs
//     evidence={"module":3506,"record_offset":3128,"parent_record_offset":252,"ancestor_record_offset":252,"inlinee":14632,"source_line":398,"archive":"_pdb/A/modules/03506.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::query::Query::iter_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/query.rs
//     evidence={"module":3506,"record_offset":444,"parent_record_offset":252,"ancestor_record_offset":252,"inlinee":14631,"source_line":473,"archive":"_pdb/A/modules/03506.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":3506,"record_offset":2872,"parent_record_offset":252,"ancestor_record_offset":252,"inlinee":6093,"source_line":1519,"archive":"_pdb/A/modules/03506.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::to_radians
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":3506,"record_offset":3340,"parent_record_offset":252,"ancestor_record_offset":252,"inlinee":22232,"source_line":903,"archive":"_pdb/A/modules/03506.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3506,"record_offset":2788,"parent_record_offset":252,"ancestor_record_offset":252,"inlinee":4711,"source_line":607,"archive":"_pdb/A/modules/03506.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::quat::Quat::from_axis_angle
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/quat.rs
//     evidence={"module":3506,"record_offset":3368,"parent_record_offset":252,"ancestor_record_offset":252,"inlinee":14670,"source_line":137,"archive":"_pdb/A/modules/03506.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::quat::impl$7::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/quat.rs
//     evidence={"module":3506,"record_offset":3576,"parent_record_offset":252,"ancestor_record_offset":252,"inlinee":59541,"source_line":846,"archive":"_pdb/A/modules/03506.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$9::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":3506,"record_offset":2988,"parent_record_offset":252,"ancestor_record_offset":252,"inlinee":14672,"source_line":1048,"archive":"_pdb/A/modules/03506.jsonl.gz"}
//   [pdb_inline_direct] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lerp","version":"0.4.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lerp-0.4.0/src/lib.rs
//     evidence={"module":3506,"record_offset":3224,"parent_record_offset":252,"ancestor_record_offset":252,"inlinee":10380,"source_line":171,"archive":"_pdb/A/modules/03506.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3506,"record_offset":2716,"parent_record_offset":2684,"ancestor_record_offset":252,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/03506.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3506,"record_offset":2684,"parent_record_offset":2644,"ancestor_record_offset":252,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/03506.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3506,"record_offset":2644,"parent_record_offset":2608,"ancestor_record_offset":252,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/03506.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$29::set_changed
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":3506,"record_offset":2576,"parent_record_offset":2552,"ancestor_record_offset":252,"inlinee":5812,"source_line":309,"archive":"_pdb/A/modules/03506.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":3506,"record_offset":2552,"parent_record_offset":2528,"ancestor_record_offset":252,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/03506.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$49::set_changed
//     owner=git_dependency_definition; count=5
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":3506,"record_offset":2496,"parent_record_offset":2472,"ancestor_record_offset":252,"inlinee":7129,"source_line":309,"archive":"_pdb/A/modules/03506.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::Events::send
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":3506,"record_offset":2608,"parent_record_offset":2528,"ancestor_record_offset":252,"inlinee":7451,"source_line":231,"archive":"_pdb/A/modules/03506.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$21::matches_component_set
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":3506,"record_offset":1064,"parent_record_offset":1032,"ancestor_record_offset":252,"inlinee":4919,"source_line":1006,"archive":"_pdb/A/modules/03506.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$21::set_table
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":3506,"record_offset":1332,"parent_record_offset":1032,"ancestor_record_offset":252,"inlinee":14620,"source_line":948,"archive":"_pdb/A/modules/03506.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$31::fetch
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":3506,"record_offset":2164,"parent_record_offset":2140,"ancestor_record_offset":252,"inlinee":23287,"source_line":1305,"archive":"_pdb/A/modules/03506.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$31::set_table
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":3506,"record_offset":948,"parent_record_offset":924,"ancestor_record_offset":252,"inlinee":23286,"source_line":1291,"archive":"_pdb/A/modules/03506.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::fetch
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":3506,"record_offset":2340,"parent_record_offset":2140,"ancestor_record_offset":252,"inlinee":14630,"source_line":1558,"archive":"_pdb/A/modules/03506.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::set_table
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":3506,"record_offset":1032,"parent_record_offset":924,"ancestor_record_offset":252,"inlinee":14622,"source_line":1547,"archive":"_pdb/A/modules/03506.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::set_table::closure$0
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":3506,"record_offset":1092,"parent_record_offset":1064,"ancestor_record_offset":252,"inlinee":14624,"source_line":1548,"archive":"_pdb/A/modules/03506.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIter::new
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs
//     evidence={"module":3506,"record_offset":496,"parent_record_offset":472,"ancestor_record_offset":252,"inlinee":14609,"source_line":36,"archive":"_pdb/A/modules/03506.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::init
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs
//     evidence={"module":3506,"record_offset":520,"parent_record_offset":496,"ancestor_record_offset":252,"inlinee":14611,"source_line":1520,"archive":"_pdb/A/modules/03506.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::next
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs
//     evidence={"module":3506,"record_offset":676,"parent_record_offset":644,"ancestor_record_offset":252,"inlinee":14614,"source_line":1587,"archive":"_pdb/A/modules/03506.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::state::QueryState::iter_unchecked_manual
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/state.rs
//     evidence={"module":3506,"record_offset":472,"parent_record_offset":444,"ancestor_record_offset":252,"inlinee":14607,"source_line":1240,"archive":"_pdb/A/modules/03506.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$5::fetch
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/world_query.rs
//     evidence={"module":3506,"record_offset":2140,"parent_record_offset":676,"ancestor_record_offset":252,"inlinee":82914,"source_line":201,"archive":"_pdb/A/modules/03506.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$5::set_table
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/world_query.rs
//     evidence={"module":3506,"record_offset":924,"parent_record_offset":676,"ancestor_record_offset":252,"inlinee":82913,"source_line":192,"archive":"_pdb/A/modules/03506.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_slice
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":3506,"record_offset":1524,"parent_record_offset":1500,"ancestor_record_offset":252,"inlinee":14643,"source_line":410,"archive":"_pdb/A/modules/03506.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseArray::contains
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":3506,"record_offset":1164,"parent_record_offset":1140,"ancestor_record_offset":252,"inlinee":4879,"source_line":47,"archive":"_pdb/A/modules/03506.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::contains
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":3506,"record_offset":1140,"parent_record_offset":1116,"ancestor_record_offset":252,"inlinee":4877,"source_line":362,"archive":"_pdb/A/modules/03506.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":3506,"record_offset":996,"parent_record_offset":972,"ancestor_record_offset":252,"inlinee":6337,"source_line":369,"archive":"_pdb/A/modules/03506.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$11::contains::closure$0
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":3506,"record_offset":1252,"parent_record_offset":1188,"ancestor_record_offset":252,"inlinee":4881,"source_line":49,"archive":"_pdb/A/modules/03506.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$16::get::closure$0
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":3506,"record_offset":1432,"parent_record_offset":1408,"ancestor_record_offset":252,"inlinee":6342,"source_line":370,"archive":"_pdb/A/modules/03506.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Column::get_data_slice
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":3506,"record_offset":1500,"parent_record_offset":1332,"ancestor_record_offset":252,"inlinee":14642,"source_line":321,"archive":"_pdb/A/modules/03506.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::entities
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":3506,"record_offset":740,"parent_record_offset":676,"ancestor_record_offset":252,"inlinee":14616,"source_line":568,"archive":"_pdb/A/modules/03506.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_column
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":3506,"record_offset":972,"parent_record_offset":948,"ancestor_record_offset":252,"inlinee":14621,"source_line":692,"archive":"_pdb/A/modules/03506.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::has_column
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":3506,"record_offset":1116,"parent_record_offset":1092,"ancestor_record_offset":252,"inlinee":14625,"source_line":713,"archive":"_pdb/A/modules/03506.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::TableRow::as_usize
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":3506,"record_offset":2188,"parent_record_offset":2164,"ancestor_record_offset":252,"inlinee":14629,"source_line":125,"archive":"_pdb/A/modules/03506.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":3506,"record_offset":840,"parent_record_offset":676,"ancestor_record_offset":252,"inlinee":14618,"source_line":834,"archive":"_pdb/A/modules/03506.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::get_entity
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs
//     evidence={"module":3506,"record_offset":3164,"parent_record_offset":3128,"ancestor_record_offset":252,"inlinee":14633,"source_line":447,"archive":"_pdb/A/modules/03506.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ptr::ThinSlicePtr::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ptr/src/lib.rs
//     evidence={"module":3506,"record_offset":2280,"parent_record_offset":2256,"ancestor_record_offset":252,"inlinee":21393,"source_line":500,"archive":"_pdb/A/modules/03506.jsonl.gz"}
//   [pdb_inline_transitive] core::bool::impl$0::then
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\bool.rs
//     evidence={"module":3506,"record_offset":2368,"parent_record_offset":2340,"ancestor_record_offset":252,"inlinee":4886,"source_line":59,"archive":"_pdb/A/modules/03506.jsonl.gz"}
//   [pdb_inline_transitive] core::bool::impl$0::then_some
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\bool.rs
//     evidence={"module":3506,"record_offset":3188,"parent_record_offset":3164,"ancestor_record_offset":252,"inlinee":14634,"source_line":33,"archive":"_pdb/A/modules/03506.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":3506,"record_offset":3948,"parent_record_offset":3600,"ancestor_record_offset":252,"inlinee":15697,"source_line":31,"archive":"_pdb/A/modules/03506.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":3506,"record_offset":3780,"parent_record_offset":3600,"ancestor_record_offset":252,"inlinee":14684,"source_line":77,"archive":"_pdb/A/modules/03506.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":3506,"record_offset":3668,"parent_record_offset":3600,"ancestor_record_offset":252,"inlinee":14683,"source_line":1011,"archive":"_pdb/A/modules/03506.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":3506,"record_offset":3012,"parent_record_offset":2988,"ancestor_record_offset":252,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/03506.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":3506,"record_offset":3308,"parent_record_offset":3224,"ancestor_record_offset":252,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/03506.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":3506,"record_offset":3252,"parent_record_offset":3224,"ancestor_record_offset":252,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/03506.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3506,"record_offset":1276,"parent_record_offset":1252,"ancestor_record_offset":252,"inlinee":4711,"source_line":607,"archive":"_pdb/A/modules/03506.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3506,"record_offset":1408,"parent_record_offset":1384,"ancestor_record_offset":252,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/03506.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3506,"record_offset":592,"parent_record_offset":568,"ancestor_record_offset":252,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/03506.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":3506,"record_offset":808,"parent_record_offset":776,"ancestor_record_offset":252,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/03506.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3506,"record_offset":2748,"parent_record_offset":2644,"ancestor_record_offset":252,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/03506.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":3506,"record_offset":864,"parent_record_offset":840,"ancestor_record_offset":252,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/03506.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":3506,"record_offset":544,"parent_record_offset":520,"ancestor_record_offset":252,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/03506.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":3506,"record_offset":888,"parent_record_offset":864,"ancestor_record_offset":252,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/03506.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":3506,"record_offset":568,"parent_record_offset":544,"ancestor_record_offset":252,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/03506.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":3506,"record_offset":776,"parent_record_offset":676,"ancestor_record_offset":252,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/03506.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::math::std_math::sin_cos
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/math.rs
//     evidence={"module":3506,"record_offset":3400,"parent_record_offset":3368,"ancestor_record_offset":252,"inlinee":21674,"source_line":177,"archive":"_pdb/A/modules/03506.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::from_xyzw
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/quat.rs
//     evidence={"module":3506,"record_offset":3544,"parent_record_offset":3368,"ancestor_record_offset":252,"inlinee":14673,"source_line":72,"archive":"_pdb/A/modules/03506.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::mul_quat
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/quat.rs
//     evidence={"module":3506,"record_offset":3600,"parent_record_offset":3576,"ancestor_record_offset":252,"inlinee":22239,"source_line":675,"archive":"_pdb/A/modules/03506.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$9::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":3506,"record_offset":3488,"parent_record_offset":3368,"ancestor_record_offset":252,"inlinee":14672,"source_line":1048,"archive":"_pdb/A/modules/03506.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"nonmax","version":"0.5.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":3506,"record_offset":1456,"parent_record_offset":1432,"ancestor_record_offset":252,"inlinee":4372,"source_line":153,"archive":"_pdb/A/modules/03506.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sin
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":3506,"record_offset":3448,"parent_record_offset":3424,"ancestor_record_offset":252,"inlinee":13074,"source_line":713,"archive":"_pdb/A/modules/03506.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sin_cos
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":3506,"record_offset":3424,"parent_record_offset":3400,"ancestor_record_offset":252,"inlinee":21675,"source_line":915,"archive":"_pdb/A/modules/03506.jsonl.gz"}
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
// 完整结构及原始类型记录: clutter_animation.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: clutter_animation.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: clutter_animation.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<system_clutter::clutter::Clutter>,ref_mut$<utils::components::transform::Transform>,enum2$<core::option::Option<ref$<system_clutter::clutter::WindowShutter> > >,enum2$<core::option::Option<ref$<system_clutter::clutter::ClutterMarkedForDeletion> > > >,tuple$<> >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::event::EventWriter<country_core::systems::audio::clutter_audio::PlayClutterSoundCmd>)
// 候选参数类型: ["bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<system_clutter::clutter::Clutter>,ref_mut$<utils::components::transform::Transform>,enum2$<core::option::Option<ref$<system_clutter::clutter::WindowShutter> > >,enum2$<core::option::Option<ref$<system_clutter::clutter::ClutterMarkedForDeletion> > > >,tuple$<> >", "bevy_ecs::system::commands::Commands", "bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>", "bevy_ecs::event::EventWriter<country_core::systems::audio::clutter_audio::PlayClutterSoundCmd>"]
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<system_clutter::clutter::Clutter>,ref_mut$<utils::components::transform::Transform>,enum2$<core::option::Option<ref$<system_clutter::clutter::WindowShutter> > >,enum2$<core::option::Option<ref$<system_clutter::clutter::ClutterMarkedForDeletion> > > >,tuple$<> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::systems::audio::clutter_audio::PlayClutterSoundCmd>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands

// ------------------------------------------------------------------
// 函数: system_clutter::clutter::clutter_animation::clutter_animation
// 信息 ID: 5fc5afc53c60cab4d0580ad0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 679ce562d522cb48ddbf2db2: void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<system_clutter::clutter::Clutter>,ref_mut$<utils::components::transform::Transform>,enum2$<core::option::Option<ref$<system_clutter::clutter::WindowShutter> > >,enum2$<core::option::Option<ref$<system_clutter::clutter::ClutterMarkedForDeletion> > > >,tuple$<> >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::event::EventWriter<country_core::systems::audio::clutter_audio::PlayClutterSoundCmd>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<system_clutter::clutter::Clutter>,ref_mut$<utils::components::transform::Transform>,enum2$<core::option::Option<ref$<system_clutter::clutter::WindowShutter> > >,enum2$<core::option::Option<ref$<system_clutter::clutter::ClutterMarkedForDeletion> > > >,tuple$<> >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::event::EventWriter<country_core::systems::audio::clutter_audio::PlayClutterSoundCmd>),void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<system_clutter::clutter::Clutter>,ref_mut$<utils::components::transform::Transform>,enum2$<core::option::Option<ref$<system_clutter::clutter::WindowShutter> > >,enum2$<core::option::Option<ref$<system_clutter::clutter::ClutterMarkedForDeletion> > > >,tuple$<> >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::event::EventWriter<country_core::systems::audio::clutter_audio::PlayClutterSoundCmd>)> > @ 0x14c5acc -> 0x1acbda0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<system_clutter::clutter::Clutter>,ref_mut$<utils::components::transform::Transform>,enum2$<core::option::Option<ref$<system_clutter::clutter::WindowShutter> > >,enum2$<core::option::Option<ref$<system_clutter::clutter::ClutterMarkedForDeletion> > > >,tuple$<> >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::event::EventWriter<country_core::systems::audio::clutter_audio::PlayClutterSoundCmd>),void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<system_clutter::clutter::Clutter>,ref_mut$<utils::components::transform::Transform>,enum2$<core::option::Option<ref$<system_clutter::clutter::WindowShutter> > >,enum2$<core::option::Option<ref$<system_clutter::clutter::ClutterMarkedForDeletion> > > >,tuple$<> >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::event::EventWriter<country_core::systems::audio::clutter_audio::PlayClutterSoundCmd>)> @ 0x14eacaa -> 0x1acbda0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
