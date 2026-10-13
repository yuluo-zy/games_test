// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/country-core/src/systems/backend/vao_update.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: c554aef15a9e59c99c88fef165180dcd08c4ffb94af0768a17cd74c1878dc8de
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: vao_update.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 430
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
// use log; // namespace candidate
//   package identities: [{"name":"log","source":"PDB_registry_path_only","version":"0.4.21"}]
// use std; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::systems::backend::vao_update::build_missing_vaos
// 记录形式: compiled_procedure; instances=1
// 源码行: 9
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":430,"record_offset":25304,"rva":6034416,"va":5374743536}
// 编译实例 RVAs: 0x5c13f0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140667640
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: vao_update.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t680182","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374743536,"raw_hex":"e8f6600a00","size":5,"soft_fail":false,"target_va":5375424064,"va":5374743877,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve::do_reserve_and_handle
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t28174703","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374743536,"raw_hex":"e86fe9ad01","size":5,"soft_fail":false,"target_va":5402919072,"va":5374744364,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..ops..drop..Drop$u20$for$u20$anyhow..Error$GT$::drop
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t2620615","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374743536,"raw_hex":"e8c7fc2700","size":5,"soft_fail":false,"target_va":5377364336,"va":5374743716,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::entity::Entities::contains
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2501204","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374743536,"raw_hex":"e8542a2600","size":5,"soft_fail":false,"target_va":5377245296,"va":5374744087,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::system::commands::Commands::entity::panic_no_entity
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t28227636","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374743536,"raw_hex":"e834b8ae01","size":5,"soft_fail":false,"target_va":5402972080,"va":5374744439,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::world::command_queue::CommandQueue::get_raw
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2502243","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374743536,"raw_hex":"e8632e2600","size":5,"soft_fail":false,"target_va":5377246528,"va":5374744280,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::asset_libraries::vao_library::AssetVAOLibrary::add
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-86238","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374743536,"raw_hex":"e822affeff","size":5,"soft_fail":false,"target_va":5374657696,"va":5374743929,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] log::__private_api::log_impl
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t2609705","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374743536,"raw_hex":"e829d22700","size":5,"soft_fail":false,"target_va":5377353776,"va":5374744066,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":430,"record_offset":26364,"parent_record_offset":25304,"ancestor_record_offset":25304,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/00430.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::query::iter::impl$1::next
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs
//     evidence={"module":430,"record_offset":25728,"parent_record_offset":25304,"ancestor_record_offset":25304,"inlinee":14613,"source_line":881,"archive":"_pdb/A/modules/00430.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::commands::Commands::entity
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs
//     evidence={"module":430,"record_offset":26644,"parent_record_offset":25304,"ancestor_record_offset":25304,"inlinee":14632,"source_line":398,"archive":"_pdb/A/modules/00430.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::commands::EntityCommands::insert
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs
//     evidence={"module":430,"record_offset":26736,"parent_record_offset":25304,"ancestor_record_offset":25304,"inlinee":14636,"source_line":945,"archive":"_pdb/A/modules/00430.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::query::Query::iter
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/query.rs
//     evidence={"module":430,"record_offset":25504,"parent_record_offset":25304,"ancestor_record_offset":25304,"inlinee":14606,"source_line":435,"archive":"_pdb/A/modules/00430.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":430,"record_offset":26560,"parent_record_offset":25304,"ancestor_record_offset":25304,"inlinee":4715,"source_line":1188,"archive":"_pdb/A/modules/00430.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":430,"record_offset":27816,"parent_record_offset":25304,"ancestor_record_offset":25304,"inlinee":4716,"source_line":329,"archive":"_pdb/A/modules/00430.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":430,"record_offset":26588,"parent_record_offset":25304,"ancestor_record_offset":25304,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/00430.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::map_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":430,"record_offset":26276,"parent_record_offset":25304,"ancestor_record_offset":25304,"inlinee":4773,"source_line":1127,"archive":"_pdb/A/modules/00430.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":430,"record_offset":25700,"parent_record_offset":25304,"ancestor_record_offset":25304,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00430.jsonl.gz"}
//   [pdb_inline_direct] country_core::asset_libraries::mesh_library::AssetMeshLibrary::get
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/mesh_library.rs
//     evidence={"module":430,"record_offset":26248,"parent_record_offset":25304,"ancestor_record_offset":25304,"inlinee":22537,"source_line":63,"archive":"_pdb/A/modules/00430.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::impl$0::deref
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/mod.rs
//     evidence={"module":430,"record_offset":26420,"parent_record_offset":25304,"ancestor_record_offset":25304,"inlinee":22680,"source_line":43,"archive":"_pdb/A/modules/00430.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/__private_api.rs
//     evidence={"module":430,"record_offset":26616,"parent_record_offset":25304,"ancestor_record_offset":25304,"inlinee":4718,"source_line":64,"archive":"_pdb/A/modules/00430.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/lib.rs
//     evidence={"module":430,"record_offset":26476,"parent_record_offset":25304,"ancestor_record_offset":25304,"inlinee":4713,"source_line":1269,"archive":"_pdb/A/modules/00430.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::capacity
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":430,"record_offset":27008,"parent_record_offset":26980,"ancestor_record_offset":25304,"inlinee":4277,"source_line":270,"archive":"_pdb/A/modules/00430.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::needs_to_grow
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":430,"record_offset":26980,"parent_record_offset":26944,"ancestor_record_offset":25304,"inlinee":4276,"source_line":414,"archive":"_pdb/A/modules/00430.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":430,"record_offset":27112,"parent_record_offset":27088,"ancestor_record_offset":25304,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/00430.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":430,"record_offset":26944,"parent_record_offset":26912,"ancestor_record_offset":25304,"inlinee":4275,"source_line":318,"archive":"_pdb/A/modules/00430.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":430,"record_offset":26444,"parent_record_offset":26420,"ancestor_record_offset":25304,"inlinee":4586,"source_line":2105,"archive":"_pdb/A/modules/00430.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":430,"record_offset":27088,"parent_record_offset":26864,"ancestor_record_offset":25304,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/00430.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":430,"record_offset":26328,"parent_record_offset":26304,"ancestor_record_offset":25304,"inlinee":7328,"source_line":2218,"archive":"_pdb/A/modules/00430.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":430,"record_offset":27060,"parent_record_offset":26864,"ancestor_record_offset":25304,"inlinee":4292,"source_line":2202,"archive":"_pdb/A/modules/00430.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":430,"record_offset":26912,"parent_record_offset":26864,"ancestor_record_offset":25304,"inlinee":4274,"source_line":944,"archive":"_pdb/A/modules/00430.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":430,"record_offset":27228,"parent_record_offset":26864,"ancestor_record_offset":25304,"inlinee":5993,"source_line":1455,"archive":"_pdb/A/modules/00430.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$29::set_changed
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":430,"record_offset":26388,"parent_record_offset":26364,"ancestor_record_offset":25304,"inlinee":5812,"source_line":309,"archive":"_pdb/A/modules/00430.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$21::fetch
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":430,"record_offset":26180,"parent_record_offset":26156,"ancestor_record_offset":25304,"inlinee":14627,"source_line":963,"archive":"_pdb/A/modules/00430.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$21::set_table
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":430,"record_offset":26068,"parent_record_offset":26044,"ancestor_record_offset":25304,"inlinee":14620,"source_line":948,"archive":"_pdb/A/modules/00430.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIter::new
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs
//     evidence={"module":430,"record_offset":25552,"parent_record_offset":25528,"ancestor_record_offset":25304,"inlinee":14609,"source_line":36,"archive":"_pdb/A/modules/00430.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::init
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs
//     evidence={"module":430,"record_offset":25576,"parent_record_offset":25552,"ancestor_record_offset":25304,"inlinee":14611,"source_line":1520,"archive":"_pdb/A/modules/00430.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::next
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs
//     evidence={"module":430,"record_offset":25756,"parent_record_offset":25728,"ancestor_record_offset":25304,"inlinee":14614,"source_line":1587,"archive":"_pdb/A/modules/00430.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::state::QueryState::iter_unchecked_manual
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/state.rs
//     evidence={"module":430,"record_offset":25528,"parent_record_offset":25504,"ancestor_record_offset":25304,"inlinee":14607,"source_line":1240,"archive":"_pdb/A/modules/00430.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$2::fetch
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/world_query.rs
//     evidence={"module":430,"record_offset":26156,"parent_record_offset":25756,"ancestor_record_offset":25304,"inlinee":14626,"source_line":201,"archive":"_pdb/A/modules/00430.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$2::set_table
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/world_query.rs
//     evidence={"module":430,"record_offset":26044,"parent_record_offset":25756,"ancestor_record_offset":25304,"inlinee":14619,"source_line":192,"archive":"_pdb/A/modules/00430.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_slice
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":430,"record_offset":26116,"parent_record_offset":26092,"ancestor_record_offset":25304,"inlinee":14643,"source_line":410,"archive":"_pdb/A/modules/00430.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Column::get_data_slice
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":430,"record_offset":26092,"parent_record_offset":26068,"ancestor_record_offset":25304,"inlinee":14642,"source_line":321,"archive":"_pdb/A/modules/00430.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::entities
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":430,"record_offset":26016,"parent_record_offset":25756,"ancestor_record_offset":25304,"inlinee":14616,"source_line":568,"archive":"_pdb/A/modules/00430.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::TableRow::as_usize
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":430,"record_offset":26204,"parent_record_offset":26180,"ancestor_record_offset":25304,"inlinee":14629,"source_line":125,"archive":"_pdb/A/modules/00430.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":430,"record_offset":25932,"parent_record_offset":25756,"ancestor_record_offset":25304,"inlinee":14618,"source_line":834,"archive":"_pdb/A/modules/00430.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::add
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs
//     evidence={"module":430,"record_offset":26792,"parent_record_offset":26764,"ancestor_record_offset":25304,"inlinee":14638,"source_line":748,"archive":"_pdb/A/modules/00430.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::get_entity
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs
//     evidence={"module":430,"record_offset":26676,"parent_record_offset":26644,"ancestor_record_offset":25304,"inlinee":14633,"source_line":447,"archive":"_pdb/A/modules/00430.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::push
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs
//     evidence={"module":430,"record_offset":26820,"parent_record_offset":26792,"ancestor_record_offset":25304,"inlinee":12140,"source_line":501,"archive":"_pdb/A/modules/00430.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::EntityCommands::add
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs
//     evidence={"module":430,"record_offset":26764,"parent_record_offset":26736,"ancestor_record_offset":25304,"inlinee":14637,"source_line":1100,"archive":"_pdb/A/modules/00430.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::command_queue::CommandQueue::push
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/command_queue.rs
//     evidence={"module":430,"record_offset":27260,"parent_record_offset":26820,"ancestor_record_offset":25304,"inlinee":12149,"source_line":78,"archive":"_pdb/A/modules/00430.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::command_queue::RawCommandQueue::push
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/command_queue.rs
//     evidence={"module":430,"record_offset":26864,"parent_record_offset":26820,"ancestor_record_offset":25304,"inlinee":12142,"source_line":157,"archive":"_pdb/A/modules/00430.jsonl.gz"}
//   [pdb_inline_transitive] core::bool::impl$0::then_some
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\bool.rs
//     evidence={"module":430,"record_offset":26700,"parent_record_offset":26676,"ancestor_record_offset":25304,"inlinee":14634,"source_line":33,"archive":"_pdb/A/modules/00430.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":430,"record_offset":27192,"parent_record_offset":27168,"ancestor_record_offset":25304,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00430.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":430,"record_offset":25648,"parent_record_offset":25624,"ancestor_record_offset":25304,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/00430.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::write_unaligned
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":430,"record_offset":27144,"parent_record_offset":26864,"ancestor_record_offset":25304,"inlinee":12143,"source_line":1546,"archive":"_pdb/A/modules/00430.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":430,"record_offset":25896,"parent_record_offset":25872,"ancestor_record_offset":25304,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/00430.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":430,"record_offset":25844,"parent_record_offset":25808,"ancestor_record_offset":25304,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/00430.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write_unaligned
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":430,"record_offset":27168,"parent_record_offset":27144,"ancestor_record_offset":25304,"inlinee":12144,"source_line":1579,"archive":"_pdb/A/modules/00430.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":430,"record_offset":25956,"parent_record_offset":25932,"ancestor_record_offset":25304,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/00430.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":430,"record_offset":25600,"parent_record_offset":25576,"ancestor_record_offset":25304,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/00430.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":430,"record_offset":25980,"parent_record_offset":25956,"ancestor_record_offset":25304,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/00430.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":430,"record_offset":25624,"parent_record_offset":25600,"ancestor_record_offset":25304,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/00430.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":430,"record_offset":25872,"parent_record_offset":25808,"ancestor_record_offset":25304,"inlinee":5091,"source_line":101,"archive":"_pdb/A/modules/00430.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":430,"record_offset":25808,"parent_record_offset":25756,"ancestor_record_offset":25304,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/00430.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":430,"record_offset":26500,"parent_record_offset":26476,"ancestor_record_offset":25304,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/00430.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":430,"record_offset":26524,"parent_record_offset":26500,"ancestor_record_offset":25304,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/00430.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::backend::vao_update::build_missing_vaos::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/backend/vao_update.rs
//     evidence={"module":430,"record_offset":26304,"parent_record_offset":26276,"ancestor_record_offset":25304,"inlinee":24884,"source_line":20,"archive":"_pdb/A/modules/00430.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::backend::vao_update::build_missing_vaos::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 20
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6134
// 首条位置: {"module":430,"inlinee":24884}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":430,"record_offset":26328,"parent_record_offset":26304,"ancestor_record_offset":26304,"inlinee":7328,"source_line":2218,"archive":"_pdb/A/modules/00430.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::backend::vao_update::rebuild_vaos
// 记录形式: compiled_procedure; instances=1
// 源码行: 39
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":430,"record_offset":27904,"rva":6035392,"va":5374744512}
// 编译实例 RVAs: 0x5c17c0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dccd0
//     owner=unresolved_namespace; count=1
//     target_aliases=9 candidates; complete list: vao_update.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5131189","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374744512,"raw_hex":"e84bb4b1ff","size":5,"soft_fail":false,"target_va":5369613520,"va":5374744704,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: vao_update.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1995511","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374744512,"raw_hex":"e8098de1ff","size":5,"soft_fail":false,"target_va":5372749712,"va":5374745218,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-3439874","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374744512,"raw_hex":"e8fe82cbff","size":5,"soft_fail":false,"target_va":5371305056,"va":5374744925,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..ops..drop..Drop$u20$for$u20$anyhow..Error$GT$::drop
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t2619671","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374744512,"raw_hex":"e817f92700","size":5,"soft_fail":false,"target_va":5377364336,"va":5374744660,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::asset_libraries::vao_library::AssetVAOLibrary::rebuild_vao
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-86562","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374744512,"raw_hex":"e8deadfeff","size":5,"soft_fail":false,"target_va":5374658336,"va":5374744893,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] log::__private_api::log_impl
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t2608712","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374744512,"raw_hex":"e848ce2700","size":5,"soft_fail":false,"target_va":5377353776,"va":5374745059,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":430,"record_offset":28096,"parent_record_offset":27904,"ancestor_record_offset":27904,"inlinee":7328,"source_line":2218,"archive":"_pdb/A/modules/00430.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":430,"record_offset":28124,"parent_record_offset":27904,"ancestor_record_offset":27904,"inlinee":4175,"source_line":422,"archive":"_pdb/A/modules/00430.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":430,"record_offset":29496,"parent_record_offset":27904,"ancestor_record_offset":27904,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/00430.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$16::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":430,"record_offset":28152,"parent_record_offset":27904,"ancestor_record_offset":27904,"inlinee":5021,"source_line":2942,"archive":"_pdb/A/modules/00430.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":430,"record_offset":29384,"parent_record_offset":27904,"ancestor_record_offset":27904,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/00430.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":430,"record_offset":29700,"parent_record_offset":27904,"ancestor_record_offset":27904,"inlinee":4715,"source_line":1188,"archive":"_pdb/A/modules/00430.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":430,"record_offset":29896,"parent_record_offset":27904,"ancestor_record_offset":27904,"inlinee":4716,"source_line":329,"archive":"_pdb/A/modules/00430.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":430,"record_offset":29728,"parent_record_offset":27904,"ancestor_record_offset":27904,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/00430.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":430,"record_offset":28264,"parent_record_offset":27904,"ancestor_record_offset":27904,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00430.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":430,"record_offset":28292,"parent_record_offset":27904,"ancestor_record_offset":27904,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/00430.jsonl.gz"}
//   [pdb_inline_direct] country_core::asset_libraries::vao_library::AssetVAOLibrary::has_vao
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/vao_library.rs
//     evidence={"module":430,"record_offset":28416,"parent_record_offset":27904,"ancestor_record_offset":27904,"inlinee":24609,"source_line":51,"archive":"_pdb/A/modules/00430.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::impl$0::deref
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/mod.rs
//     evidence={"module":430,"record_offset":29440,"parent_record_offset":27904,"ancestor_record_offset":27904,"inlinee":22680,"source_line":43,"archive":"_pdb/A/modules/00430.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/__private_api.rs
//     evidence={"module":430,"record_offset":29756,"parent_record_offset":27904,"ancestor_record_offset":27904,"inlinee":4718,"source_line":64,"archive":"_pdb/A/modules/00430.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/lib.rs
//     evidence={"module":430,"record_offset":29616,"parent_record_offset":27904,"ancestor_record_offset":27904,"inlinee":4713,"source_line":1269,"archive":"_pdb/A/modules/00430.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":430,"record_offset":30184,"parent_record_offset":30160,"ancestor_record_offset":27904,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00430.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":430,"record_offset":30160,"parent_record_offset":30060,"ancestor_record_offset":27904,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00430.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":430,"record_offset":30092,"parent_record_offset":30060,"ancestor_record_offset":27904,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00430.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":430,"record_offset":29552,"parent_record_offset":29528,"ancestor_record_offset":27904,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/00430.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":430,"record_offset":30060,"parent_record_offset":30032,"ancestor_record_offset":27904,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00430.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":430,"record_offset":29464,"parent_record_offset":29440,"ancestor_record_offset":27904,"inlinee":4586,"source_line":2105,"archive":"_pdb/A/modules/00430.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":430,"record_offset":29528,"parent_record_offset":29496,"ancestor_record_offset":27904,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/00430.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$29::set_changed
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":430,"record_offset":29408,"parent_record_offset":29384,"ancestor_record_offset":27904,"inlinee":5812,"source_line":309,"archive":"_pdb/A/modules/00430.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":430,"record_offset":29136,"parent_record_offset":29112,"ancestor_record_offset":27904,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00430.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":430,"record_offset":28688,"parent_record_offset":28664,"ancestor_record_offset":27904,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/00430.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":430,"record_offset":28772,"parent_record_offset":28748,"ancestor_record_offset":27904,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/00430.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":430,"record_offset":28712,"parent_record_offset":28688,"ancestor_record_offset":27904,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00430.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":430,"record_offset":30124,"parent_record_offset":30092,"ancestor_record_offset":27904,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/00430.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":430,"record_offset":28884,"parent_record_offset":28860,"ancestor_record_offset":27904,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/00430.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":430,"record_offset":30032,"parent_record_offset":30008,"ancestor_record_offset":27904,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00430.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":430,"record_offset":28224,"parent_record_offset":28200,"ancestor_record_offset":27904,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/00430.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":430,"record_offset":29048,"parent_record_offset":29024,"ancestor_record_offset":27904,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/00430.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":430,"record_offset":29024,"parent_record_offset":29000,"ancestor_record_offset":27904,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/00430.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":430,"record_offset":28380,"parent_record_offset":28356,"ancestor_record_offset":27904,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/00430.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":430,"record_offset":28328,"parent_record_offset":28292,"ancestor_record_offset":27904,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/00430.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":430,"record_offset":29584,"parent_record_offset":29496,"ancestor_record_offset":27904,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00430.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":430,"record_offset":28176,"parent_record_offset":28152,"ancestor_record_offset":27904,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/00430.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":430,"record_offset":28200,"parent_record_offset":28176,"ancestor_record_offset":27904,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/00430.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":430,"record_offset":28356,"parent_record_offset":28292,"ancestor_record_offset":27904,"inlinee":5091,"source_line":101,"archive":"_pdb/A/modules/00430.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":430,"record_offset":29640,"parent_record_offset":29616,"ancestor_record_offset":27904,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/00430.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":430,"record_offset":29664,"parent_record_offset":29640,"ancestor_record_offset":27904,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/00430.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::impl$16::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/mod.rs
//     evidence={"module":430,"record_offset":29184,"parent_record_offset":29160,"ancestor_record_offset":27904,"inlinee":22473,"source_line":47,"archive":"_pdb/A/modules/00430.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::impl$7::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/mod.rs
//     evidence={"module":430,"record_offset":29160,"parent_record_offset":29136,"ancestor_record_offset":27904,"inlinee":22471,"source_line":103,"archive":"_pdb/A/modules/00430.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\lib.rs
//     evidence={"module":430,"record_offset":29112,"parent_record_offset":29088,"ancestor_record_offset":27904,"inlinee":4854,"source_line":170,"archive":"_pdb/A/modules/00430.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::contains_key
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":430,"record_offset":28464,"parent_record_offset":28440,"ancestor_record_offset":27904,"inlinee":8704,"source_line":1418,"archive":"_pdb/A/modules/00430.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":430,"record_offset":28488,"parent_record_offset":28464,"ancestor_record_offset":27904,"inlinee":5562,"source_line":1351,"archive":"_pdb/A/modules/00430.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":430,"record_offset":29088,"parent_record_offset":28952,"ancestor_record_offset":27904,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/00430.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":430,"record_offset":28516,"parent_record_offset":28488,"ancestor_record_offset":27904,"inlinee":5617,"source_line":257,"archive":"_pdb/A/modules/00430.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":430,"record_offset":29000,"parent_record_offset":28976,"ancestor_record_offset":27904,"inlinee":5640,"source_line":513,"archive":"_pdb/A/modules/00430.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":430,"record_offset":28976,"parent_record_offset":28952,"ancestor_record_offset":27904,"inlinee":5639,"source_line":682,"archive":"_pdb/A/modules/00430.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":430,"record_offset":29324,"parent_record_offset":28592,"ancestor_record_offset":27904,"inlinee":5645,"source_line":175,"archive":"_pdb/A/modules/00430.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":430,"record_offset":28568,"parent_record_offset":28544,"ancestor_record_offset":27904,"inlinee":5620,"source_line":1456,"archive":"_pdb/A/modules/00430.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":430,"record_offset":28544,"parent_record_offset":28488,"ancestor_record_offset":27904,"inlinee":7330,"source_line":1479,"archive":"_pdb/A/modules/00430.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":430,"record_offset":28592,"parent_record_offset":28568,"ancestor_record_offset":27904,"inlinee":5622,"source_line":2168,"archive":"_pdb/A/modules/00430.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":430,"record_offset":28832,"parent_record_offset":28804,"ancestor_record_offset":27904,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/00430.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":430,"record_offset":28860,"parent_record_offset":28832,"ancestor_record_offset":27904,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/00430.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::remove_lowest_bit
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":430,"record_offset":28920,"parent_record_offset":28804,"ancestor_record_offset":27904,"inlinee":5633,"source_line":37,"archive":"_pdb/A/modules/00430.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":430,"record_offset":28804,"parent_record_offset":28592,"ancestor_record_offset":27904,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/00430.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::h2
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":430,"record_offset":28636,"parent_record_offset":28592,"ancestor_record_offset":27904,"inlinee":5624,"source_line":150,"archive":"_pdb/A/modules/00430.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":430,"record_offset":28952,"parent_record_offset":28592,"ancestor_record_offset":27904,"inlinee":5635,"source_line":1466,"archive":"_pdb/A/modules/00430.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":430,"record_offset":28664,"parent_record_offset":28592,"ancestor_record_offset":27904,"inlinee":5626,"source_line":52,"archive":"_pdb/A/modules/00430.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_byte
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":430,"record_offset":28748,"parent_record_offset":28592,"ancestor_record_offset":27904,"inlinee":5627,"source_line":79,"archive":"_pdb/A/modules/00430.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":430,"record_offset":29232,"parent_record_offset":28592,"ancestor_record_offset":27904,"inlinee":5643,"source_line":97,"archive":"_pdb/A/modules/00430.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::contains_key
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":430,"record_offset":28440,"parent_record_offset":28416,"ancestor_record_offset":27904,"inlinee":17300,"source_line":1042,"archive":"_pdb/A/modules/00430.jsonl.gz"}
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
// 完整结构及原始类型记录: vao_update.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: vao_update.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: vao_update.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 2; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh> > >,bevy_ecs::query::filter::Without<country_core::asset_libraries::Handle<country_core::render::vao::Vao> > >,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::vao_library::AssetVAOLibrary>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh> > >,bevy_ecs::query::filter::Without<country_core::asset_libraries::Handle<country_core::render::vao::Vao> > >", "bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>", "bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::ResMut<country_core::asset_libraries::vao_library::AssetVAOLibrary>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] query_without_filter: bevy_ecs::query::filter::Without<country_core::asset_libraries::Handle<country_core::render::vao::Vao> >
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh> > >,bevy_ecs::query::filter::Without<country_core::asset_libraries::Handle<country_core::render::vao::Vao> > >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::vao_library::AssetVAOLibrary>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::vao_library::AssetVAOLibrary>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>", "bevy_ecs::change_detection::ResMut<country_core::asset_libraries::vao_library::AssetVAOLibrary>", "bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::vao_library::AssetVAOLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>

