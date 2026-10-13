// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/systems/critters/src/sheep/sheep_umbrella.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: b4d2c9ae4216ed9db8be88091f797f95ed14cf8a44ad26a19e3f6375987a43ed
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: sheep_umbrella.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 3616
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
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_critters::sheep::sheep_umbrella::sheep_umbrellas
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3616,"record_offset":252,"rva":28546512,"va":5397255632}
// 编译实例 RVAs: 0x1b395d0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: sheep_umbrella.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-24507278","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397255632,"raw_hex":"e8720c8afe","size":5,"soft_fail":false,"target_va":5372749712,"va":5397256985,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405b2e90
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: sheep_umbrella.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-22571990","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397255632,"raw_hex":"e82a94a7fe","size":5,"soft_fail":false,"target_va":5374684816,"va":5397256801,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406e5f70
//     owner=unresolved_namespace; count=2
//     target_aliases=22 candidates; complete list: sheep_umbrella.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-21314749","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397255632,"raw_hex":"e843c3bafe","size":5,"soft_fail":false,"target_va":5375942512,"va":5397257256,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406f21d0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: sheep_umbrella.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-21264550","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397255632,"raw_hex":"e85a87bbfe","size":5,"soft_fail":false,"target_va":5375992272,"va":5397256817,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve::do_reserve_and_handle
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t5662053","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397255632,"raw_hex":"e865655600","size":5,"soft_fail":false,"target_va":5402919072,"va":5397257014,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..ops..drop..Drop$u20$for$u20$anyhow..Error$GT$::drop
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-19893277","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397255632,"raw_hex":"e8e373d0fe","size":5,"soft_fail":false,"target_va":5377364336,"va":5397257608,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::query::state::QueryState$LT$D$C$F$GT$::get_unchecked_manual
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-6167331","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397255632,"raw_hex":"e8dde4a1ff","size":5,"soft_fail":false,"target_va":5391089264,"va":5397256590,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::system::commands::Commands::spawn
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-5836956","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397255632,"raw_hex":"e864efa6ff","size":5,"soft_fail":false,"target_va":5391419392,"va":5397256343,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::world::command_queue::CommandQueue::get_raw
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-20009899","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397255632,"raw_hex":"e855accefe","size":5,"soft_fail":false,"target_va":5377246528,"va":5397256422,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t5721788","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397255632,"raw_hex":"e8bc4e5700","size":5,"soft_fail":false,"target_va":5402978880,"va":5397257087,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t5722982","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397255632,"raw_hex":"e866535700","size":5,"soft_fail":false,"target_va":5402980128,"va":5397257141,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::audio_resources::AudioCore::instantiate_audio
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-22574729","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397255632,"raw_hex":"e87789a7fe","size":5,"soft_fail":false,"target_va":5374682016,"va":5397256740,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::audio_resources::AudioInstance::start_if_not_playing
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-22574158","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397255632,"raw_hex":"e8b28ba7fe","size":5,"soft_fail":false,"target_va":5374682656,"va":5397256809,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::easing_functions::EasingFunction::apply
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-20317851","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397255632,"raw_hex":"e865f9c9fe","size":5,"soft_fail":false,"target_va":5376938064,"va":5397255910,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$50::deref_mut
//     owner=git_dependency_definition; count=6
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":3616,"record_offset":624,"parent_record_offset":252,"ancestor_record_offset":252,"inlinee":7127,"source_line":326,"archive":"_pdb/A/modules/03616.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::query::iter::impl$1::next
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs
//     evidence={"module":3616,"record_offset":880,"parent_record_offset":252,"ancestor_record_offset":252,"inlinee":14613,"source_line":881,"archive":"_pdb/A/modules/03616.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::commands::EntityCommands::insert
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs
//     evidence={"module":3616,"record_offset":1712,"parent_record_offset":252,"ancestor_record_offset":252,"inlinee":14636,"source_line":945,"archive":"_pdb/A/modules/03616.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::query::Query::get_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/query.rs
//     evidence={"module":3616,"record_offset":2712,"parent_record_offset":252,"ancestor_record_offset":252,"inlinee":14710,"source_line":963,"archive":"_pdb/A/modules/03616.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::query::Query::iter_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/query.rs
//     evidence={"module":3616,"record_offset":428,"parent_record_offset":252,"ancestor_record_offset":252,"inlinee":14631,"source_line":473,"archive":"_pdb/A/modules/03616.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":3616,"record_offset":736,"parent_record_offset":252,"ancestor_record_offset":252,"inlinee":6093,"source_line":1519,"archive":"_pdb/A/modules/03616.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3616,"record_offset":2676,"parent_record_offset":252,"ancestor_record_offset":252,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/03616.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3616,"record_offset":2928,"parent_record_offset":252,"ancestor_record_offset":252,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/03616.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":3616,"record_offset":2852,"parent_record_offset":252,"ancestor_record_offset":252,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/03616.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$9::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":3616,"record_offset":768,"parent_record_offset":252,"ancestor_record_offset":252,"inlinee":14672,"source_line":1048,"archive":"_pdb/A/modules/03616.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3616,"record_offset":3116,"parent_record_offset":3092,"ancestor_record_offset":252,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/03616.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3616,"record_offset":3092,"parent_record_offset":3036,"ancestor_record_offset":252,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/03616.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::capacity
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3616,"record_offset":1984,"parent_record_offset":1956,"ancestor_record_offset":252,"inlinee":4277,"source_line":270,"archive":"_pdb/A/modules/03616.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3616,"record_offset":3064,"parent_record_offset":3036,"ancestor_record_offset":252,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/03616.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::needs_to_grow
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3616,"record_offset":1956,"parent_record_offset":1920,"ancestor_record_offset":252,"inlinee":4276,"source_line":414,"archive":"_pdb/A/modules/03616.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3616,"record_offset":2088,"parent_record_offset":2064,"ancestor_record_offset":252,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/03616.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3616,"record_offset":1920,"parent_record_offset":1888,"ancestor_record_offset":252,"inlinee":4275,"source_line":318,"archive":"_pdb/A/modules/03616.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3616,"record_offset":3036,"parent_record_offset":3008,"ancestor_record_offset":252,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/03616.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3616,"record_offset":2064,"parent_record_offset":1844,"ancestor_record_offset":252,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/03616.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3616,"record_offset":2036,"parent_record_offset":1844,"ancestor_record_offset":252,"inlinee":4292,"source_line":2202,"archive":"_pdb/A/modules/03616.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3616,"record_offset":1888,"parent_record_offset":1844,"ancestor_record_offset":252,"inlinee":4274,"source_line":944,"archive":"_pdb/A/modules/03616.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$49::set_changed
//     owner=git_dependency_definition; count=6
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":3616,"record_offset":648,"parent_record_offset":624,"ancestor_record_offset":252,"inlinee":7129,"source_line":309,"archive":"_pdb/A/modules/03616.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$21::fetch
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":3616,"record_offset":1524,"parent_record_offset":1500,"ancestor_record_offset":252,"inlinee":14627,"source_line":963,"archive":"_pdb/A/modules/03616.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$31::fetch
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":3616,"record_offset":1616,"parent_record_offset":1500,"ancestor_record_offset":252,"inlinee":23287,"source_line":1305,"archive":"_pdb/A/modules/03616.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$31::set_table
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":3616,"record_offset":1212,"parent_record_offset":1188,"ancestor_record_offset":252,"inlinee":23286,"source_line":1291,"archive":"_pdb/A/modules/03616.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIter::new
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs
//     evidence={"module":3616,"record_offset":476,"parent_record_offset":452,"ancestor_record_offset":252,"inlinee":14609,"source_line":36,"archive":"_pdb/A/modules/03616.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::init
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs
//     evidence={"module":3616,"record_offset":500,"parent_record_offset":476,"ancestor_record_offset":252,"inlinee":14611,"source_line":1520,"archive":"_pdb/A/modules/03616.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::next
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs
//     evidence={"module":3616,"record_offset":904,"parent_record_offset":880,"ancestor_record_offset":252,"inlinee":14614,"source_line":1587,"archive":"_pdb/A/modules/03616.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::state::QueryState::iter_unchecked_manual
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/state.rs
//     evidence={"module":3616,"record_offset":452,"parent_record_offset":428,"ancestor_record_offset":252,"inlinee":14607,"source_line":1240,"archive":"_pdb/A/modules/03616.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$3::fetch
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/world_query.rs
//     evidence={"module":3616,"record_offset":1500,"parent_record_offset":904,"ancestor_record_offset":252,"inlinee":22691,"source_line":201,"archive":"_pdb/A/modules/03616.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$3::set_table
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/world_query.rs
//     evidence={"module":3616,"record_offset":1188,"parent_record_offset":904,"ancestor_record_offset":252,"inlinee":22690,"source_line":192,"archive":"_pdb/A/modules/03616.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_slice
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":3616,"record_offset":1460,"parent_record_offset":1436,"ancestor_record_offset":252,"inlinee":14643,"source_line":410,"archive":"_pdb/A/modules/03616.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":3616,"record_offset":1264,"parent_record_offset":1240,"ancestor_record_offset":252,"inlinee":6337,"source_line":369,"archive":"_pdb/A/modules/03616.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$16::get::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":3616,"record_offset":1312,"parent_record_offset":1288,"ancestor_record_offset":252,"inlinee":6342,"source_line":370,"archive":"_pdb/A/modules/03616.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Column::get_data_slice
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":3616,"record_offset":1436,"parent_record_offset":1212,"ancestor_record_offset":252,"inlinee":14642,"source_line":321,"archive":"_pdb/A/modules/03616.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::entities
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":3616,"record_offset":1160,"parent_record_offset":904,"ancestor_record_offset":252,"inlinee":14616,"source_line":568,"archive":"_pdb/A/modules/03616.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_column
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":3616,"record_offset":1240,"parent_record_offset":1212,"ancestor_record_offset":252,"inlinee":14621,"source_line":692,"archive":"_pdb/A/modules/03616.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::TableRow::as_usize
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":3616,"record_offset":1548,"parent_record_offset":1524,"ancestor_record_offset":252,"inlinee":14629,"source_line":125,"archive":"_pdb/A/modules/03616.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":3616,"record_offset":1076,"parent_record_offset":904,"ancestor_record_offset":252,"inlinee":14618,"source_line":834,"archive":"_pdb/A/modules/03616.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::add
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs
//     evidence={"module":3616,"record_offset":1768,"parent_record_offset":1740,"ancestor_record_offset":252,"inlinee":14638,"source_line":748,"archive":"_pdb/A/modules/03616.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::push
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs
//     evidence={"module":3616,"record_offset":1800,"parent_record_offset":1768,"ancestor_record_offset":252,"inlinee":12140,"source_line":501,"archive":"_pdb/A/modules/03616.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::EntityCommands::add
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs
//     evidence={"module":3616,"record_offset":1740,"parent_record_offset":1712,"ancestor_record_offset":252,"inlinee":14637,"source_line":1100,"archive":"_pdb/A/modules/03616.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::command_queue::CommandQueue::push
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/command_queue.rs
//     evidence={"module":3616,"record_offset":2208,"parent_record_offset":1800,"ancestor_record_offset":252,"inlinee":12149,"source_line":78,"archive":"_pdb/A/modules/03616.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::command_queue::RawCommandQueue::push
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/command_queue.rs
//     evidence={"module":3616,"record_offset":1844,"parent_record_offset":1800,"ancestor_record_offset":252,"inlinee":12142,"source_line":157,"archive":"_pdb/A/modules/03616.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ptr::ThinSlicePtr::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ptr/src/lib.rs
//     evidence={"module":3616,"record_offset":1640,"parent_record_offset":1616,"ancestor_record_offset":252,"inlinee":21393,"source_line":500,"archive":"_pdb/A/modules/03616.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":3616,"record_offset":2168,"parent_record_offset":2144,"ancestor_record_offset":252,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/03616.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":3616,"record_offset":792,"parent_record_offset":768,"ancestor_record_offset":252,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/03616.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3616,"record_offset":1288,"parent_record_offset":1264,"ancestor_record_offset":252,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/03616.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":3616,"record_offset":1384,"parent_record_offset":1360,"ancestor_record_offset":252,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/03616.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3616,"record_offset":2896,"parent_record_offset":2852,"ancestor_record_offset":252,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/03616.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3616,"record_offset":572,"parent_record_offset":548,"ancestor_record_offset":252,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/03616.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::write_unaligned
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3616,"record_offset":2120,"parent_record_offset":1844,"ancestor_record_offset":252,"inlinee":12143,"source_line":1546,"archive":"_pdb/A/modules/03616.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":3616,"record_offset":1040,"parent_record_offset":1016,"ancestor_record_offset":252,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/03616.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":3616,"record_offset":988,"parent_record_offset":952,"ancestor_record_offset":252,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/03616.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write_unaligned
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3616,"record_offset":2144,"parent_record_offset":2120,"ancestor_record_offset":252,"inlinee":12144,"source_line":1579,"archive":"_pdb/A/modules/03616.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":3616,"record_offset":1100,"parent_record_offset":1076,"ancestor_record_offset":252,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/03616.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":3616,"record_offset":1336,"parent_record_offset":1312,"ancestor_record_offset":252,"inlinee":5964,"source_line":679,"archive":"_pdb/A/modules/03616.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":3616,"record_offset":524,"parent_record_offset":500,"ancestor_record_offset":252,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/03616.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":3616,"record_offset":1124,"parent_record_offset":1100,"ancestor_record_offset":252,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/03616.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":3616,"record_offset":1360,"parent_record_offset":1336,"ancestor_record_offset":252,"inlinee":4373,"source_line":232,"archive":"_pdb/A/modules/03616.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":3616,"record_offset":548,"parent_record_offset":524,"ancestor_record_offset":252,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/03616.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":3616,"record_offset":1016,"parent_record_offset":952,"ancestor_record_offset":252,"inlinee":5091,"source_line":101,"archive":"_pdb/A/modules/03616.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":3616,"record_offset":952,"parent_record_offset":904,"ancestor_record_offset":252,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/03616.jsonl.gz"}
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
// 完整结构及原始类型记录: sheep_umbrella.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: sheep_umbrella.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: sheep_umbrella.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::query::Query<tuple$<ref$<utils::components::transform::Transform>,ref$<enum2$<country_core::resources::critters::CritterLocomotion> >,ref_mut$<country_core::resources::critters::sheep::UmbrellaHolder> >,bevy_ecs::query::filter::With<country_core::resources::critters::sheep::Sheep> >,bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::critters::sheep::SheepAccessory>,ref_mut$<utils::components::transform::Transform> >,bevy_ecs::query::filter::Without<country_core::resources::critters::sheep::Sheep> >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)
// 候选参数类型: ["bevy_ecs::system::query::Query<tuple$<ref$<utils::components::transform::Transform>,ref$<enum2$<country_core::resources::critters::CritterLocomotion> >,ref_mut$<country_core::resources::critters::sheep::UmbrellaHolder> >,bevy_ecs::query::filter::With<country_core::resources::critters::sheep::Sheep> >", "bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::critters::sheep::SheepAccessory>,ref_mut$<utils::components::transform::Transform> >,bevy_ecs::query::filter::Without<country_core::resources::critters::sheep::Sheep> >", "bevy_ecs::system::commands::Commands", "bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>", "bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>"]
// [候选系统接口] query_with_filter: bevy_ecs::query::filter::With<country_core::resources::critters::sheep::Sheep>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<ref$<utils::components::transform::Transform>,ref$<enum2$<country_core::resources::critters::CritterLocomotion> >,ref_mut$<country_core::resources::critters::sheep::UmbrellaHolder> >,bevy_ecs::query::filter::With<country_core::resources::critters::sheep::Sheep> >
// [候选系统接口] query_without_filter: bevy_ecs::query::filter::Without<country_core::resources::critters::sheep::Sheep>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::critters::sheep::SheepAccessory>,ref_mut$<utils::components::transform::Transform> >,bevy_ecs::query::filter::Without<country_core::resources::critters::sheep::Sheep> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands

