// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/systems/decorator/src/ui_systems/handle_splitting_roof_decorators.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: baa502012795025423f499a23b94b9a0b9d6c868aad212d6a7b5bffd5a834d3d
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: handle_splitting_roof_decorators.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 4378
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
// use country_core; // namespace candidate
//   package identities: []
// use fixedbitset; // namespace candidate
//   package identities: [{"name":"fixedbitset","source":"PDB_registry_path_only","version":"0.5.7"}]
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"PDB_registry_path_only","version":"0.27.0"}]
// use nonmax; // namespace candidate
//   package identities: [{"name":"nonmax","source":"PDB_registry_path_only","version":"0.5.5"}]
// use smallvec; // namespace candidate
//   package identities: [{"name":"smallvec","source":"PDB_registry_path_only","version":"1.13.2"}]
// use std; // namespace candidate
//   package identities: []
// use system_decorator; // namespace candidate
//   package identities: []
// use utils; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_decorator::ui_systems::handle_splitting_roof_decorators::try_display_roof_decorator_parts_and_get_hovered_part_id
// 记录形式: compiled_procedure; instances=1
// 源码行: 25
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":4378,"record_offset":1932,"rva":31878224,"va":5400587344}
// 编译实例 RVAs: 0x1e66c50
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: handle_splitting_roof_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-27839291","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400587344,"raw_hex":"e8c53457fe","size":5,"soft_fail":false,"target_va":5372749696,"va":5400588982,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=6
//     target_aliases=1 candidates; complete list: handle_splitting_roof_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-27839454","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400587344,"raw_hex":"e8223457fe","size":5,"soft_fail":false,"target_va":5372749712,"va":5400589161,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406bb180
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: handle_splitting_roof_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-24821540","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400587344,"raw_hex":"e8dc4085fe","size":5,"soft_fail":false,"target_va":5375766912,"va":5400588447,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t2388032","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400587344,"raw_hex":"e840702400","size":5,"soft_fail":false,"target_va":5402978144,"va":5400590107,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t-23059058","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400587344,"raw_hex":"e88e25a0fe","size":5,"soft_fail":false,"target_va":5377529888,"va":5400588941,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t2389055","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400587344,"raw_hex":"e83f742400","size":5,"soft_fail":false,"target_va":5402979152,"va":5400590092,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume::get_position_from_mesh_uv
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-26527722","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400587344,"raw_hex":"e816386bfe","size":5,"soft_fail":false,"target_va":5374060928,"va":5400588645,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator::DecoratorInfo::dst_as_roof
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-26017469","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400587344,"raw_hex":"e8430173fe","size":5,"soft_fail":false,"target_va":5374571120,"va":5400588584,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator_storage::DecoratorStorageAddr::as_wall
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-25870787","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400587344,"raw_hex":"e83d3e75fe","size":5,"soft_fail":false,"target_va":5374716704,"va":5400587486,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_walls::PublicWalls::roof_entity
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-25151470","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400587344,"raw_hex":"e8123880fe","size":5,"soft_fail":false,"target_va":5375436032,"va":5400587497,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierFactory::new
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t-25233327","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400587344,"raw_hex":"e851f87efe","size":5,"soft_fail":false,"target_va":5375355744,"va":5400589066,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::signifier_create::UiSignifierStream::send
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t-18315497","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400587344,"raw_hex":"e81787e8fe","size":5,"soft_fail":false,"target_va":5382273632,"va":5400589124,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::wall::valid_enclosure::ShapeParameters::as_rectangle
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-24863468","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400587344,"raw_hex":"e8149d84fe","size":5,"soft_fail":false,"target_va":5375724992,"va":5400588455,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::wall::valid_enclosure::ShapeParameters::perimeter
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-24863804","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400587344,"raw_hex":"e8c49b84fe","size":5,"soft_fail":false,"target_va":5375724768,"va":5400588567,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::edge_length
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-22621178","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400587344,"raw_hex":"e806d4a6fe","size":5,"soft_fail":false,"target_va":5377967328,"va":5400588501,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::get_closest_edge_id
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-22627359","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400587344,"raw_hex":"e8e1bba6fe","size":5,"soft_fail":false,"target_va":5377961136,"va":5400588490,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::system::query::Query::get
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/query.rs
//     evidence={"module":4378,"record_offset":2140,"parent_record_offset":1932,"ancestor_record_offset":1932,"inlinee":21384,"source_line":846,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::ui_systems::handle_splitting_roof_decorators::display_roof_decorator_parts_and_get_hovered_part_id
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/ui_systems/handle_splitting_roof_decorators.rs
//     evidence={"module":4378,"record_offset":3516,"parent_record_offset":1932,"ancestor_record_offset":1932,"inlinee":120860,"source_line":64,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":4378,"record_offset":5852,"parent_record_offset":5828,"ancestor_record_offset":1932,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":4378,"record_offset":5876,"parent_record_offset":5852,"ancestor_record_offset":1932,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":4378,"record_offset":6196,"parent_record_offset":6172,"ancestor_record_offset":1932,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":4378,"record_offset":5796,"parent_record_offset":5764,"ancestor_record_offset":1932,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":4378,"record_offset":5828,"parent_record_offset":5796,"ancestor_record_offset":1932,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":4378,"record_offset":6172,"parent_record_offset":6116,"ancestor_record_offset":1932,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":4378,"record_offset":5764,"parent_record_offset":5732,"ancestor_record_offset":1932,"inlinee":4330,"source_line":217,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":4378,"record_offset":5648,"parent_record_offset":3516,"ancestor_record_offset":1932,"inlinee":4097,"source_line":629,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":4378,"record_offset":5696,"parent_record_offset":5672,"ancestor_record_offset":1932,"inlinee":4101,"source_line":637,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":4378,"record_offset":6144,"parent_record_offset":6116,"ancestor_record_offset":1932,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":4378,"record_offset":6116,"parent_record_offset":6092,"ancestor_record_offset":1932,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::new
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":4378,"record_offset":5732,"parent_record_offset":3516,"ancestor_record_offset":1932,"inlinee":5312,"source_line":392,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::impl$24::eq
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/archetype.rs
//     evidence={"module":4378,"record_offset":2340,"parent_record_offset":2196,"ancestor_record_offset":1932,"inlinee":21390,"source_line":74,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::Entities::get
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/entity/mod.rs
//     evidence={"module":4378,"record_offset":2196,"parent_record_offset":2168,"ancestor_record_offset":1932,"inlinee":21387,"source_line":739,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$21::fetch
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":4378,"record_offset":3424,"parent_record_offset":2596,"ancestor_record_offset":1932,"inlinee":14627,"source_line":963,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$21::set_archetype
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":4378,"record_offset":3056,"parent_record_offset":2596,"ancestor_record_offset":1932,"inlinee":21391,"source_line":933,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$21::set_table
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":4378,"record_offset":3080,"parent_record_offset":3056,"ancestor_record_offset":1932,"inlinee":14620,"source_line":948,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::state::QueryState::get_unchecked_manual
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/state.rs
//     evidence={"module":4378,"record_offset":2168,"parent_record_offset":2140,"ancestor_record_offset":1932,"inlinee":21385,"source_line":859,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_slice
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":4378,"record_offset":3384,"parent_record_offset":3360,"ancestor_record_offset":1932,"inlinee":14643,"source_line":410,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseArray::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":4378,"record_offset":3156,"parent_record_offset":3132,"ancestor_record_offset":1932,"inlinee":6338,"source_line":56,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":4378,"record_offset":3132,"parent_record_offset":3108,"ancestor_record_offset":1932,"inlinee":6337,"source_line":369,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$11::get::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":4378,"record_offset":3204,"parent_record_offset":3180,"ancestor_record_offset":1932,"inlinee":6340,"source_line":58,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$16::get::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":4378,"record_offset":3292,"parent_record_offset":3268,"ancestor_record_offset":1932,"inlinee":6342,"source_line":370,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Column::get_data_slice
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":4378,"record_offset":3360,"parent_record_offset":3080,"ancestor_record_offset":1932,"inlinee":14642,"source_line":321,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_column
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":4378,"record_offset":3108,"parent_record_offset":3080,"ancestor_record_offset":1932,"inlinee":14621,"source_line":692,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":4378,"record_offset":3028,"parent_record_offset":2596,"ancestor_record_offset":1932,"inlinee":14618,"source_line":834,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ptr::ThinSlicePtr::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ptr/src/lib.rs
//     evidence={"module":4378,"record_offset":3448,"parent_record_offset":3424,"ancestor_record_offset":1932,"inlinee":21393,"source_line":500,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] core::bool::impl$0::then
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\bool.rs
//     evidence={"module":4378,"record_offset":2404,"parent_record_offset":2372,"ancestor_record_offset":1932,"inlinee":4886,"source_line":59,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$24::ne
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":4378,"record_offset":2308,"parent_record_offset":2284,"ancestor_record_offset":1932,"inlinee":13915,"source_line":1471,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$54::lt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":4378,"record_offset":4752,"parent_record_offset":4728,"ancestor_record_offset":1932,"inlinee":4237,"source_line":1553,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":4378,"record_offset":4960,"parent_record_offset":4936,"ancestor_record_offset":1932,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":4378,"record_offset":4196,"parent_record_offset":3780,"ancestor_record_offset":1932,"inlinee":15697,"source_line":31,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ss
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":4378,"record_offset":4104,"parent_record_offset":4048,"ancestor_record_offset":1932,"inlinee":21747,"source_line":20,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":4378,"record_offset":3876,"parent_record_offset":3852,"ancestor_record_offset":1932,"inlinee":14684,"source_line":77,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":4378,"record_offset":3908,"parent_record_offset":3824,"ancestor_record_offset":1932,"inlinee":14683,"source_line":1011,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_sub_ps
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":4378,"record_offset":3968,"parent_record_offset":3780,"ancestor_record_offset":1932,"inlinee":14685,"source_line":54,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":4378,"record_offset":4728,"parent_record_offset":4704,"ancestor_record_offset":1932,"inlinee":4235,"source_line":753,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":4378,"record_offset":4704,"parent_record_offset":3516,"ancestor_record_offset":1932,"inlinee":4233,"source_line":844,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::impl$2::ne
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":4378,"record_offset":2284,"parent_record_offset":2196,"ancestor_record_offset":1932,"inlinee":21388,"source_line":136,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":4378,"record_offset":4644,"parent_record_offset":4616,"ancestor_record_offset":1932,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":4378,"record_offset":5580,"parent_record_offset":5552,"ancestor_record_offset":1932,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":4378,"record_offset":4436,"parent_record_offset":4404,"ancestor_record_offset":1932,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":4378,"record_offset":3228,"parent_record_offset":3204,"ancestor_record_offset":1932,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":4378,"record_offset":3180,"parent_record_offset":3156,"ancestor_record_offset":1932,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":4378,"record_offset":5672,"parent_record_offset":5648,"ancestor_record_offset":1932,"inlinee":4099,"source_line":1172,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":4378,"record_offset":4824,"parent_record_offset":3516,"ancestor_record_offset":1932,"inlinee":4138,"source_line":972,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":4378,"record_offset":6044,"parent_record_offset":3516,"ancestor_record_offset":1932,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":4378,"record_offset":2512,"parent_record_offset":2480,"ancestor_record_offset":1932,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":4378,"record_offset":5904,"parent_record_offset":5876,"ancestor_record_offset":1932,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":4378,"record_offset":3000,"parent_record_offset":2596,"ancestor_record_offset":1932,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":4378,"record_offset":2228,"parent_record_offset":2196,"ancestor_record_offset":1932,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":4378,"record_offset":2252,"parent_record_offset":2228,"ancestor_record_offset":1932,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::contains
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":4378,"record_offset":2372,"parent_record_offset":2168,"ancestor_record_offset":1932,"inlinee":4884,"source_line":392,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::contains_unchecked
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":4378,"record_offset":2456,"parent_record_offset":2432,"ancestor_record_offset":1932,"inlinee":4889,"source_line":408,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::get_unchecked
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":4378,"record_offset":2480,"parent_record_offset":2456,"ancestor_record_offset":1932,"inlinee":4890,"source_line":171,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::impl$2::contains::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":4378,"record_offset":2432,"parent_record_offset":2404,"ancestor_record_offset":1932,"inlinee":4888,"source_line":395,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::mul_vec3
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/quat.rs
//     evidence={"module":4378,"record_offset":3756,"parent_record_offset":3516,"ancestor_record_offset":1932,"inlinee":14676,"source_line":659,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::mul_vec3a
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/quat.rs
//     evidence={"module":4378,"record_offset":3780,"parent_record_offset":3756,"ancestor_record_offset":1932,"inlinee":14681,"source_line":731,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::cross
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec3a.rs
//     evidence={"module":4378,"record_offset":4224,"parent_record_offset":3780,"ancestor_record_offset":1932,"inlinee":14682,"source_line":220,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec3a.rs
//     evidence={"module":4378,"record_offset":5008,"parent_record_offset":4984,"ancestor_record_offset":1932,"inlinee":14680,"source_line":90,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$45::from
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec3a.rs
//     evidence={"module":4378,"record_offset":4984,"parent_record_offset":4960,"ancestor_record_offset":1932,"inlinee":14678,"source_line":1358,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::distance
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":4378,"record_offset":6244,"parent_record_offset":3516,"ancestor_record_offset":1932,"inlinee":21439,"source_line":462,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":4378,"record_offset":4496,"parent_record_offset":3516,"ancestor_record_offset":1932,"inlinee":13203,"source_line":195,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$12::add
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":4378,"record_offset":5552,"parent_record_offset":4856,"ancestor_record_offset":1932,"inlinee":14692,"source_line":1081,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$17::sub
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":4378,"record_offset":4404,"parent_record_offset":3516,"ancestor_record_offset":1932,"inlinee":21370,"source_line":1135,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$9::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":4378,"record_offset":4616,"parent_record_offset":3516,"ancestor_record_offset":1932,"inlinee":14672,"source_line":1048,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_in_x
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/sse2.rs
//     evidence={"module":4378,"record_offset":3852,"parent_record_offset":3824,"ancestor_record_offset":1932,"inlinee":21746,"source_line":42,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_into_m128
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/sse2.rs
//     evidence={"module":4378,"record_offset":3824,"parent_record_offset":3780,"ancestor_record_offset":1932,"inlinee":21745,"source_line":66,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nonmax","version":"0.5.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":4378,"record_offset":3316,"parent_record_offset":3292,"ancestor_record_offset":1932,"inlinee":4372,"source_line":153,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":4378,"record_offset":6372,"parent_record_offset":6336,"ancestor_record_offset":1932,"inlinee":13701,"source_line":441,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::decorator_visual::cottage_dormers::cottage_dormer::cottage_dormer_dims
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/decorator_visual/cottage_dormers/cottage_dormer.rs
//     evidence={"module":4378,"record_offset":4788,"parent_record_offset":3516,"ancestor_record_offset":1932,"inlinee":21669,"source_line":31,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::ui_systems::handle_splitting_roof_decorators::display_roof_decorator_parts_and_get_hovered_part_id::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/ui_systems/handle_splitting_roof_decorators.rs
//     evidence={"module":4378,"record_offset":4856,"parent_record_offset":4824,"ancestor_record_offset":1932,"inlinee":120862,"source_line":128,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::ui_systems::handle_splitting_roof_decorators::display_roof_decorator_parts_and_get_hovered_part_id
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 64
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1D81C
// 首条位置: {"module":4378,"inlinee":120860}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":4378,"record_offset":5648,"parent_record_offset":3516,"ancestor_record_offset":3516,"inlinee":4097,"source_line":629,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_direct] alloc::sync::Arc::new
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":4378,"record_offset":5732,"parent_record_offset":3516,"ancestor_record_offset":3516,"inlinee":5312,"source_line":392,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_direct] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":4378,"record_offset":4704,"parent_record_offset":3516,"ancestor_record_offset":3516,"inlinee":4233,"source_line":844,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":4378,"record_offset":4824,"parent_record_offset":3516,"ancestor_record_offset":3516,"inlinee":4138,"source_line":972,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":4378,"record_offset":6044,"parent_record_offset":3516,"ancestor_record_offset":3516,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::quat::Quat::mul_vec3
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/quat.rs
//     evidence={"module":4378,"record_offset":3756,"parent_record_offset":3516,"ancestor_record_offset":3516,"inlinee":14676,"source_line":659,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::distance
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":4378,"record_offset":6244,"parent_record_offset":3516,"ancestor_record_offset":3516,"inlinee":21439,"source_line":462,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":4378,"record_offset":4496,"parent_record_offset":3516,"ancestor_record_offset":3516,"inlinee":13203,"source_line":195,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$12::add
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":4378,"record_offset":5956,"parent_record_offset":3516,"ancestor_record_offset":3516,"inlinee":14692,"source_line":1081,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$17::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":4378,"record_offset":4404,"parent_record_offset":3516,"ancestor_record_offset":3516,"inlinee":21370,"source_line":1135,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$9::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":4378,"record_offset":4616,"parent_record_offset":3516,"ancestor_record_offset":3516,"inlinee":14672,"source_line":1048,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::decorator_visual::cottage_dormers::cottage_dormer::cottage_dormer_dims
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/decorator_visual/cottage_dormers/cottage_dormer.rs
//     evidence={"module":4378,"record_offset":4788,"parent_record_offset":3516,"ancestor_record_offset":3516,"inlinee":21669,"source_line":31,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":4378,"record_offset":5852,"parent_record_offset":5828,"ancestor_record_offset":3516,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":4378,"record_offset":5876,"parent_record_offset":5852,"ancestor_record_offset":3516,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":4378,"record_offset":6196,"parent_record_offset":6172,"ancestor_record_offset":3516,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":4378,"record_offset":5796,"parent_record_offset":5764,"ancestor_record_offset":3516,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":4378,"record_offset":5828,"parent_record_offset":5796,"ancestor_record_offset":3516,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":4378,"record_offset":6172,"parent_record_offset":6116,"ancestor_record_offset":3516,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":4378,"record_offset":5764,"parent_record_offset":5732,"ancestor_record_offset":3516,"inlinee":4330,"source_line":217,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":4378,"record_offset":5696,"parent_record_offset":5672,"ancestor_record_offset":3516,"inlinee":4101,"source_line":637,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":4378,"record_offset":6144,"parent_record_offset":6116,"ancestor_record_offset":3516,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":4378,"record_offset":6116,"parent_record_offset":6092,"ancestor_record_offset":3516,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$54::lt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":4378,"record_offset":4752,"parent_record_offset":4728,"ancestor_record_offset":3516,"inlinee":4237,"source_line":1553,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":4378,"record_offset":4960,"parent_record_offset":4936,"ancestor_record_offset":3516,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":4378,"record_offset":4196,"parent_record_offset":3780,"ancestor_record_offset":3516,"inlinee":15697,"source_line":31,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ss
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":4378,"record_offset":4104,"parent_record_offset":4048,"ancestor_record_offset":3516,"inlinee":21747,"source_line":20,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":4378,"record_offset":3876,"parent_record_offset":3852,"ancestor_record_offset":3516,"inlinee":14684,"source_line":77,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":4378,"record_offset":3908,"parent_record_offset":3824,"ancestor_record_offset":3516,"inlinee":14683,"source_line":1011,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_sub_ps
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":4378,"record_offset":3968,"parent_record_offset":3780,"ancestor_record_offset":3516,"inlinee":14685,"source_line":54,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":4378,"record_offset":4728,"parent_record_offset":4704,"ancestor_record_offset":3516,"inlinee":4235,"source_line":753,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":4378,"record_offset":4644,"parent_record_offset":4616,"ancestor_record_offset":3516,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":4378,"record_offset":5580,"parent_record_offset":5552,"ancestor_record_offset":3516,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":4378,"record_offset":4436,"parent_record_offset":4404,"ancestor_record_offset":3516,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":4378,"record_offset":5672,"parent_record_offset":5648,"ancestor_record_offset":3516,"inlinee":4099,"source_line":1172,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":4378,"record_offset":6068,"parent_record_offset":6044,"ancestor_record_offset":3516,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":4378,"record_offset":5904,"parent_record_offset":5876,"ancestor_record_offset":3516,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::mul_vec3
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/quat.rs
//     evidence={"module":4378,"record_offset":4936,"parent_record_offset":4856,"ancestor_record_offset":3516,"inlinee":14676,"source_line":659,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::mul_vec3a
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/quat.rs
//     evidence={"module":4378,"record_offset":3780,"parent_record_offset":3756,"ancestor_record_offset":3516,"inlinee":14681,"source_line":731,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::cross
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec3a.rs
//     evidence={"module":4378,"record_offset":4224,"parent_record_offset":3780,"ancestor_record_offset":3516,"inlinee":14682,"source_line":220,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec3a.rs
//     evidence={"module":4378,"record_offset":5008,"parent_record_offset":4984,"ancestor_record_offset":3516,"inlinee":14680,"source_line":90,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$45::from
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec3a.rs
//     evidence={"module":4378,"record_offset":4984,"parent_record_offset":4960,"ancestor_record_offset":3516,"inlinee":14678,"source_line":1358,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":4378,"record_offset":6336,"parent_record_offset":6268,"ancestor_record_offset":3516,"inlinee":13203,"source_line":195,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$12::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":4378,"record_offset":5552,"parent_record_offset":4856,"ancestor_record_offset":3516,"inlinee":14692,"source_line":1081,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$17::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":4378,"record_offset":6268,"parent_record_offset":6244,"ancestor_record_offset":3516,"inlinee":21370,"source_line":1135,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$9::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":4378,"record_offset":4880,"parent_record_offset":4856,"ancestor_record_offset":3516,"inlinee":14672,"source_line":1048,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_in_x
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/sse2.rs
//     evidence={"module":4378,"record_offset":3852,"parent_record_offset":3824,"ancestor_record_offset":3516,"inlinee":21746,"source_line":42,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_into_m128
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/sse2.rs
//     evidence={"module":4378,"record_offset":3824,"parent_record_offset":3780,"ancestor_record_offset":3516,"inlinee":21745,"source_line":66,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":4378,"record_offset":6372,"parent_record_offset":6336,"ancestor_record_offset":3516,"inlinee":13701,"source_line":441,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::ui_systems::handle_splitting_roof_decorators::display_roof_decorator_parts_and_get_hovered_part_id::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/ui_systems/handle_splitting_roof_decorators.rs
//     evidence={"module":4378,"record_offset":4856,"parent_record_offset":4824,"ancestor_record_offset":3516,"inlinee":120862,"source_line":128,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::ui_systems::handle_splitting_roof_decorators::display_roof_decorator_parts_and_get_hovered_part_id::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 128
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1D81E
// 首条位置: {"module":4378,"inlinee":120862}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::sse2::quat::Quat::mul_vec3
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/quat.rs
//     evidence={"module":4378,"record_offset":4936,"parent_record_offset":4856,"ancestor_record_offset":4856,"inlinee":14676,"source_line":659,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$12::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":4378,"record_offset":5552,"parent_record_offset":4856,"ancestor_record_offset":4856,"inlinee":14692,"source_line":1081,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$9::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":4378,"record_offset":4880,"parent_record_offset":4856,"ancestor_record_offset":4856,"inlinee":14672,"source_line":1048,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":4378,"record_offset":4960,"parent_record_offset":4936,"ancestor_record_offset":4856,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":4378,"record_offset":5284,"parent_record_offset":5044,"ancestor_record_offset":4856,"inlinee":15697,"source_line":31,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ss
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":4378,"record_offset":5192,"parent_record_offset":5136,"ancestor_record_offset":4856,"inlinee":21747,"source_line":20,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":4378,"record_offset":5084,"parent_record_offset":5044,"ancestor_record_offset":4856,"inlinee":14684,"source_line":77,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":4378,"record_offset":5372,"parent_record_offset":5312,"ancestor_record_offset":4856,"inlinee":14683,"source_line":1011,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_sub_ps
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":4378,"record_offset":5428,"parent_record_offset":5312,"ancestor_record_offset":4856,"inlinee":14685,"source_line":54,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":4378,"record_offset":4904,"parent_record_offset":4880,"ancestor_record_offset":4856,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":4378,"record_offset":5580,"parent_record_offset":5552,"ancestor_record_offset":4856,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::mul_vec3a
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/quat.rs
//     evidence={"module":4378,"record_offset":5044,"parent_record_offset":4936,"ancestor_record_offset":4856,"inlinee":14681,"source_line":731,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::cross
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec3a.rs
//     evidence={"module":4378,"record_offset":5312,"parent_record_offset":5044,"ancestor_record_offset":4856,"inlinee":14682,"source_line":220,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec3a.rs
//     evidence={"module":4378,"record_offset":5008,"parent_record_offset":4984,"ancestor_record_offset":4856,"inlinee":14680,"source_line":90,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$45::from
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec3a.rs
//     evidence={"module":4378,"record_offset":4984,"parent_record_offset":4960,"ancestor_record_offset":4856,"inlinee":14678,"source_line":1358,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_in_x
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/sse2.rs
//     evidence={"module":4378,"record_offset":5136,"parent_record_offset":5112,"ancestor_record_offset":4856,"inlinee":21746,"source_line":42,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_into_m128
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/sse2.rs
//     evidence={"module":4378,"record_offset":5112,"parent_record_offset":5044,"ancestor_record_offset":4856,"inlinee":21745,"source_line":66,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::ui_systems::handle_splitting_roof_decorators::display_roof_decorator_parts_and_get_hovered_part_id::closure$1
// 记录形式: compiled_procedure; instances=1
// 源码行: 139
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":4378,"record_offset":8176,"rva":31881712,"va":5400590832}
// 编译实例 RVAs: 0x1e679f0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: handle_splitting_roof_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-27841202","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400590832,"raw_hex":"e84e2d57fe","size":5,"soft_fail":false,"target_va":5372749696,"va":5400590893,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: handle_splitting_roof_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-27841549","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400590832,"raw_hex":"e8f32b57fe","size":5,"soft_fail":false,"target_va":5372749712,"va":5400591256,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t2386954","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400590832,"raw_hex":"e80a6c2400","size":5,"soft_fail":false,"target_va":5402978144,"va":5400591185,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::animation_duration
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-25234329","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400590832,"raw_hex":"e867f47efe","size":5,"soft_fail":false,"target_va":5375356704,"va":5400591028,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::build
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-25234229","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400590832,"raw_hex":"e8cbf47efe","size":5,"soft_fail":false,"target_va":5375356896,"va":5400591120,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::new
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-25235028","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400590832,"raw_hex":"e8acf17efe","size":5,"soft_fail":false,"target_va":5375355984,"va":5400591007,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::position
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-25234534","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400590832,"raw_hex":"e89af37efe","size":5,"soft_fail":false,"target_va":5375356560,"va":5400591089,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::scale
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-25234398","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400590832,"raw_hex":"e822f47efe","size":5,"soft_fail":false,"target_va":5375356656,"va":5400591049,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":4378,"record_offset":8400,"parent_record_offset":8176,"ancestor_record_offset":8176,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_direct] alloc::slice::impl$0::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":4378,"record_offset":9104,"parent_record_offset":8176,"ancestor_record_offset":8176,"inlinee":4441,"source_line":461,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":4378,"record_offset":8548,"parent_record_offset":8176,"ancestor_record_offset":8176,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":4378,"record_offset":9188,"parent_record_offset":8176,"ancestor_record_offset":8176,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":4378,"record_offset":8456,"parent_record_offset":8432,"ancestor_record_offset":8176,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":4378,"record_offset":8480,"parent_record_offset":8456,"ancestor_record_offset":8176,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":4378,"record_offset":9260,"parent_record_offset":9236,"ancestor_record_offset":8176,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":4378,"record_offset":8432,"parent_record_offset":8400,"ancestor_record_offset":8176,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":4378,"record_offset":9236,"parent_record_offset":9212,"ancestor_record_offset":8176,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":4378,"record_offset":9212,"parent_record_offset":9188,"ancestor_record_offset":8176,"inlinee":4113,"source_line":1240,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":4378,"record_offset":8836,"parent_record_offset":8808,"ancestor_record_offset":8176,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":4378,"record_offset":8808,"parent_record_offset":8780,"ancestor_record_offset":8176,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":4378,"record_offset":8980,"parent_record_offset":8808,"ancestor_record_offset":8176,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":4378,"record_offset":8744,"parent_record_offset":8716,"ancestor_record_offset":8176,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":4378,"record_offset":9128,"parent_record_offset":9104,"ancestor_record_offset":8176,"inlinee":4442,"source_line":100,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":4378,"record_offset":8716,"parent_record_offset":8688,"ancestor_record_offset":8176,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":4378,"record_offset":8660,"parent_record_offset":8632,"ancestor_record_offset":8176,"inlinee":4415,"source_line":412,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":4378,"record_offset":8688,"parent_record_offset":8660,"ancestor_record_offset":8176,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":4378,"record_offset":8632,"parent_record_offset":8604,"ancestor_record_offset":8176,"inlinee":4413,"source_line":822,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\str.rs
//     evidence={"module":4378,"record_offset":8604,"parent_record_offset":8576,"ancestor_record_offset":8176,"inlinee":4411,"source_line":210,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$44::from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":4378,"record_offset":8576,"parent_record_offset":8548,"ancestor_record_offset":8176,"inlinee":4938,"source_line":2705,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::from_raw_parts_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":4378,"record_offset":9152,"parent_record_offset":9128,"ancestor_record_offset":8176,"inlinee":4443,"source_line":819,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":4378,"record_offset":8780,"parent_record_offset":8744,"ancestor_record_offset":8176,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":4378,"record_offset":9040,"parent_record_offset":9016,"ancestor_record_offset":8176,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":4378,"record_offset":9016,"parent_record_offset":8744,"ancestor_record_offset":8176,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":4378,"record_offset":8504,"parent_record_offset":8480,"ancestor_record_offset":8176,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::ui_systems::handle_splitting_roof_decorators::display_roof_decorator_parts_and_get_hovered_part_id::closure$2
// 记录形式: compiled_procedure; instances=1
// 源码行: 166
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":4378,"record_offset":9304,"rva":31882160,"va":5400591280}
// 编译实例 RVAs: 0x1e67bb0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: handle_splitting_roof_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-27841650","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400591280,"raw_hex":"e88e2b57fe","size":5,"soft_fail":false,"target_va":5372749696,"va":5400591341,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: handle_splitting_roof_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-27841997","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400591280,"raw_hex":"e8332a57fe","size":5,"soft_fail":false,"target_va":5372749712,"va":5400591704,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t2386506","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400591280,"raw_hex":"e84a6a2400","size":5,"soft_fail":false,"target_va":5402978144,"va":5400591633,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::animation_duration
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-25234777","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400591280,"raw_hex":"e8a7f27efe","size":5,"soft_fail":false,"target_va":5375356704,"va":5400591476,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::build
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-25234677","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400591280,"raw_hex":"e80bf37efe","size":5,"soft_fail":false,"target_va":5375356896,"va":5400591568,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::new
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-25235476","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400591280,"raw_hex":"e8ecef7efe","size":5,"soft_fail":false,"target_va":5375355984,"va":5400591455,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::position
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-25234982","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400591280,"raw_hex":"e8daf17efe","size":5,"soft_fail":false,"target_va":5375356560,"va":5400591537,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::scale
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-25234846","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400591280,"raw_hex":"e862f27efe","size":5,"soft_fail":false,"target_va":5375356656,"va":5400591497,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":4378,"record_offset":9528,"parent_record_offset":9304,"ancestor_record_offset":9304,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_direct] alloc::slice::impl$0::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":4378,"record_offset":10232,"parent_record_offset":9304,"ancestor_record_offset":9304,"inlinee":4441,"source_line":461,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":4378,"record_offset":9676,"parent_record_offset":9304,"ancestor_record_offset":9304,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":4378,"record_offset":10316,"parent_record_offset":9304,"ancestor_record_offset":9304,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":4378,"record_offset":9584,"parent_record_offset":9560,"ancestor_record_offset":9304,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":4378,"record_offset":9608,"parent_record_offset":9584,"ancestor_record_offset":9304,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":4378,"record_offset":10388,"parent_record_offset":10364,"ancestor_record_offset":9304,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":4378,"record_offset":9560,"parent_record_offset":9528,"ancestor_record_offset":9304,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":4378,"record_offset":10364,"parent_record_offset":10340,"ancestor_record_offset":9304,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":4378,"record_offset":10340,"parent_record_offset":10316,"ancestor_record_offset":9304,"inlinee":4113,"source_line":1240,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":4378,"record_offset":9964,"parent_record_offset":9936,"ancestor_record_offset":9304,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":4378,"record_offset":9936,"parent_record_offset":9908,"ancestor_record_offset":9304,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":4378,"record_offset":10108,"parent_record_offset":9936,"ancestor_record_offset":9304,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":4378,"record_offset":9872,"parent_record_offset":9844,"ancestor_record_offset":9304,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":4378,"record_offset":10256,"parent_record_offset":10232,"ancestor_record_offset":9304,"inlinee":4442,"source_line":100,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":4378,"record_offset":9844,"parent_record_offset":9816,"ancestor_record_offset":9304,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":4378,"record_offset":9788,"parent_record_offset":9760,"ancestor_record_offset":9304,"inlinee":4415,"source_line":412,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":4378,"record_offset":9816,"parent_record_offset":9788,"ancestor_record_offset":9304,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":4378,"record_offset":9760,"parent_record_offset":9732,"ancestor_record_offset":9304,"inlinee":4413,"source_line":822,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\str.rs
//     evidence={"module":4378,"record_offset":9732,"parent_record_offset":9704,"ancestor_record_offset":9304,"inlinee":4411,"source_line":210,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$44::from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":4378,"record_offset":9704,"parent_record_offset":9676,"ancestor_record_offset":9304,"inlinee":4938,"source_line":2705,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::from_raw_parts_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":4378,"record_offset":10280,"parent_record_offset":10256,"ancestor_record_offset":9304,"inlinee":4443,"source_line":819,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":4378,"record_offset":9908,"parent_record_offset":9872,"ancestor_record_offset":9304,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":4378,"record_offset":10168,"parent_record_offset":10144,"ancestor_record_offset":9304,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":4378,"record_offset":10144,"parent_record_offset":9872,"ancestor_record_offset":9304,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":4378,"record_offset":9632,"parent_record_offset":9608,"ancestor_record_offset":9304,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::ui_systems::handle_splitting_roof_decorators::display_roof_decorator_parts_and_get_hovered_part_id::closure$3
// 记录形式: compiled_procedure; instances=1
// 源码行: 183
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":4378,"record_offset":10432,"rva":31882608,"va":5400591728}
// 编译实例 RVAs: 0x1e67d70
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: handle_splitting_roof_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-27842098","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400591728,"raw_hex":"e8ce2957fe","size":5,"soft_fail":false,"target_va":5372749696,"va":5400591789,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: handle_splitting_roof_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-27842445","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400591728,"raw_hex":"e8732857fe","size":5,"soft_fail":false,"target_va":5372749712,"va":5400592152,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t2386058","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400591728,"raw_hex":"e88a682400","size":5,"soft_fail":false,"target_va":5402978144,"va":5400592081,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::animation_duration
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-25235225","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400591728,"raw_hex":"e8e7f07efe","size":5,"soft_fail":false,"target_va":5375356704,"va":5400591924,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::build
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-25235125","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400591728,"raw_hex":"e84bf17efe","size":5,"soft_fail":false,"target_va":5375356896,"va":5400592016,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::new
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-25235924","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400591728,"raw_hex":"e82cee7efe","size":5,"soft_fail":false,"target_va":5375355984,"va":5400591903,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::position
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-25235430","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400591728,"raw_hex":"e81af07efe","size":5,"soft_fail":false,"target_va":5375356560,"va":5400591985,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::scale
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-25235294","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400591728,"raw_hex":"e8a2f07efe","size":5,"soft_fail":false,"target_va":5375356656,"va":5400591945,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":4378,"record_offset":10656,"parent_record_offset":10432,"ancestor_record_offset":10432,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_direct] alloc::slice::impl$0::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":4378,"record_offset":11360,"parent_record_offset":10432,"ancestor_record_offset":10432,"inlinee":4441,"source_line":461,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":4378,"record_offset":10804,"parent_record_offset":10432,"ancestor_record_offset":10432,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":4378,"record_offset":11444,"parent_record_offset":10432,"ancestor_record_offset":10432,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":4378,"record_offset":10712,"parent_record_offset":10688,"ancestor_record_offset":10432,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":4378,"record_offset":10736,"parent_record_offset":10712,"ancestor_record_offset":10432,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":4378,"record_offset":11516,"parent_record_offset":11492,"ancestor_record_offset":10432,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":4378,"record_offset":10688,"parent_record_offset":10656,"ancestor_record_offset":10432,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":4378,"record_offset":11492,"parent_record_offset":11468,"ancestor_record_offset":10432,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":4378,"record_offset":11468,"parent_record_offset":11444,"ancestor_record_offset":10432,"inlinee":4113,"source_line":1240,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":4378,"record_offset":11092,"parent_record_offset":11064,"ancestor_record_offset":10432,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":4378,"record_offset":11064,"parent_record_offset":11036,"ancestor_record_offset":10432,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":4378,"record_offset":11236,"parent_record_offset":11064,"ancestor_record_offset":10432,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":4378,"record_offset":11000,"parent_record_offset":10972,"ancestor_record_offset":10432,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":4378,"record_offset":11384,"parent_record_offset":11360,"ancestor_record_offset":10432,"inlinee":4442,"source_line":100,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":4378,"record_offset":10972,"parent_record_offset":10944,"ancestor_record_offset":10432,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":4378,"record_offset":10916,"parent_record_offset":10888,"ancestor_record_offset":10432,"inlinee":4415,"source_line":412,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":4378,"record_offset":10944,"parent_record_offset":10916,"ancestor_record_offset":10432,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":4378,"record_offset":10888,"parent_record_offset":10860,"ancestor_record_offset":10432,"inlinee":4413,"source_line":822,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\str.rs
//     evidence={"module":4378,"record_offset":10860,"parent_record_offset":10832,"ancestor_record_offset":10432,"inlinee":4411,"source_line":210,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$44::from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":4378,"record_offset":10832,"parent_record_offset":10804,"ancestor_record_offset":10432,"inlinee":4938,"source_line":2705,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::from_raw_parts_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":4378,"record_offset":11408,"parent_record_offset":11384,"ancestor_record_offset":10432,"inlinee":4443,"source_line":819,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":4378,"record_offset":11036,"parent_record_offset":11000,"ancestor_record_offset":10432,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":4378,"record_offset":11296,"parent_record_offset":11272,"ancestor_record_offset":10432,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":4378,"record_offset":11272,"parent_record_offset":11000,"ancestor_record_offset":10432,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":4378,"record_offset":10760,"parent_record_offset":10736,"ancestor_record_offset":10432,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::ui_systems::handle_splitting_roof_decorators::handle_moving_out_roof_decorator_part
// 记录形式: compiled_procedure; instances=1
// 源码行: 202
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":4378,"record_offset":11560,"rva":31883056,"va":5400592176}
// 编译实例 RVAs: 0x1e67f30
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140004020
//     owner=unresolved_namespace; count=1
//     target_aliases=154 candidates; complete list: handle_splitting_roof_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-31869040","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400592176,"raw_hex":"e890b719fe","size":5,"soft_fail":false,"target_va":5368725536,"va":5400594571,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dccd0
//     owner=unresolved_namespace; count=1
//     target_aliases=9 candidates; complete list: handle_splitting_roof_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-30978864","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400592176,"raw_hex":"e8d04c27fe","size":5,"soft_fail":false,"target_va":5369613520,"va":5400592379,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: handle_splitting_roof_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-27845205","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400592176,"raw_hex":"e8ab1d57fe","size":5,"soft_fail":false,"target_va":5372749712,"va":5400594912,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140593250
//     owner=unresolved_namespace; count=1
//     target_aliases=15 candidates; complete list: handle_splitting_roof_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-26038074","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400592176,"raw_hex":"e8c6b072fe","size":5,"soft_fail":false,"target_va":5374554704,"va":5400592773,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405baba0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: handle_splitting_roof_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-25875503","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400592176,"raw_hex":"e8d12b75fe","size":5,"soft_fail":false,"target_va":5374716832,"va":5400592330,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140633130
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: handle_splitting_roof_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-25383023","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400592176,"raw_hex":"e891af7cfe","size":5,"soft_fail":false,"target_va":5375209776,"va":5400592794,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d01eb0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: handle_splitting_roof_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18246981","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400592176,"raw_hex":"e8bb92e9fe","size":5,"soft_fail":false,"target_va":5382348464,"va":5400595440,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d06720
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: handle_splitting_roof_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18225819","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400592176,"raw_hex":"e865e5e9fe","size":5,"soft_fail":false,"target_va":5382367008,"va":5400592822,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-29288617","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400592176,"raw_hex":"e8571741fe","size":5,"soft_fail":false,"target_va":5371305056,"va":5400593668,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t2383846","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400592176,"raw_hex":"e8e65f2400","size":5,"soft_fail":false,"target_va":5402978880,"va":5400595029,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t2384135","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400592176,"raw_hex":"e807612400","size":5,"soft_fail":false,"target_va":5402979152,"va":5400595012,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t2383799","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400592176,"raw_hex":"e8b75f2400","size":5,"soft_fail":false,"target_va":5402978944,"va":5400595140,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms::position
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-26230091","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400592176,"raw_hex":"e8b5c26ffe","size":5,"soft_fail":false,"target_va":5374362208,"va":5400592294,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator::DecoratorInfo::dst_as_roof
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-26021509","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400592176,"raw_hex":"e87bf172fe","size":5,"soft_fail":false,"target_va":5374571120,"va":5400592624,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator::TransientDecoratorIdGen::generate
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-26021637","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400592176,"raw_hex":"e8fbf072fe","size":5,"soft_fail":false,"target_va":5374572432,"va":5400594064,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo::subtype
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-25379996","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400592176,"raw_hex":"e864bb7cfe","size":5,"soft_fail":false,"target_va":5375212688,"va":5400592679,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] smallvec::SmallVec$LT$A$GT$::reserve_one_unchecked
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2997738","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400592176,"raw_hex":"e8eabd2d00","size":5,"soft_fail":false,"target_va":5403590992,"va":5400593249,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_decorator::apply_decorator_op::apply_decorator_op
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t-18228394","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400592176,"raw_hex":"e856dbe9fe","size":5,"soft_fail":false,"target_va":5382365568,"va":5400593957,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_decorator::ui_systems::handle_splitting_roof_decorators::handle_moving_out_roof_decorator_part::closure$1
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t1960","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400592176,"raw_hex":"e8a8070000","size":5,"soft_fail":false,"target_va":5400595696,"va":5400593731,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":4378,"record_offset":14288,"parent_record_offset":11560,"ancestor_record_offset":11560,"inlinee":4175,"source_line":422,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":4378,"record_offset":14420,"parent_record_offset":11560,"ancestor_record_offset":11560,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":4378,"record_offset":15360,"parent_record_offset":11560,"ancestor_record_offset":11560,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::filter::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\filter.rs
//     evidence={"module":4378,"record_offset":13296,"parent_record_offset":11560,"ancestor_record_offset":11560,"inlinee":8575,"source_line":59,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_direct] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":4378,"record_offset":13944,"parent_record_offset":11560,"ancestor_record_offset":11560,"inlinee":4983,"source_line":604,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_none
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":4378,"record_offset":14836,"parent_record_offset":11560,"ancestor_record_offset":11560,"inlinee":4710,"source_line":651,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::map_or
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":4378,"record_offset":14736,"parent_record_offset":11560,"ancestor_record_offset":11560,"inlinee":4773,"source_line":1127,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":4378,"record_offset":11824,"parent_record_offset":11560,"ancestor_record_offset":11560,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":4378,"record_offset":14920,"parent_record_offset":11560,"ancestor_record_offset":11560,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo::bbx_ls
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":4378,"record_offset":12924,"parent_record_offset":11560,"ancestor_record_offset":11560,"inlinee":21725,"source_line":267,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$10::mul_assign
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":4378,"record_offset":13888,"parent_record_offset":11560,"ancestor_record_offset":11560,"inlinee":14658,"source_line":982,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$17::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":4378,"record_offset":13776,"parent_record_offset":11560,"ancestor_record_offset":11560,"inlinee":14656,"source_line":1051,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$9::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":4378,"record_offset":13832,"parent_record_offset":11560,"ancestor_record_offset":11560,"inlinee":14701,"source_line":972,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_direct] smallvec::SmallVec::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"smallvec","version":"1.13.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/smallvec-1.13.2/src/lib.rs
//     evidence={"module":4378,"record_offset":12860,"parent_record_offset":11560,"ancestor_record_offset":11560,"inlinee":21555,"source_line":779,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_direct] smallvec::SmallVec::push
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"smallvec","version":"1.13.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/smallvec-1.13.2/src/lib.rs
//     evidence={"module":4378,"record_offset":13116,"parent_record_offset":11560,"ancestor_record_offset":11560,"inlinee":21554,"source_line":1114,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_direct] smallvec::impl$44::next
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"smallvec","version":"1.13.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/smallvec-1.13.2/src/lib.rs
//     evidence={"module":4378,"record_offset":14316,"parent_record_offset":11560,"ancestor_record_offset":11560,"inlinee":34931,"source_line":2237,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_direct] smallvec::impl$49::into_iter
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"smallvec","version":"1.13.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/smallvec-1.13.2/src/lib.rs
//     evidence={"module":4378,"record_offset":14044,"parent_record_offset":11560,"ancestor_record_offset":11560,"inlinee":34978,"source_line":2290,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::get_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":4378,"record_offset":11860,"parent_record_offset":11560,"ancestor_record_offset":11560,"inlinee":5613,"source_line":1070,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::decorator_visual::cottage_dormers::cottage_dormer::cottage_dormer_dims
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/decorator_visual/cottage_dormers/cottage_dormer.rs
//     evidence={"module":4378,"record_offset":12888,"parent_record_offset":11560,"ancestor_record_offset":11560,"inlinee":21669,"source_line":31,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle3d::height
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":4378,"record_offset":13088,"parent_record_offset":11560,"ancestor_record_offset":11560,"inlinee":21736,"source_line":65,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":4378,"record_offset":15272,"parent_record_offset":15248,"ancestor_record_offset":11560,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":4378,"record_offset":15248,"parent_record_offset":15160,"ancestor_record_offset":11560,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":4378,"record_offset":15188,"parent_record_offset":15160,"ancestor_record_offset":11560,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":4378,"record_offset":14476,"parent_record_offset":14452,"ancestor_record_offset":11560,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":4378,"record_offset":15160,"parent_record_offset":15136,"ancestor_record_offset":11560,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":4378,"record_offset":14452,"parent_record_offset":14420,"ancestor_record_offset":11560,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\array\iter.rs
//     evidence={"module":4378,"record_offset":13372,"parent_record_offset":13344,"ancestor_record_offset":11560,"inlinee":6222,"source_line":239,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::impl$2::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\array\iter.rs
//     evidence={"module":4378,"record_offset":13460,"parent_record_offset":13396,"ancestor_record_offset":11560,"inlinee":11996,"source_line":245,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":4378,"record_offset":12644,"parent_record_offset":12620,"ancestor_record_offset":11560,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":4378,"record_offset":12112,"parent_record_offset":12088,"ancestor_record_offset":11560,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":4378,"record_offset":12196,"parent_record_offset":12172,"ancestor_record_offset":11560,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":4378,"record_offset":13044,"parent_record_offset":13020,"ancestor_record_offset":11560,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":4378,"record_offset":12136,"parent_record_offset":12112,"ancestor_record_offset":11560,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::find
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":4378,"record_offset":13320,"parent_record_offset":13296,"ancestor_record_offset":11560,"inlinee":6348,"source_line":2866,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::find::check::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":4378,"record_offset":13644,"parent_record_offset":13344,"ancestor_record_offset":11560,"inlinee":6192,"source_line":2873,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":4378,"record_offset":13344,"parent_record_offset":13320,"ancestor_record_offset":11560,"inlinee":4926,"source_line":2401,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::impl$0::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":4378,"record_offset":14972,"parent_record_offset":14948,"ancestor_record_offset":11560,"inlinee":6886,"source_line":4105,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":4378,"record_offset":15088,"parent_record_offset":15060,"ancestor_record_offset":11560,"inlinee":5355,"source_line":992,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":4378,"record_offset":13972,"parent_record_offset":13944,"ancestor_record_offset":11560,"inlinee":4984,"source_line":2074,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":4378,"record_offset":15212,"parent_record_offset":15188,"ancestor_record_offset":11560,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":4378,"record_offset":12308,"parent_record_offset":12284,"ancestor_record_offset":11560,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":4378,"record_offset":13856,"parent_record_offset":13832,"ancestor_record_offset":11560,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$364::mul_assign
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":4378,"record_offset":13912,"parent_record_offset":13888,"ancestor_record_offset":11560,"inlinee":14660,"source_line":886,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":4378,"record_offset":13800,"parent_record_offset":13776,"ancestor_record_offset":11560,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$3::call_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\function.rs
//     evidence={"module":4378,"record_offset":13668,"parent_record_offset":13644,"ancestor_record_offset":11560,"inlinee":4399,"source_line":293,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::index_range::IndexRange::next_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\index_range.rs
//     evidence={"module":4378,"record_offset":13432,"parent_record_offset":13396,"ancestor_record_offset":11560,"inlinee":6100,"source_line":54,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::index_range::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\index_range.rs
//     evidence={"module":4378,"record_offset":13396,"parent_record_offset":13372,"ancestor_record_offset":11560,"inlinee":6069,"source_line":118,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":4378,"record_offset":14860,"parent_record_offset":14836,"ancestor_record_offset":11560,"inlinee":4711,"source_line":607,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":4378,"record_offset":12988,"parent_record_offset":12960,"ancestor_record_offset":11560,"inlinee":4138,"source_line":972,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":4378,"record_offset":13540,"parent_record_offset":13516,"ancestor_record_offset":11560,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":4378,"record_offset":13572,"parent_record_offset":13484,"ancestor_record_offset":11560,"inlinee":6098,"source_line":1250,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":4378,"record_offset":15036,"parent_record_offset":14920,"ancestor_record_offset":11560,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":4378,"record_offset":13196,"parent_record_offset":13116,"ancestor_record_offset":11560,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":4378,"record_offset":12556,"parent_record_offset":12532,"ancestor_record_offset":11560,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":4378,"record_offset":12448,"parent_record_offset":12424,"ancestor_record_offset":11560,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":4378,"record_offset":13596,"parent_record_offset":13572,"ancestor_record_offset":11560,"inlinee":4136,"source_line":1246,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":4378,"record_offset":14508,"parent_record_offset":14420,"ancestor_record_offset":11560,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":4378,"record_offset":13484,"parent_record_offset":13460,"ancestor_record_offset":11560,"inlinee":5964,"source_line":679,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":4378,"record_offset":13516,"parent_record_offset":13484,"ancestor_record_offset":11560,"inlinee":4373,"source_line":232,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator::impl$25::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":4378,"record_offset":12668,"parent_record_offset":12644,"ancestor_record_offset":11560,"inlinee":21651,"source_line":84,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary::get_collision_bbx_ls
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":4378,"record_offset":12960,"parent_record_offset":12924,"ancestor_record_offset":11560,"inlinee":21727,"source_line":199,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::impl$3::get_collision_bbx_ls::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":4378,"record_offset":13020,"parent_record_offset":12988,"ancestor_record_offset":11560,"inlinee":21755,"source_line":202,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\lib.rs
//     evidence={"module":4378,"record_offset":12620,"parent_record_offset":12596,"ancestor_record_offset":11560,"inlinee":4854,"source_line":170,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner_mut
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":4378,"record_offset":11908,"parent_record_offset":11884,"ancestor_record_offset":11560,"inlinee":5615,"source_line":1461,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_mut
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":4378,"record_offset":11884,"parent_record_offset":11860,"ancestor_record_offset":11560,"inlinee":5614,"source_line":1449,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":4378,"record_offset":12596,"parent_record_offset":12376,"ancestor_record_offset":11560,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":4378,"record_offset":11940,"parent_record_offset":11908,"ancestor_record_offset":11560,"inlinee":5617,"source_line":257,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":4378,"record_offset":12508,"parent_record_offset":12484,"ancestor_record_offset":11560,"inlinee":5640,"source_line":513,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":4378,"record_offset":12484,"parent_record_offset":12376,"ancestor_record_offset":11560,"inlinee":5639,"source_line":682,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":4378,"record_offset":12424,"parent_record_offset":12400,"ancestor_record_offset":11560,"inlinee":5638,"source_line":355,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":4378,"record_offset":12804,"parent_record_offset":12016,"ancestor_record_offset":11560,"inlinee":5645,"source_line":175,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":4378,"record_offset":12400,"parent_record_offset":12376,"ancestor_record_offset":11560,"inlinee":5636,"source_line":997,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":4378,"record_offset":11992,"parent_record_offset":11968,"ancestor_record_offset":11560,"inlinee":5620,"source_line":1456,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get_mut
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":4378,"record_offset":11968,"parent_record_offset":11908,"ancestor_record_offset":11560,"inlinee":5619,"source_line":1489,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":4378,"record_offset":12016,"parent_record_offset":11992,"ancestor_record_offset":11560,"inlinee":5622,"source_line":2168,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":4378,"record_offset":12256,"parent_record_offset":12228,"ancestor_record_offset":11560,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":4378,"record_offset":12284,"parent_record_offset":12256,"ancestor_record_offset":11560,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::remove_lowest_bit
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":4378,"record_offset":12344,"parent_record_offset":12228,"ancestor_record_offset":11560,"inlinee":5633,"source_line":37,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":4378,"record_offset":12228,"parent_record_offset":12016,"ancestor_record_offset":11560,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::h2
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":4378,"record_offset":12060,"parent_record_offset":12016,"ancestor_record_offset":11560,"inlinee":5624,"source_line":150,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":4378,"record_offset":12376,"parent_record_offset":12016,"ancestor_record_offset":11560,"inlinee":5635,"source_line":1466,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":4378,"record_offset":12088,"parent_record_offset":12016,"ancestor_record_offset":11560,"inlinee":5626,"source_line":52,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_byte
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":4378,"record_offset":12172,"parent_record_offset":12016,"ancestor_record_offset":11560,"inlinee":5627,"source_line":79,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":4378,"record_offset":12712,"parent_record_offset":12016,"ancestor_record_offset":11560,"inlinee":5643,"source_line":97,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] smallvec::SmallVec::as_ptr
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"smallvec","version":"1.13.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/smallvec-1.13.2/src/lib.rs
//     evidence={"module":4378,"record_offset":14352,"parent_record_offset":14316,"ancestor_record_offset":11560,"inlinee":59979,"source_line":1716,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] smallvec::SmallVec::len
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"smallvec","version":"1.13.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/smallvec-1.13.2/src/lib.rs
//     evidence={"module":4378,"record_offset":14100,"parent_record_offset":14044,"ancestor_record_offset":11560,"inlinee":34970,"source_line":952,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] smallvec::SmallVec::set_len
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"smallvec","version":"1.13.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/smallvec-1.13.2/src/lib.rs
//     evidence={"module":4378,"record_offset":14256,"parent_record_offset":14044,"ancestor_record_offset":11560,"inlinee":34979,"source_line":919,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] smallvec::SmallVec::spilled
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"smallvec","version":"1.13.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/smallvec-1.13.2/src/lib.rs
//     evidence={"module":4378,"record_offset":13256,"parent_record_offset":13232,"ancestor_record_offset":11560,"inlinee":18839,"source_line":1001,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] smallvec::SmallVec::triple
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"smallvec","version":"1.13.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/smallvec-1.13.2/src/lib.rs
//     evidence={"module":4378,"record_offset":14148,"parent_record_offset":14100,"ancestor_record_offset":11560,"inlinee":34971,"source_line":971,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] smallvec::SmallVec::triple_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"smallvec","version":"1.13.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/smallvec-1.13.2/src/lib.rs
//     evidence={"module":4378,"record_offset":13232,"parent_record_offset":13116,"ancestor_record_offset":11560,"inlinee":21556,"source_line":984,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] smallvec::SmallVecData::heap_mut
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"smallvec","version":"1.13.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/smallvec-1.13.2/src/lib.rs
//     evidence={"module":4378,"record_offset":13168,"parent_record_offset":13116,"ancestor_record_offset":11560,"inlinee":21558,"source_line":727,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] smallvec::impl$33::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"smallvec","version":"1.13.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/smallvec-1.13.2/src/lib.rs
//     evidence={"module":4378,"record_offset":15060,"parent_record_offset":15036,"ancestor_record_offset":11560,"inlinee":18837,"source_line":2118,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] smallvec::impl$43::drop
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"smallvec","version":"1.13.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/smallvec-1.13.2/src/lib.rs
//     evidence={"module":4378,"record_offset":14948,"parent_record_offset":14920,"ancestor_record_offset":11560,"inlinee":34929,"source_line":2228,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] smallvec::impl$44::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"smallvec","version":"1.13.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/smallvec-1.13.2/src/lib.rs
//     evidence={"module":4378,"record_offset":14996,"parent_record_offset":14972,"ancestor_record_offset":11560,"inlinee":34931,"source_line":2237,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::ui_systems::handle_splitting_roof_decorators::handle_moving_out_roof_decorator_part::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/ui_systems/handle_splitting_roof_decorators.rs
//     evidence={"module":4378,"record_offset":13692,"parent_record_offset":13668,"ancestor_record_offset":11560,"inlinee":120864,"source_line":270,"archive":"_pdb/A/modules/04378.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::ui_systems::handle_splitting_roof_decorators::handle_moving_out_roof_decorator_part::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 270
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1D820
// 首条位置: {"module":4378,"inlinee":120864}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::ui_systems::handle_splitting_roof_decorators::handle_moving_out_roof_decorator_part::closure$1
// 记录形式: compiled_procedure; instances=1
// 源码行: 310
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":4378,"record_offset":15960,"rva":31886576,"va":5400595696}
// 编译实例 RVAs: 0x1e68cf0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1406bb180
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: handle_splitting_roof_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-24828885","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400595696,"raw_hex":"e82b2485fe","size":5,"soft_fail":false,"target_va":5375766912,"va":5400595792,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::wall::valid_enclosure::ShapeParameters::as_rectangle
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-24870813","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400595696,"raw_hex":"e8638084fe","size":5,"soft_fail":false,"target_va":5375724992,"va":5400595800,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::wall::valid_enclosure::ShapeParameters::perimeter
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-24871153","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400595696,"raw_hex":"e80f7f84fe","size":5,"soft_fail":false,"target_va":5375724768,"va":5400595916,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::edge_length
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-22628525","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400595696,"raw_hex":"e853b7a6fe","size":5,"soft_fail":false,"target_va":5377967328,"va":5400595848,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::get_closest_edge_id
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-22634706","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400595696,"raw_hex":"e82e9fa6fe","size":5,"soft_fail":false,"target_va":5377961136,"va":5400595837,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
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
// 完整结构及原始类型记录: handle_splitting_roof_decorators.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] system_decorator::ui_systems::handle_splitting_roof_decorators::handle_moving_out_roof_decorator_part::FracturedPart
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 1
// [PDB 泛型关系，非字段关系] {"type": "array$<system_decorator::ui_systems::handle_splitting_roof_decorators::handle_moving_out_roof_decorator_part::FracturedPart,2>", "constructor": "array$", "arguments": ["system_decorator::ui_systems::handle_splitting_roof_decorators::handle_moving_out_roof_decorator_part::FracturedPart", "2"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "smallvec::IntoIter<array$<system_decorator::ui_systems::handle_splitting_roof_decorators::handle_moving_out_roof_decorator_part::FracturedPart,2> >", "constructor": "smallvec::IntoIter", "arguments": ["array$<system_decorator::ui_systems::handle_splitting_roof_decorators::handle_moving_out_roof_decorator_part::FracturedPart,2>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<smallvec::IntoIter<array$<system_decorator::ui_systems::handle_splitting_roof_decorators::handle_moving_out_roof_decorator_part::FracturedPart,2> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["smallvec::IntoIter<array$<system_decorator::ui_systems::handle_splitting_roof_decorators::handle_moving_out_roof_decorator_part::FracturedPart,2> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: handle_splitting_roof_decorators.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: handle_splitting_roof_decorators.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: system_decorator::ui_systems::handle_splitting_roof_decorators::display_roof_decorator_parts_and_get_hovered_part_id
// 信息 ID: c6d253aa80f9a1d8652bbdca
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::ui_systems::handle_splitting_roof_decorators::try_display_roof_decorator_parts_and_get_hovered_part_id; module=4378, inline_record=3516; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::ui_systems::handle_splitting_roof_decorators::display_roof_decorator_parts_and_get_hovered_part_id::closure$0
// 信息 ID: 04a8ac586827e4b72864d7fb
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_decorator::ui_systems::handle_splitting_roof_decorators::display_roof_decorator_parts_and_get_hovered_part_id", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::ui_systems::handle_splitting_roof_decorators::try_display_roof_decorator_parts_and_get_hovered_part_id; module=4378, inline_record=4856; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::ui_systems::handle_splitting_roof_decorators::display_roof_decorator_parts_and_get_hovered_part_id::closure$1
// 信息 ID: 40a92f0cde4f6956dce91795
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "system_decorator::ui_systems::handle_splitting_roof_decorators::display_roof_decorator_parts_and_get_hovered_part_id", "captures": "unknown"}
// [闭包环境类型引用] system_decorator::ui_systems::handle_splitting_roof_decorators::display_roof_decorator_parts_and_get_hovered_part_id::closure_env$1 <- core::ops::function::FnOnce::call_once<system_decorator::ui_systems::handle_splitting_roof_decorators::display_roof_decorator_parts_and_get_hovered_part_id::closure_env$1,tuple$<ref$<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,ref$<country_core::asset_libraries::shader_library::AssetShaderLibrary>,ref_mut$<bevy_ecs::system::commands::Commands> > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 1 条。
// [静态直接引用] core::ops::function::FnOnce::call_once<system_decorator::ui_systems::handle_splitting_roof_decorators::display_roof_decorator_parts_and_get_hovered_part_id::closure_env$1,tuple$<ref$<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,ref$<country_core::asset_libraries::shader_library::AssetShaderLibrary>,ref_mut$<bevy_ecs::system::commands::Commands> > > | core::ops::function::FnOnce::call_once<system_decorator::ui_systems::handle_splitting_wall_decorators::display_decorator_wall_parts_and_get_hovered_part_id::closure_env$0,tuple$<ref$<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,ref$<country_core::asset_libraries::shader_library::AssetShaderLibrary>,ref_mut$<bevy_ecs::system::commands::Commands> > > @ 0x1e66c02 -> 0x1e679f0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::ui_systems::handle_splitting_roof_decorators::display_roof_decorator_parts_and_get_hovered_part_id::closure$2
// 信息 ID: 4f86b62401b614d4a941406e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "system_decorator::ui_systems::handle_splitting_roof_decorators::display_roof_decorator_parts_and_get_hovered_part_id", "captures": "unknown"}
// [闭包环境类型引用] system_decorator::ui_systems::handle_splitting_roof_decorators::display_roof_decorator_parts_and_get_hovered_part_id::closure_env$2 <- core::ops::function::FnOnce::call_once<system_decorator::ui_systems::handle_splitting_roof_decorators::display_roof_decorator_parts_and_get_hovered_part_id::closure_env$2,tuple$<ref$<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,ref$<country_core::asset_libraries::shader_library::AssetShaderLibrary>,ref_mut$<bevy_ecs::system::commands::Commands> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_decorator::ui_systems::handle_splitting_roof_decorators::display_roof_decorator_parts_and_get_hovered_part_id::closure_env$2 <- core::ptr::drop_in_place<system_decorator::ui_systems::handle_splitting_roof_decorators::display_roof_decorator_parts_and_get_hovered_part_id::closure_env$2>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 2 条。
// [静态直接引用] core::ops::function::FnOnce::call_once<system_decorator::ui_systems::handle_splitting_roof_decorators::display_roof_decorator_parts_and_get_hovered_part_id::closure_env$2,tuple$<ref$<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,ref$<country_core::asset_libraries::shader_library::AssetShaderLibrary>,ref_mut$<bevy_ecs::system::commands::Commands> > > | core::ops::function::FnOnce::call_once<system_decorator::ui_systems::handle_splitting_wall_decorators::display_decorator_wall_parts_and_get_hovered_part_id::closure_env$1,tuple$<ref$<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,ref$<country_core::asset_libraries::shader_library::AssetShaderLibrary>,ref_mut$<bevy_ecs::system::commands::Commands> > > @ 0x1e66c22 -> 0x1e67bb0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::ui_systems::handle_splitting_roof_decorators::display_roof_decorator_parts_and_get_hovered_part_id::closure$3
// 信息 ID: db6ce2f35da54739c5b5ea6e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 3, "lexical_owner": "system_decorator::ui_systems::handle_splitting_roof_decorators::display_roof_decorator_parts_and_get_hovered_part_id", "captures": "unknown"}
// [闭包环境类型引用] system_decorator::ui_systems::handle_splitting_roof_decorators::display_roof_decorator_parts_and_get_hovered_part_id::closure_env$3 <- core::ops::function::FnOnce::call_once<system_decorator::ui_systems::handle_splitting_roof_decorators::display_roof_decorator_parts_and_get_hovered_part_id::closure_env$3,tuple$<ref$<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,ref$<country_core::asset_libraries::shader_library::AssetShaderLibrary>,ref_mut$<bevy_ecs::system::commands::Commands> > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 1 条。
// [静态直接引用] core::ops::function::FnOnce::call_once<system_decorator::ui_systems::handle_splitting_roof_decorators::display_roof_decorator_parts_and_get_hovered_part_id::closure_env$3,tuple$<ref$<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,ref$<country_core::asset_libraries::shader_library::AssetShaderLibrary>,ref_mut$<bevy_ecs::system::commands::Commands> > > | core::ops::function::FnOnce::call_once<system_decorator::ui_systems::handle_splitting_wall_decorators::display_decorator_wall_parts_and_get_hovered_part_id::closure_env$2,tuple$<ref$<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,ref$<country_core::asset_libraries::shader_library::AssetShaderLibrary>,ref_mut$<bevy_ecs::system::commands::Commands> > > @ 0x1e66c42 -> 0x1e67d70 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::ui_systems::handle_splitting_roof_decorators::handle_moving_out_roof_decorator_part
// 信息 ID: 92751afda432de6d733a5b82
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::ui_systems::ui_move_decorator::ui_move_decorator @ 0x1c76fe4 -> 0x1e67f30 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::ui_systems::handle_splitting_roof_decorators::handle_moving_out_roof_decorator_part::closure$0
// 信息 ID: 16b6180409ad562132a5d605
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_decorator::ui_systems::handle_splitting_roof_decorators::handle_moving_out_roof_decorator_part", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::ui_systems::handle_splitting_roof_decorators::handle_moving_out_roof_decorator_part; module=4378, inline_record=13692; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::ui_systems::handle_splitting_roof_decorators::handle_moving_out_roof_decorator_part::closure$1
// 信息 ID: 05c7a6eb26ad5b46fc29e56c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "system_decorator::ui_systems::handle_splitting_roof_decorators::handle_moving_out_roof_decorator_part", "captures": "unknown"}
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::ui_systems::handle_splitting_roof_decorators::handle_moving_out_roof_decorator_part @ 0x1e68543 -> 0x1e68cf0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::handle_splitting_roof_decorators::handle_moving_out_roof_decorator_part @ 0x1e686e0 -> 0x1e68cf0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::handle_splitting_roof_decorators::handle_moving_out_roof_decorator_part @ 0x1e68859 -> 0x1e68cf0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::ui_systems::handle_splitting_roof_decorators::try_display_roof_decorator_parts_and_get_hovered_part_id
// 信息 ID: 415bad6cadfba201be47fd17
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::ui_systems::ui_move_decorator::ui_move_decorator @ 0x1c76ad5 -> 0x1e66c50 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
