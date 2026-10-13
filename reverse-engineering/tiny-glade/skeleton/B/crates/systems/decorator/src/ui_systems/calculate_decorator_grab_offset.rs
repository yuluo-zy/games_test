// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/systems/decorator/src/ui_systems/calculate_decorator_grab_offset.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: 444383ea8b7c64bd257512f14db02e603dbe7c659d8547c80f606176c977529c
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: calculate_decorator_grab_offset.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 1516
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
// use nonmax; // namespace candidate
//   package identities: [{"name":"nonmax","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.5.5"}]
// use system_decorator; // namespace candidate
//   package identities: [{"name":"system-decorator","source":"local_or_path","version":"0.1.0"}]
// use utils; // namespace candidate
//   package identities: [{"name":"utils","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_decorator::ui_systems::calculate_decorator_grab_offset::try_get_grab_offset_ss_if_valid
// 记录形式: compiled_procedure; instances=1
// 源码行: 27
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1516,"record_offset":376,"rva":19364784,"va":5388073904}
// 编译实例 RVAs: 0x1277bb0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t23466711","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388073904,"raw_hex":"e8d7126601","size":5,"soft_fail":false,"target_va":5411541776,"va":5388075060,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t23467827","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388073904,"raw_hex":"e833176601","size":5,"soft_fail":false,"target_va":5411542880,"va":5388075048,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator_storage::DecoratorStorageAddr::as_wall
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-8764182","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388073904,"raw_hex":"e8ea447aff","size":5,"soft_fail":false,"target_va":5379310144,"va":5388074321,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_walls::PublicWalls::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-8281720","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388073904,"raw_hex":"e888a181ff","size":5,"soft_fail":false,"target_va":5379792624,"va":5388074339,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_walls::WallEntities::wall_entity
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-8282020","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388073904,"raw_hex":"e85ca081ff","size":5,"soft_fail":false,"target_va":5379792384,"va":5388074399,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::wall_trigger_volume::WallTriggerVolume::get_position_from_mesh_uv
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-10010064","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388073904,"raw_hex":"e8304267ff","size":5,"soft_fail":false,"target_va":5378064544,"va":5388074603,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::window::WindowSize::as_vec2
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-9625644","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388073904,"raw_hex":"e8d41f6dff","size":5,"soft_fail":false,"target_va":5378448400,"va":5388074039,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::utils::world_space_to_screen_space
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-8058067","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388073904,"raw_hex":"e82d0b85ff","size":5,"soft_fail":false,"target_va":5380016000,"va":5388074062,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_decorator::ui_systems::calculate_decorator_grab_offset::raycast_with_ss_offset
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t938","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388073904,"raw_hex":"e8aa030000","size":5,"soft_fail":false,"target_va":5388075088,"va":5388074145,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::wall_space::WallSpaceCoord::to_world_space
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-6184605","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388073904,"raw_hex":"e863a1a1ff","size":5,"soft_fail":false,"target_va":5381890192,"va":5388074792,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::system::query::Query::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":1516,"record_offset":784,"parent_record_offset":376,"ancestor_record_offset":376,"inlinee":4837,"source_line":1385,"archive":"_pdb/B/modules/01516.jsonl.gz"}
//   [pdb_inline_direct] core::bool::impl$0::then_some
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\bool.rs
//     evidence={"module":1516,"record_offset":3232,"parent_record_offset":376,"ancestor_record_offset":376,"inlinee":19342,"source_line":33,"archive":"_pdb/B/modules/01516.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1516,"record_offset":684,"parent_record_offset":376,"ancestor_record_offset":376,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/01516.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1516,"record_offset":2384,"parent_record_offset":376,"ancestor_record_offset":376,"inlinee":4197,"source_line":1017,"archive":"_pdb/B/modules/01516.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1516,"record_offset":3344,"parent_record_offset":376,"ancestor_record_offset":376,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/01516.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::stairs::stairs_state::impl$16::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/stairs/stairs_state.rs
//     evidence={"module":1516,"record_offset":3260,"parent_record_offset":376,"ancestor_record_offset":376,"inlinee":4501,"source_line":38,"archive":"_pdb/B/modules/01516.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator::impl$23::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":1516,"record_offset":656,"parent_record_offset":376,"ancestor_record_offset":376,"inlinee":24851,"source_line":21,"archive":"_pdb/B/modules/01516.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::impl$9::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":1516,"record_offset":3204,"parent_record_offset":376,"ancestor_record_offset":376,"inlinee":4919,"source_line":69,"archive":"_pdb/B/modules/01516.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::public_wall_state::PublicWallState::is_premade_shape
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_wall_state.rs
//     evidence={"module":1516,"record_offset":720,"parent_record_offset":376,"ancestor_record_offset":376,"inlinee":24411,"source_line":113,"archive":"_pdb/B/modules/01516.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1516,"record_offset":592,"parent_record_offset":376,"ancestor_record_offset":376,"inlinee":9561,"source_line":1418,"archive":"_pdb/B/modules/01516.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1516,"record_offset":1132,"parent_record_offset":1108,"ancestor_record_offset":376,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/01516.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1516,"record_offset":1180,"parent_record_offset":1156,"ancestor_record_offset":376,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/01516.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1516,"record_offset":1156,"parent_record_offset":1132,"ancestor_record_offset":376,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/01516.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1516,"record_offset":1108,"parent_record_offset":1084,"ancestor_record_offset":376,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/01516.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1516,"record_offset":1084,"parent_record_offset":984,"ancestor_record_offset":376,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/01516.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1516,"record_offset":1520,"parent_record_offset":1496,"ancestor_record_offset":376,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/01516.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::impl$25::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":1516,"record_offset":1284,"parent_record_offset":952,"ancestor_record_offset":376,"inlinee":4846,"source_line":76,"archive":"_pdb/B/modules/01516.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::Entities::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/entity/mod.rs
//     evidence={"module":1516,"record_offset":952,"parent_record_offset":900,"ancestor_record_offset":376,"inlinee":4840,"source_line":842,"archive":"_pdb/B/modules/01516.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::StorageSwitch::extract
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":1516,"record_offset":2232,"parent_record_offset":2208,"ancestor_record_offset":376,"inlinee":4876,"source_line":2472,"archive":"_pdb/B/modules/01516.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$26::set_archetype
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":1516,"record_offset":1756,"parent_record_offset":900,"ancestor_record_offset":376,"inlinee":4856,"source_line":1160,"archive":"_pdb/B/modules/01516.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$26::set_table
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":1516,"record_offset":1780,"parent_record_offset":1756,"ancestor_record_offset":376,"inlinee":4857,"source_line":1175,"archive":"_pdb/B/modules/01516.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$27::fetch
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":1516,"record_offset":2208,"parent_record_offset":900,"ancestor_record_offset":376,"inlinee":4874,"source_line":1229,"archive":"_pdb/B/modules/01516.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$27::fetch::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":1516,"record_offset":2256,"parent_record_offset":2232,"ancestor_record_offset":376,"inlinee":4878,"source_line":1235,"archive":"_pdb/B/modules/01516.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_array::BlobArray::get_sub_slice
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_array.rs
//     evidence={"module":1516,"record_offset":2156,"parent_record_offset":2132,"ancestor_record_offset":376,"inlinee":4872,"source_line":143,"archive":"_pdb/B/modules/01516.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseArray::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":1516,"record_offset":1880,"parent_record_offset":1856,"ancestor_record_offset":376,"inlinee":4864,"source_line":58,"archive":"_pdb/B/modules/01516.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":1516,"record_offset":1856,"parent_record_offset":1832,"ancestor_record_offset":376,"inlinee":4862,"source_line":395,"archive":"_pdb/B/modules/01516.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$16::get::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":1516,"record_offset":2016,"parent_record_offset":1992,"ancestor_record_offset":376,"inlinee":4866,"source_line":396,"archive":"_pdb/B/modules/01516.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_column
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":1516,"record_offset":1832,"parent_record_offset":1804,"ancestor_record_offset":376,"inlinee":4860,"source_line":475,"archive":"_pdb/B/modules/01516.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_data_slice_for
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":1516,"record_offset":1804,"parent_record_offset":1780,"ancestor_record_offset":376,"inlinee":4859,"source_line":363,"archive":"_pdb/B/modules/01516.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":1516,"record_offset":1496,"parent_record_offset":900,"ancestor_record_offset":376,"inlinee":4854,"source_line":721,"archive":"_pdb/B/modules/01516.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::column::ThinColumn::get_data_slice
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/column.rs
//     evidence={"module":1516,"record_offset":2132,"parent_record_offset":2108,"ancestor_record_offset":376,"inlinee":4870,"source_line":303,"archive":"_pdb/B/modules/01516.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::impl$4::get_data_slice_for::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":1516,"record_offset":2108,"parent_record_offset":2084,"ancestor_record_offset":376,"inlinee":4868,"source_line":368,"archive":"_pdb/B/modules/01516.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::as_readonly
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":1516,"record_offset":816,"parent_record_offset":784,"ancestor_record_offset":376,"inlinee":13705,"source_line":542,"archive":"_pdb/B/modules/01516.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::copy_unsafe
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":1516,"record_offset":864,"parent_record_offset":840,"ancestor_record_offset":376,"inlinee":13707,"source_line":636,"archive":"_pdb/B/modules/01516.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::get_inner
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":1516,"record_offset":900,"parent_record_offset":784,"ancestor_record_offset":376,"inlinee":4838,"source_line":1591,"archive":"_pdb/B/modules/01516.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::reborrow_unsafe
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":1516,"record_offset":840,"parent_record_offset":816,"ancestor_record_offset":376,"inlinee":13706,"source_line":618,"archive":"_pdb/B/modules/01516.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ptr::ThinSlicePtr::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ptr","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ptr-0.16.0/src/lib.rs
//     evidence={"module":1516,"record_offset":2280,"parent_record_offset":2256,"ancestor_record_offset":376,"inlinee":4880,"source_line":526,"archive":"_pdb/B/modules/01516.jsonl.gz"}
//   [pdb_inline_transitive] core::bool::impl$0::then
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\bool.rs
//     evidence={"module":1516,"record_offset":1344,"parent_record_offset":1316,"ancestor_record_offset":376,"inlinee":4781,"source_line":61,"archive":"_pdb/B/modules/01516.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$24::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1516,"record_offset":1252,"parent_record_offset":1228,"ancestor_record_offset":376,"inlinee":4844,"source_line":1742,"archive":"_pdb/B/modules/01516.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1516,"record_offset":2896,"parent_record_offset":2476,"ancestor_record_offset":376,"inlinee":11327,"source_line":33,"archive":"_pdb/B/modules/01516.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ss
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1516,"record_offset":2804,"parent_record_offset":2748,"ancestor_record_offset":376,"inlinee":11326,"source_line":21,"archive":"_pdb/B/modules/01516.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1516,"record_offset":2572,"parent_record_offset":2548,"ancestor_record_offset":376,"inlinee":11323,"source_line":81,"archive":"_pdb/B/modules/01516.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1516,"record_offset":2604,"parent_record_offset":2520,"ancestor_record_offset":376,"inlinee":11324,"source_line":1016,"archive":"_pdb/B/modules/01516.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_sub_ps
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1516,"record_offset":2668,"parent_record_offset":2476,"ancestor_record_offset":376,"inlinee":11325,"source_line":57,"archive":"_pdb/B/modules/01516.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::impl$2::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":1516,"record_offset":1228,"parent_record_offset":952,"ancestor_record_offset":376,"inlinee":4842,"source_line":200,"archive":"_pdb/B/modules/01516.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1516,"record_offset":3136,"parent_record_offset":3108,"ancestor_record_offset":376,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/01516.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1516,"record_offset":620,"parent_record_offset":592,"ancestor_record_offset":376,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/01516.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":1516,"record_offset":1928,"parent_record_offset":1904,"ancestor_record_offset":376,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/01516.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1516,"record_offset":1904,"parent_record_offset":1880,"ancestor_record_offset":376,"inlinee":4518,"source_line":1447,"archive":"_pdb/B/modules/01516.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1516,"record_offset":1952,"parent_record_offset":1928,"ancestor_record_offset":376,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/01516.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1516,"record_offset":1992,"parent_record_offset":1856,"ancestor_record_offset":376,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/01516.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1516,"record_offset":1448,"parent_record_offset":1416,"ancestor_record_offset":376,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/01516.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1516,"record_offset":2348,"parent_record_offset":900,"ancestor_record_offset":376,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/01516.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":1516,"record_offset":984,"parent_record_offset":952,"ancestor_record_offset":376,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/01516.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_noubcheck
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1516,"record_offset":1052,"parent_record_offset":1024,"ancestor_record_offset":376,"inlinee":4164,"source_line":87,"archive":"_pdb/B/modules/01516.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1516,"record_offset":1024,"parent_record_offset":984,"ancestor_record_offset":376,"inlinee":4145,"source_line":226,"archive":"_pdb/B/modules/01516.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator::impl$29::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":1516,"record_offset":3284,"parent_record_offset":3260,"ancestor_record_offset":376,"inlinee":4664,"source_line":155,"archive":"_pdb/B/modules/01516.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::contains
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":1516,"record_offset":1316,"parent_record_offset":900,"ancestor_record_offset":376,"inlinee":4848,"source_line":392,"archive":"_pdb/B/modules/01516.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::contains_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":1516,"record_offset":1392,"parent_record_offset":1368,"ancestor_record_offset":376,"inlinee":4851,"source_line":408,"archive":"_pdb/B/modules/01516.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::get_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":1516,"record_offset":1416,"parent_record_offset":1392,"ancestor_record_offset":376,"inlinee":4852,"source_line":171,"archive":"_pdb/B/modules/01516.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::impl$2::contains::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":1516,"record_offset":1368,"parent_record_offset":1344,"ancestor_record_offset":376,"inlinee":4850,"source_line":395,"archive":"_pdb/B/modules/01516.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::mul_vec3
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/quat.rs
//     evidence={"module":1516,"record_offset":2452,"parent_record_offset":2420,"ancestor_record_offset":376,"inlinee":11318,"source_line":711,"archive":"_pdb/B/modules/01516.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::mul_vec3a
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/quat.rs
//     evidence={"module":1516,"record_offset":2476,"parent_record_offset":2452,"ancestor_record_offset":376,"inlinee":11319,"source_line":788,"archive":"_pdb/B/modules/01516.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::cross
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec3a.rs
//     evidence={"module":1516,"record_offset":2924,"parent_record_offset":2476,"ancestor_record_offset":376,"inlinee":11329,"source_line":229,"archive":"_pdb/B/modules/01516.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1516,"record_offset":3108,"parent_record_offset":2420,"ancestor_record_offset":376,"inlinee":4752,"source_line":1333,"archive":"_pdb/B/modules/01516.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_in_x
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/sse2.rs
//     evidence={"module":1516,"record_offset":2548,"parent_record_offset":2520,"ancestor_record_offset":376,"inlinee":11322,"source_line":42,"archive":"_pdb/B/modules/01516.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_into_m128
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/sse2.rs
//     evidence={"module":1516,"record_offset":2520,"parent_record_offset":2476,"ancestor_record_offset":376,"inlinee":11321,"source_line":66,"archive":"_pdb/B/modules/01516.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nonmax","version":"0.5.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":1516,"record_offset":2040,"parent_record_offset":2016,"ancestor_record_offset":376,"inlinee":4171,"source_line":153,"archive":"_pdb/B/modules/01516.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::ui_systems::calculate_decorator_grab_offset::try_get_grab_offset_ss_if_valid::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/ui_systems/calculate_decorator_grab_offset.rs
//     evidence={"module":1516,"record_offset":2420,"parent_record_offset":2384,"ancestor_record_offset":376,"inlinee":71493,"source_line":52,"archive":"_pdb/B/modules/01516.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::ui_systems::calculate_decorator_grab_offset::try_get_grab_offset_ss_if_valid::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 52
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x11745
// 首条位置: {"module":1516,"inlinee":71493}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::sse2::quat::Quat::mul_vec3
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/quat.rs
//     evidence={"module":1516,"record_offset":2452,"parent_record_offset":2420,"ancestor_record_offset":2420,"inlinee":11318,"source_line":711,"archive":"_pdb/B/modules/01516.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1516,"record_offset":3108,"parent_record_offset":2420,"ancestor_record_offset":2420,"inlinee":4752,"source_line":1333,"archive":"_pdb/B/modules/01516.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1516,"record_offset":2896,"parent_record_offset":2476,"ancestor_record_offset":2420,"inlinee":11327,"source_line":33,"archive":"_pdb/B/modules/01516.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ss
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1516,"record_offset":2804,"parent_record_offset":2748,"ancestor_record_offset":2420,"inlinee":11326,"source_line":21,"archive":"_pdb/B/modules/01516.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1516,"record_offset":2572,"parent_record_offset":2548,"ancestor_record_offset":2420,"inlinee":11323,"source_line":81,"archive":"_pdb/B/modules/01516.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1516,"record_offset":2604,"parent_record_offset":2520,"ancestor_record_offset":2420,"inlinee":11324,"source_line":1016,"archive":"_pdb/B/modules/01516.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_sub_ps
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1516,"record_offset":2668,"parent_record_offset":2476,"ancestor_record_offset":2420,"inlinee":11325,"source_line":57,"archive":"_pdb/B/modules/01516.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1516,"record_offset":3136,"parent_record_offset":3108,"ancestor_record_offset":2420,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/01516.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::mul_vec3a
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/quat.rs
//     evidence={"module":1516,"record_offset":2476,"parent_record_offset":2452,"ancestor_record_offset":2420,"inlinee":11319,"source_line":788,"archive":"_pdb/B/modules/01516.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::cross
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec3a.rs
//     evidence={"module":1516,"record_offset":2924,"parent_record_offset":2476,"ancestor_record_offset":2420,"inlinee":11329,"source_line":229,"archive":"_pdb/B/modules/01516.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_in_x
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/sse2.rs
//     evidence={"module":1516,"record_offset":2548,"parent_record_offset":2520,"ancestor_record_offset":2420,"inlinee":11322,"source_line":42,"archive":"_pdb/B/modules/01516.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_into_m128
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/sse2.rs
//     evidence={"module":1516,"record_offset":2520,"parent_record_offset":2476,"ancestor_record_offset":2420,"inlinee":11321,"source_line":66,"archive":"_pdb/B/modules/01516.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::ui_systems::calculate_decorator_grab_offset::raycast_with_ss_offset
// 记录形式: compiled_procedure; instances=1
// 源码行: 166
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1516,"record_offset":3404,"rva":19365968,"va":5388075088}
// 编译实例 RVAs: 0x1278050
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1408cc630
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: calculate_decorator_grab_offset.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10140463","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388075088,"raw_hex":"e8d14465ff","size":5,"soft_fail":false,"target_va":5377934896,"va":5388075354,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1412773f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: calculate_decorator_grab_offset.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3468","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388075088,"raw_hex":"e874f2ffff","size":5,"soft_fail":false,"target_va":5388071920,"va":5388075383,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::window::WindowSize::as_vec2
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-9626777","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388075088,"raw_hex":"e8671b6dff","size":5,"soft_fail":false,"target_va":5378448400,"va":5388075172,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::collision::world::Raycasting::raycast
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-10141215","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388075088,"raw_hex":"e8e14165ff","size":5,"soft_fail":false,"target_va":5377934048,"va":5388075258,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::ray::Ray::from_mouse_ss
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t18917","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388075088,"raw_hex":"e8e5490000","size":5,"soft_fail":false,"target_va":5388094128,"va":5388075206,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1516,"record_offset":3696,"parent_record_offset":3404,"ancestor_record_offset":3404,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/01516.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1516,"record_offset":3760,"parent_record_offset":3404,"ancestor_record_offset":3404,"inlinee":6345,"source_line":1218,"archive":"_pdb/B/modules/01516.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1516,"record_offset":4036,"parent_record_offset":3404,"ancestor_record_offset":3404,"inlinee":4666,"source_line":998,"archive":"_pdb/B/modules/01516.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1516,"record_offset":3592,"parent_record_offset":3404,"ancestor_record_offset":3404,"inlinee":4702,"source_line":1283,"archive":"_pdb/B/modules/01516.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::ui_systems::ui_decorator_affordance_dispatch::decorator_raycast_flags
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/ui_systems/ui_decorator_affordance_dispatch.rs
//     evidence={"module":1516,"record_offset":3668,"parent_record_offset":3404,"ancestor_record_offset":3404,"inlinee":25049,"source_line":818,"archive":"_pdb/B/modules/01516.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1516,"record_offset":3616,"parent_record_offset":3592,"ancestor_record_offset":3404,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/01516.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1516,"record_offset":3876,"parent_record_offset":3848,"ancestor_record_offset":3404,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/01516.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":1516,"record_offset":3728,"parent_record_offset":3696,"ancestor_record_offset":3404,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/01516.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1516,"record_offset":3848,"parent_record_offset":3816,"ancestor_record_offset":3404,"inlinee":4590,"source_line":1245,"archive":"_pdb/B/modules/01516.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1516,"record_offset":3940,"parent_record_offset":3816,"ancestor_record_offset":3404,"inlinee":4752,"source_line":1333,"archive":"_pdb/B/modules/01516.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::ui_systems::calculate_decorator_grab_offset::raycast_with_ss_offset::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/ui_systems/calculate_decorator_grab_offset.rs
//     evidence={"module":1516,"record_offset":3816,"parent_record_offset":3760,"ancestor_record_offset":3404,"inlinee":71496,"source_line":189,"archive":"_pdb/B/modules/01516.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::ui_systems::calculate_decorator_grab_offset::raycast_with_ss_offset::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/ui_systems/calculate_decorator_grab_offset.rs
//     evidence={"module":1516,"record_offset":3788,"parent_record_offset":3760,"ancestor_record_offset":3404,"inlinee":71495,"source_line":196,"archive":"_pdb/B/modules/01516.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::ui_systems::calculate_decorator_grab_offset::raycast_with_ss_offset::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 189
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x11748
// 首条位置: {"module":1516,"inlinee":71496}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1516,"record_offset":3848,"parent_record_offset":3816,"ancestor_record_offset":3816,"inlinee":4590,"source_line":1245,"archive":"_pdb/B/modules/01516.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1516,"record_offset":3940,"parent_record_offset":3816,"ancestor_record_offset":3816,"inlinee":4752,"source_line":1333,"archive":"_pdb/B/modules/01516.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1516,"record_offset":3968,"parent_record_offset":3940,"ancestor_record_offset":3816,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/01516.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1516,"record_offset":3876,"parent_record_offset":3848,"ancestor_record_offset":3816,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/01516.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::ui_systems::calculate_decorator_grab_offset::raycast_with_ss_offset::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 196
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x11747
// 首条位置: {"module":1516,"inlinee":71495}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
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
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: system_decorator::ui_systems::calculate_decorator_grab_offset::raycast_with_ss_offset
// 信息 ID: 57fe0cbee5243c527c63dc52
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::ui_systems::ui_move_decorator::ui_move_decorator @ 0x7a6468 -> 0x1278050 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::calculate_decorator_grab_offset::try_get_grab_offset_ss_if_valid @ 0x1277ca1 -> 0x1278050 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::ui_systems::calculate_decorator_grab_offset::raycast_with_ss_offset::closure$0
// 信息 ID: 50cced3aa69ad695c9dc02a7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_decorator::ui_systems::calculate_decorator_grab_offset::raycast_with_ss_offset", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::ui_systems::calculate_decorator_grab_offset::raycast_with_ss_offset; module=1516, inline_record=3816; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::ui_systems::calculate_decorator_grab_offset::raycast_with_ss_offset::closure$1
// 信息 ID: c78f0d89a7ebf0b49bf7dc62
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "system_decorator::ui_systems::calculate_decorator_grab_offset::raycast_with_ss_offset", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::ui_systems::calculate_decorator_grab_offset::raycast_with_ss_offset; module=1516, inline_record=3788; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::ui_systems::calculate_decorator_grab_offset::try_get_grab_offset_ss_if_valid
// 信息 ID: e8b7ed101e91f44d36753faa
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::ui_systems::ui_move_decorator::ui_move_decorator @ 0x7a6378 -> 0x1277bb0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::ui_move_decorator::ui_move_decorator @ 0x7a53c2 -> 0x1277bb0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::ui_systems::calculate_decorator_grab_offset::try_get_grab_offset_ss_if_valid::closure$0
// 信息 ID: a365a56ed5304766baf66478
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_decorator::ui_systems::calculate_decorator_grab_offset::try_get_grab_offset_ss_if_valid", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::ui_systems::calculate_decorator_grab_offset::try_get_grab_offset_ss_if_valid; module=1516, inline_record=2420; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