// ------------------------------------------------------------------
// 函数: system_critters::sheep::sheep_umbrella::sheep_umbrellas
// 信息 ID: 0ba58d109bbab8be1233bdbb
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 3b47c9e896ee223b6ab00e3f: void (*)(bevy_ecs::system::query::Query<tuple$<ref$<utils::components::transform::Transform>,ref$<enum2$<country_core::resources::critters::CritterLocomotion> >,ref_mut$<country_core::resources::critters::sheep::UmbrellaHolder> >,bevy_ecs::query::filter::With<country_core::resources::critters::sheep::Sheep> >,bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::critters::sheep::SheepAccessory>,ref_mut$<utils::components::transform::Transform> >,bevy_ecs::query::filter::Without<country_core::resources::critters::sheep::Sheep> >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref$<utils::components::transform::Transform>,ref$<enum2$<country_core::resources::critters::CritterLocomotion> >,ref_mut$<country_core::resources::critters::sheep::UmbrellaHolder> >,bevy_ecs::query::filter::With<country_core::resources::critters::sheep::Sheep> >,bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::critters::sheep::SheepAccessory>,ref_mut$<utils::components::transform::Transform> >,bevy_ecs::query::filter::Without<country_core::resources::critters::sheep::Sheep> >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>),void (*)(bevy_ecs::system::query::Query<tuple$<ref$<utils::components::transform::Transform>,ref$<enum2$<country_core::resources::critters::CritterLocomotion> >,ref_mut$<country_core::resources::critters::sheep::UmbrellaHolder> >,bevy_ecs::query::filter::With<country_core::resources::critters::sheep::Sheep> >,bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::critters::sheep::SheepAccessory>,ref_mut$<utils::components::transform::Transform> >,bevy_ecs::query::filter::Without<country_core::resources::critters::sheep::Sheep> >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)> > @ 0x158245a -> 0x1b395d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::system::query::Query<tuple$<ref$<utils::components::transform::Transform>,ref$<enum2$<country_core::resources::critters::CritterLocomotion> >,ref_mut$<country_core::resources::critters::sheep::UmbrellaHolder> >,bevy_ecs::query::filter::With<country_core::resources::critters::sheep::Sheep> >,bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::critters::sheep::SheepAccessory>,ref_mut$<utils::components::transform::Transform> >,bevy_ecs::query::filter::Without<country_core::resources::critters::sheep::Sheep> >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>),void (*)(bevy_ecs::system::query::Query<tuple$<ref$<utils::components::transform::Transform>,ref$<enum2$<country_core::resources::critters::CritterLocomotion> >,ref_mut$<country_core::resources::critters::sheep::UmbrellaHolder> >,bevy_ecs::query::filter::With<country_core::resources::critters::sheep::Sheep> >,bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::critters::sheep::SheepAccessory>,ref_mut$<utils::components::transform::Transform> >,bevy_ecs::query::filter::Without<country_core::resources::critters::sheep::Sheep> >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)> @ 0x1596a10 -> 0x1b395d0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