// ------------------------------------------------------------------
// 函数: country_core::systems::backend::vao_update::build_missing_vaos
// 信息 ID: 815ccba9af541df50f4c3f80
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] dfcc0d5189e464f5542538b1: void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh> > >,bevy_ecs::query::filter::Without<country_core::asset_libraries::Handle<country_core::render::vao::Vao> > >,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::vao_library::AssetVAOLibrary>,bevy_ecs::system::commands::Commands)
// 引用方: 原程序直接转移 4 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh> > >,bevy_ecs::query::filter::Without<country_core::asset_libraries::Handle<country_core::render::vao::Vao> > >,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::vao_library::AssetVAOLibrary>,bevy_ecs::system::commands::Commands),void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh> > >,bevy_ecs::query::filter::Without<country_core::asset_libraries::Handle<country_core::render::vao::Vao> > >,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::vao_library::AssetVAOLibrary>,bevy_ecs::system::commands::Commands)> > @ 0xa98d5 -> 0x5c13f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh> > >,bevy_ecs::query::filter::Without<country_core::asset_libraries::Handle<country_core::render::vao::Vao> > >,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::vao_library::AssetVAOLibrary>,bevy_ecs::system::commands::Commands),void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh> > >,bevy_ecs::query::filter::Without<country_core::asset_libraries::Handle<country_core::render::vao::Vao> > >,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::vao_library::AssetVAOLibrary>,bevy_ecs::system::commands::Commands)> @ 0x334df3 -> 0x5c13f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh> > >,bevy_ecs::query::filter::Without<country_core::asset_libraries::Handle<country_core::render::vao::Vao> > >,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::vao_library::AssetVAOLibrary>,bevy_ecs::system::commands::Commands),void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh> > >,bevy_ecs::query::filter::Without<country_core::asset_libraries::Handle<country_core::render::vao::Vao> > >,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::vao_library::AssetVAOLibrary>,bevy_ecs::system::commands::Commands)> @ 0xdeacb3 -> 0x5c13f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh> > >,bevy_ecs::query::filter::Without<country_core::asset_libraries::Handle<country_core::render::vao::Vao> > >,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::vao_library::AssetVAOLibrary>,bevy_ecs::system::commands::Commands),void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh> > >,bevy_ecs::query::filter::Without<country_core::asset_libraries::Handle<country_core::render::vao::Vao> > >,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::vao_library::AssetVAOLibrary>,bevy_ecs::system::commands::Commands)> > @ 0xe609f5 -> 0x5c13f0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::backend::vao_update::build_missing_vaos::closure$0
// 信息 ID: 8694bc538e66e56908b11cb7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::backend::vao_update::build_missing_vaos", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::backend::vao_update::build_missing_vaos; module=430, inline_record=26304; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::backend::vao_update::rebuild_vaos
// 信息 ID: 82662a6a930ac6f8539b6e4a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 999974cea127b5d5b0295d62: void (*)(bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::vao_library::AssetVAOLibrary>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>)
// 引用方: 原程序直接转移 4 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::vao_library::AssetVAOLibrary>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>),void (*)(bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::vao_library::AssetVAOLibrary>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>)> > @ 0xc6790 -> 0x5c17c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::vao_library::AssetVAOLibrary>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>),void (*)(bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::vao_library::AssetVAOLibrary>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>)> @ 0x334a25 -> 0x5c17c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::vao_library::AssetVAOLibrary>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>),void (*)(bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::vao_library::AssetVAOLibrary>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>)> @ 0xdebe05 -> 0x5c17c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::vao_library::AssetVAOLibrary>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>),void (*)(bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::vao_library::AssetVAOLibrary>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>)> > @ 0xe50180 -> 0x5c17c0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
