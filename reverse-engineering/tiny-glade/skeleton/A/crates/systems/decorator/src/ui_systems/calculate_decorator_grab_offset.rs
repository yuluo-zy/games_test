// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/systems/decorator/src/ui_systems/calculate_decorator_grab_offset.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 45937db87e38f850c4dede42a4c43ebffa316382f23cc6a4b0b03a4c24f5948b
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: calculate_decorator_grab_offset.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 4377
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use bevy_ecs; // namespace candidate
//   package identities: []
// use bevy_ptr; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: []
// use fixedbitset; // namespace candidate
//   package identities: [{"name":"fixedbitset","source":"PDB_registry_path_only","version":"0.5.7"}]
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"PDB_registry_path_only","version":"0.27.0"}]
// use nonmax; // namespace candidate
//   package identities: [{"name":"nonmax","source":"PDB_registry_path_only","version":"0.5.5"}]
// use system_decorator; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_decorator::ui_systems::calculate_decorator_grab_offset::try_get_grab_offset_ss_if_valid
// 记录形式: compiled_procedure; instances=1
// 源码行: 26
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":4377,"record_offset":500,"rva":31876688,"va":5400585808}
// 编译实例 RVAs: 0x1e66650
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t2392084","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400585808,"raw_hex":"e814802400","size":5,"soft_fail":false,"target_va":5402978880,"va":5400586791,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t2393279","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400585808,"raw_hex":"e8bf842400","size":5,"soft_fail":false,"target_va":5402980128,"va":5400586844,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator_storage::DecoratorStorageAddr::as_wall
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t-25869429","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400585808,"raw_hex":"e88b4375fe","size":5,"soft_fail":false,"target_va":5374716704,"va":5400586128,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_walls::PublicWalls::get
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-25151292","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400585808,"raw_hex":"e8c43880fe","size":5,"soft_fail":false,"target_va":5375434976,"va":5400586263,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_walls::WallEntities::wall_entity
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-25151576","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400585808,"raw_hex":"e8a83780fe","size":5,"soft_fail":false,"target_va":5375434752,"va":5400586323,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::wall_trigger_volume::WallTriggerVolume::get_position_from_mesh_uv
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-18375074","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400585808,"raw_hex":"e85e9ee7fe","size":5,"soft_fail":false,"target_va":5382211456,"va":5400586525,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::window::WindowSize::as_vec2
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t-25173739","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400585808,"raw_hex":"e815e17ffe","size":5,"soft_fail":false,"target_va":5375412208,"va":5400585942,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::utils::world_space_to_screen_space
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-25224642","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400585808,"raw_hex":"e83e1a7ffe","size":5,"soft_fail":false,"target_va":5375361328,"va":5400585965,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_decorator::ui_systems::calculate_decorator_grab_offset::raycast_with_ss_offset
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t820","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400585808,"raw_hex":"e834030000","size":5,"soft_fail":false,"target_va":5400586864,"va":5400586039,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::wall_space::WallSpaceCoord::to_world_space
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-18330006","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400585808,"raw_hex":"e86a4ee8fe","size":5,"soft_fail":false,"target_va":5382256592,"va":5400586593,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::system::query::Query::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/query.rs
//     evidence={"module":4377,"record_offset":888,"parent_record_offset":500,"ancestor_record_offset":500,"inlinee":21384,"source_line":846,"archive":"_pdb/A/modules/04377.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":4377,"record_offset":788,"parent_record_offset":500,"ancestor_record_offset":500,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/04377.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":4377,"record_offset":1840,"parent_record_offset":500,"ancestor_record_offset":500,"inlinee":4138,"source_line":972,"archive":"_pdb/A/modules/04377.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":4377,"record_offset":2648,"parent_record_offset":500,"ancestor_record_offset":500,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/04377.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::impl$9::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":4377,"record_offset":760,"parent_record_offset":500,"ancestor_record_offset":500,"inlinee":8043,"source_line":65,"archive":"_pdb/A/modules/04377.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::public_wall_state::PublicWallState::is_premade_shape
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_wall_state.rs
//     evidence={"module":4377,"record_offset":824,"parent_record_offset":500,"ancestor_record_offset":500,"inlinee":21375,"source_line":96,"archive":"_pdb/A/modules/04377.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$17::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":4377,"record_offset":704,"parent_record_offset":500,"ancestor_record_offset":500,"inlinee":14656,"source_line":1051,"archive":"_pdb/A/modules/04377.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::impl$24::eq
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/archetype.rs
//     evidence={"module":4377,"record_offset":1104,"parent_record_offset":960,"ancestor_record_offset":500,"inlinee":21390,"source_line":74,"archive":"_pdb/A/modules/04377.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::Entities::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/entity/mod.rs
//     evidence={"module":4377,"record_offset":960,"parent_record_offset":916,"ancestor_record_offset":500,"inlinee":21387,"source_line":739,"archive":"_pdb/A/modules/04377.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$21::fetch
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":4377,"record_offset":1748,"parent_record_offset":916,"ancestor_record_offset":500,"inlinee":14627,"source_line":963,"archive":"_pdb/A/modules/04377.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$21::set_archetype
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":4377,"record_offset":1380,"parent_record_offset":916,"ancestor_record_offset":500,"inlinee":21391,"source_line":933,"archive":"_pdb/A/modules/04377.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$21::set_table
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":4377,"record_offset":1404,"parent_record_offset":1380,"ancestor_record_offset":500,"inlinee":14620,"source_line":948,"archive":"_pdb/A/modules/04377.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::state::QueryState::get_unchecked_manual
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/state.rs
//     evidence={"module":4377,"record_offset":916,"parent_record_offset":888,"ancestor_record_offset":500,"inlinee":21385,"source_line":859,"archive":"_pdb/A/modules/04377.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_slice
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":4377,"record_offset":1708,"parent_record_offset":1684,"ancestor_record_offset":500,"inlinee":14643,"source_line":410,"archive":"_pdb/A/modules/04377.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseArray::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":4377,"record_offset":1480,"parent_record_offset":1456,"ancestor_record_offset":500,"inlinee":6338,"source_line":56,"archive":"_pdb/A/modules/04377.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":4377,"record_offset":1456,"parent_record_offset":1432,"ancestor_record_offset":500,"inlinee":6337,"source_line":369,"archive":"_pdb/A/modules/04377.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$11::get::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":4377,"record_offset":1528,"parent_record_offset":1504,"ancestor_record_offset":500,"inlinee":6340,"source_line":58,"archive":"_pdb/A/modules/04377.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$16::get::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":4377,"record_offset":1616,"parent_record_offset":1592,"ancestor_record_offset":500,"inlinee":6342,"source_line":370,"archive":"_pdb/A/modules/04377.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Column::get_data_slice
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":4377,"record_offset":1684,"parent_record_offset":1404,"ancestor_record_offset":500,"inlinee":14642,"source_line":321,"archive":"_pdb/A/modules/04377.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_column
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":4377,"record_offset":1432,"parent_record_offset":1404,"ancestor_record_offset":500,"inlinee":14621,"source_line":692,"archive":"_pdb/A/modules/04377.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":4377,"record_offset":1352,"parent_record_offset":916,"ancestor_record_offset":500,"inlinee":14618,"source_line":834,"archive":"_pdb/A/modules/04377.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ptr::ThinSlicePtr::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ptr/src/lib.rs
//     evidence={"module":4377,"record_offset":1772,"parent_record_offset":1748,"ancestor_record_offset":500,"inlinee":21393,"source_line":500,"archive":"_pdb/A/modules/04377.jsonl.gz"}
//   [pdb_inline_transitive] core::bool::impl$0::then
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\bool.rs
//     evidence={"module":4377,"record_offset":1168,"parent_record_offset":1136,"ancestor_record_offset":500,"inlinee":4886,"source_line":59,"archive":"_pdb/A/modules/04377.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$24::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":4377,"record_offset":1072,"parent_record_offset":1048,"ancestor_record_offset":500,"inlinee":13915,"source_line":1471,"archive":"_pdb/A/modules/04377.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":4377,"record_offset":2344,"parent_record_offset":1928,"ancestor_record_offset":500,"inlinee":15697,"source_line":31,"archive":"_pdb/A/modules/04377.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ss
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":4377,"record_offset":2252,"parent_record_offset":2196,"ancestor_record_offset":500,"inlinee":21747,"source_line":20,"archive":"_pdb/A/modules/04377.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":4377,"record_offset":2024,"parent_record_offset":2000,"ancestor_record_offset":500,"inlinee":14684,"source_line":77,"archive":"_pdb/A/modules/04377.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":4377,"record_offset":2056,"parent_record_offset":1972,"ancestor_record_offset":500,"inlinee":14683,"source_line":1011,"archive":"_pdb/A/modules/04377.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_sub_ps
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":4377,"record_offset":2116,"parent_record_offset":1928,"ancestor_record_offset":500,"inlinee":14685,"source_line":54,"archive":"_pdb/A/modules/04377.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::impl$2::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":4377,"record_offset":1048,"parent_record_offset":960,"ancestor_record_offset":500,"inlinee":21388,"source_line":136,"archive":"_pdb/A/modules/04377.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":4377,"record_offset":2580,"parent_record_offset":2552,"ancestor_record_offset":500,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/04377.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":4377,"record_offset":728,"parent_record_offset":704,"ancestor_record_offset":500,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/04377.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":4377,"record_offset":1552,"parent_record_offset":1528,"ancestor_record_offset":500,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/04377.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":4377,"record_offset":1504,"parent_record_offset":1480,"ancestor_record_offset":500,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/04377.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":4377,"record_offset":1276,"parent_record_offset":1244,"ancestor_record_offset":500,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/04377.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":4377,"record_offset":1324,"parent_record_offset":916,"ancestor_record_offset":500,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/04377.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":4377,"record_offset":992,"parent_record_offset":960,"ancestor_record_offset":500,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/04377.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":4377,"record_offset":1016,"parent_record_offset":992,"ancestor_record_offset":500,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/04377.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::contains
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":4377,"record_offset":1136,"parent_record_offset":916,"ancestor_record_offset":500,"inlinee":4884,"source_line":392,"archive":"_pdb/A/modules/04377.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::contains_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":4377,"record_offset":1220,"parent_record_offset":1196,"ancestor_record_offset":500,"inlinee":4889,"source_line":408,"archive":"_pdb/A/modules/04377.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::get_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":4377,"record_offset":1244,"parent_record_offset":1220,"ancestor_record_offset":500,"inlinee":4890,"source_line":171,"archive":"_pdb/A/modules/04377.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::impl$2::contains::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":4377,"record_offset":1196,"parent_record_offset":1168,"ancestor_record_offset":500,"inlinee":4888,"source_line":395,"archive":"_pdb/A/modules/04377.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::mul_vec3
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/quat.rs
//     evidence={"module":4377,"record_offset":1904,"parent_record_offset":1872,"ancestor_record_offset":500,"inlinee":14676,"source_line":659,"archive":"_pdb/A/modules/04377.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::mul_vec3a
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/quat.rs
//     evidence={"module":4377,"record_offset":1928,"parent_record_offset":1904,"ancestor_record_offset":500,"inlinee":14681,"source_line":731,"archive":"_pdb/A/modules/04377.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::cross
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec3a.rs
//     evidence={"module":4377,"record_offset":2372,"parent_record_offset":1928,"ancestor_record_offset":500,"inlinee":14682,"source_line":220,"archive":"_pdb/A/modules/04377.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$12::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":4377,"record_offset":2552,"parent_record_offset":1872,"ancestor_record_offset":500,"inlinee":14692,"source_line":1081,"archive":"_pdb/A/modules/04377.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_in_x
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/sse2.rs
//     evidence={"module":4377,"record_offset":2000,"parent_record_offset":1972,"ancestor_record_offset":500,"inlinee":21746,"source_line":42,"archive":"_pdb/A/modules/04377.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_into_m128
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/sse2.rs
//     evidence={"module":4377,"record_offset":1972,"parent_record_offset":1928,"ancestor_record_offset":500,"inlinee":21745,"source_line":66,"archive":"_pdb/A/modules/04377.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nonmax","version":"0.5.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":4377,"record_offset":1640,"parent_record_offset":1616,"ancestor_record_offset":500,"inlinee":4372,"source_line":153,"archive":"_pdb/A/modules/04377.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::ui_systems::calculate_decorator_grab_offset::try_get_grab_offset_ss_if_valid::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/ui_systems/calculate_decorator_grab_offset.rs
//     evidence={"module":4377,"record_offset":1872,"parent_record_offset":1840,"ancestor_record_offset":500,"inlinee":120851,"source_line":51,"archive":"_pdb/A/modules/04377.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::ui_systems::calculate_decorator_grab_offset::try_get_grab_offset_ss_if_valid::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 51
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1D813
// 首条位置: {"module":4377,"inlinee":120851}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::sse2::quat::Quat::mul_vec3
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/quat.rs
//     evidence={"module":4377,"record_offset":1904,"parent_record_offset":1872,"ancestor_record_offset":1872,"inlinee":14676,"source_line":659,"archive":"_pdb/A/modules/04377.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$12::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":4377,"record_offset":2552,"parent_record_offset":1872,"ancestor_record_offset":1872,"inlinee":14692,"source_line":1081,"archive":"_pdb/A/modules/04377.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":4377,"record_offset":2344,"parent_record_offset":1928,"ancestor_record_offset":1872,"inlinee":15697,"source_line":31,"archive":"_pdb/A/modules/04377.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ss
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":4377,"record_offset":2252,"parent_record_offset":2196,"ancestor_record_offset":1872,"inlinee":21747,"source_line":20,"archive":"_pdb/A/modules/04377.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":4377,"record_offset":2024,"parent_record_offset":2000,"ancestor_record_offset":1872,"inlinee":14684,"source_line":77,"archive":"_pdb/A/modules/04377.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":4377,"record_offset":2056,"parent_record_offset":1972,"ancestor_record_offset":1872,"inlinee":14683,"source_line":1011,"archive":"_pdb/A/modules/04377.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_sub_ps
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":4377,"record_offset":2116,"parent_record_offset":1928,"ancestor_record_offset":1872,"inlinee":14685,"source_line":54,"archive":"_pdb/A/modules/04377.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":4377,"record_offset":2580,"parent_record_offset":2552,"ancestor_record_offset":1872,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/04377.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::mul_vec3a
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/quat.rs
//     evidence={"module":4377,"record_offset":1928,"parent_record_offset":1904,"ancestor_record_offset":1872,"inlinee":14681,"source_line":731,"archive":"_pdb/A/modules/04377.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::cross
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec3a.rs
//     evidence={"module":4377,"record_offset":2372,"parent_record_offset":1928,"ancestor_record_offset":1872,"inlinee":14682,"source_line":220,"archive":"_pdb/A/modules/04377.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_in_x
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/sse2.rs
//     evidence={"module":4377,"record_offset":2000,"parent_record_offset":1972,"ancestor_record_offset":1872,"inlinee":21746,"source_line":42,"archive":"_pdb/A/modules/04377.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_into_m128
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/sse2.rs
//     evidence={"module":4377,"record_offset":1972,"parent_record_offset":1928,"ancestor_record_offset":1872,"inlinee":21745,"source_line":66,"archive":"_pdb/A/modules/04377.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::ui_systems::calculate_decorator_grab_offset::raycast_with_ss_offset
// 记录形式: compiled_procedure; instances=1
// 源码行: 152
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":4377,"record_offset":2680,"rva":31877744,"va":5400586864}
// 编译实例 RVAs: 0x1e66a70
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] country_core::resources::terrain_height::TerrainHeights::intersect_ray
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-18716105","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400586864,"raw_hex":"e8376ae2fe","size":5,"soft_fail":false,"target_va":5381870928,"va":5400587028,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::window::WindowSize::as_vec2
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-25174741","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400586864,"raw_hex":"e82bdd7ffe","size":5,"soft_fail":false,"target_va":5375412208,"va":5400586944,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::collision::world::RaycastWorld::legacy_raycast
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-26078094","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400586864,"raw_hex":"e8721472fe","size":5,"soft_fail":false,"target_va":5374508912,"va":5400587001,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::ray::Ray::from_mouse_ss
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-17426151","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400586864,"raw_hex":"e81919f6fe","size":5,"soft_fail":false,"target_va":5383160832,"va":5400586978,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":4377,"record_offset":2944,"parent_record_offset":2680,"ancestor_record_offset":2680,"inlinee":4099,"source_line":1172,"archive":"_pdb/A/modules/04377.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":4377,"record_offset":3192,"parent_record_offset":2680,"ancestor_record_offset":2680,"inlinee":4779,"source_line":953,"archive":"_pdb/A/modules/04377.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$12::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":4377,"record_offset":2860,"parent_record_offset":2680,"ancestor_record_offset":2680,"inlinee":17198,"source_line":1002,"archive":"_pdb/A/modules/04377.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":4377,"record_offset":3032,"parent_record_offset":3004,"ancestor_record_offset":2680,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/04377.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":4377,"record_offset":2884,"parent_record_offset":2860,"ancestor_record_offset":2680,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/04377.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$12::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":4377,"record_offset":3096,"parent_record_offset":2972,"ancestor_record_offset":2680,"inlinee":14692,"source_line":1081,"archive":"_pdb/A/modules/04377.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$9::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":4377,"record_offset":3004,"parent_record_offset":2972,"ancestor_record_offset":2680,"inlinee":14672,"source_line":1048,"archive":"_pdb/A/modules/04377.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::ui_systems::calculate_decorator_grab_offset::raycast_with_ss_offset::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/ui_systems/calculate_decorator_grab_offset.rs
//     evidence={"module":4377,"record_offset":2972,"parent_record_offset":2944,"ancestor_record_offset":2680,"inlinee":120853,"source_line":173,"archive":"_pdb/A/modules/04377.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::ui_systems::calculate_decorator_grab_offset::raycast_with_ss_offset::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 173
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1D815
// 首条位置: {"module":4377,"inlinee":120853}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec3::impl$12::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":4377,"record_offset":3096,"parent_record_offset":2972,"ancestor_record_offset":2972,"inlinee":14692,"source_line":1081,"archive":"_pdb/A/modules/04377.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$9::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":4377,"record_offset":3004,"parent_record_offset":2972,"ancestor_record_offset":2972,"inlinee":14672,"source_line":1048,"archive":"_pdb/A/modules/04377.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":4377,"record_offset":3032,"parent_record_offset":3004,"ancestor_record_offset":2972,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/04377.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":4377,"record_offset":3124,"parent_record_offset":3096,"ancestor_record_offset":2972,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/04377.jsonl.gz"}
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
// 完整结构及原始类型记录: calculate_decorator_grab_offset.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: calculate_decorator_grab_offset.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: calculate_decorator_grab_offset.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: system_decorator::ui_systems::calculate_decorator_grab_offset::raycast_with_ss_offset
// 信息 ID: 131b842e794739ab9aa3c27e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::ui_systems::ui_move_decorator::ui_move_decorator @ 0x1c77795 -> 0x1e66a70 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::calculate_decorator_grab_offset::try_get_grab_offset_ss_if_valid @ 0x1e66737 -> 0x1e66a70 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::ui_systems::calculate_decorator_grab_offset::raycast_with_ss_offset::closure$0
// 信息 ID: f21dce2be3e724304b492471
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_decorator::ui_systems::calculate_decorator_grab_offset::raycast_with_ss_offset", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::ui_systems::calculate_decorator_grab_offset::raycast_with_ss_offset; module=4377, inline_record=2972; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::ui_systems::calculate_decorator_grab_offset::try_get_grab_offset_ss_if_valid
// 信息 ID: 7c130b9c2d13ed58fb86d1df
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::ui_systems::ui_move_decorator::ui_move_decorator @ 0x1c76875 -> 0x1e66650 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::ui_move_decorator::ui_move_decorator @ 0x1c77686 -> 0x1e66650 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::ui_systems::calculate_decorator_grab_offset::try_get_grab_offset_ss_if_valid::closure$0
// 信息 ID: 18392221fd4dfa8e721c68d8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_decorator::ui_systems::calculate_decorator_grab_offset::try_get_grab_offset_ss_if_valid", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::ui_systems::calculate_decorator_grab_offset::try_get_grab_offset_ss_if_valid; module=4377, inline_record=1872; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
