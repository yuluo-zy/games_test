// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/systems/decorator/src/cull_oob_decorators.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: f77f6a9427ced10bfbdb215aa416b6f95032b0a5611f678b886f893d466ffef3
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: cull_oob_decorators.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 383, 1276, 1284, 1286, 2742, 2767, 2802, 3832
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
// use itertools; // namespace candidate
//   package identities: [{"name":"itertools","source":"PDB_registry_path_only","version":"0.10.5"}]
// use nonmax; // namespace candidate
//   package identities: [{"name":"nonmax","source":"PDB_registry_path_only","version":"0.5.5"}]
// use puffin; // namespace candidate
//   package identities: [{"name":"puffin","source":"PDB_registry_path_only","version":"0.14.1"}]
// use std; // namespace candidate
//   package identities: []
// use system_decorator; // namespace candidate
//   package identities: []
// use utils; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_decorator::cull_oob_decorators::check_if_dormer_ridge_is_within_roof_bounds
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":383,"record_offset":31712,"rva":5252400,"va":5373961520}
// 编译实例 RVAs: 0x502530
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: cull_oob_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1212282","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373961520,"raw_hex":"e88680edff","size":5,"soft_fail":false,"target_va":5372749712,"va":5373961989,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1407da760
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: cull_oob_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2981662","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373961520,"raw_hex":"e81e7f2d00","size":5,"soft_fail":false,"target_va":5376943968,"va":5373962301,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1407da780
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: cull_oob_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2981840","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373961520,"raw_hex":"e8d07f2d00","size":5,"soft_fail":false,"target_va":5376944000,"va":5373962155,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t29017466","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373961520,"raw_hex":"e87ac5ba01","size":5,"soft_fail":false,"target_va":5402979344,"va":5373961873,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::gothic_window_profile::gothic_half_dormer_profile_curve
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t531041","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373961520,"raw_hex":"e8611a0800","size":5,"soft_fail":false,"target_va":5374492896,"va":5373961850,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::roofs::roof_shape::Roof::height
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1801318","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373961520,"raw_hex":"e8667c1b00","size":5,"soft_fail":false,"target_va":5375763504,"va":5373962181,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::roofs::roof_shape::Roof::roof_shape_at_intersection_y
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1802671","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373961520,"raw_hex":"e8af811b00","size":5,"soft_fail":false,"target_va":5375764720,"va":5373962044,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_walls::PublicWalls::roof_entity
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1474399","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373961520,"raw_hex":"e85f7f1600","size":5,"soft_fail":false,"target_va":5375436032,"va":5373961628,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_decorator::decorator_visual::gothic_dormer::gothic_dormer_profile_curve
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t8435187","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373961520,"raw_hex":"e8f3b58000","size":5,"soft_fail":false,"target_va":5382397136,"va":5373961944,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::circle::Circle2d::closest_point_on_perimeter
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4006795","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373961520,"raw_hex":"e88b233d00","size":5,"soft_fail":false,"target_va":5377968928,"va":5373962128,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::closest_point_on_perimeter
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t3999403","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373961520,"raw_hex":"e8ab063d00","size":5,"soft_fail":false,"target_va":5377961504,"va":5373962096,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::impl$12::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":383,"record_offset":32920,"parent_record_offset":31712,"ancestor_record_offset":31712,"inlinee":5450,"source_line":2828,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::and_then
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":383,"record_offset":31908,"parent_record_offset":31712,"ancestor_record_offset":31712,"inlinee":4151,"source_line":1408,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":383,"record_offset":33100,"parent_record_offset":31712,"ancestor_record_offset":31712,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":383,"record_offset":33848,"parent_record_offset":31712,"ancestor_record_offset":31712,"inlinee":14697,"source_line":157,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::normalize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":383,"record_offset":33612,"parent_record_offset":31712,"ancestor_record_offset":31712,"inlinee":21371,"source_line":450,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$12::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":383,"record_offset":33428,"parent_record_offset":31712,"ancestor_record_offset":31712,"inlinee":14692,"source_line":1081,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$17::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":383,"record_offset":33516,"parent_record_offset":31712,"ancestor_record_offset":31712,"inlinee":21370,"source_line":1135,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$9::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":383,"record_offset":33372,"parent_record_offset":31712,"ancestor_record_offset":31712,"inlinee":14672,"source_line":1048,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":383,"record_offset":33272,"parent_record_offset":33248,"ancestor_record_offset":31712,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":383,"record_offset":33248,"parent_record_offset":33156,"ancestor_record_offset":31712,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":383,"record_offset":33184,"parent_record_offset":33156,"ancestor_record_offset":31712,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":383,"record_offset":33156,"parent_record_offset":33132,"ancestor_record_offset":31712,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::impl$24::eq
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/archetype.rs
//     evidence={"module":383,"record_offset":32176,"parent_record_offset":32032,"ancestor_record_offset":31712,"inlinee":21390,"source_line":74,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::Entities::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/entity/mod.rs
//     evidence={"module":383,"record_offset":32032,"parent_record_offset":31988,"ancestor_record_offset":31712,"inlinee":21387,"source_line":739,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$21::fetch
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":383,"record_offset":32820,"parent_record_offset":31988,"ancestor_record_offset":31712,"inlinee":14627,"source_line":963,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$21::set_archetype
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":383,"record_offset":32452,"parent_record_offset":31988,"ancestor_record_offset":31712,"inlinee":21391,"source_line":933,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$21::set_table
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":383,"record_offset":32476,"parent_record_offset":32452,"ancestor_record_offset":31712,"inlinee":14620,"source_line":948,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::state::QueryState::get_unchecked_manual
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/state.rs
//     evidence={"module":383,"record_offset":31988,"parent_record_offset":31960,"ancestor_record_offset":31712,"inlinee":21385,"source_line":859,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_slice
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":383,"record_offset":32780,"parent_record_offset":32756,"ancestor_record_offset":31712,"inlinee":14643,"source_line":410,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseArray::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":383,"record_offset":32552,"parent_record_offset":32528,"ancestor_record_offset":31712,"inlinee":6338,"source_line":56,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":383,"record_offset":32528,"parent_record_offset":32504,"ancestor_record_offset":31712,"inlinee":6337,"source_line":369,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$11::get::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":383,"record_offset":32600,"parent_record_offset":32576,"ancestor_record_offset":31712,"inlinee":6340,"source_line":58,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$16::get::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":383,"record_offset":32688,"parent_record_offset":32664,"ancestor_record_offset":31712,"inlinee":6342,"source_line":370,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Column::get_data_slice
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":383,"record_offset":32756,"parent_record_offset":32476,"ancestor_record_offset":31712,"inlinee":14642,"source_line":321,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_column
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":383,"record_offset":32504,"parent_record_offset":32476,"ancestor_record_offset":31712,"inlinee":14621,"source_line":692,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":383,"record_offset":32424,"parent_record_offset":31988,"ancestor_record_offset":31712,"inlinee":14618,"source_line":834,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/query.rs
//     evidence={"module":383,"record_offset":31960,"parent_record_offset":31936,"ancestor_record_offset":31712,"inlinee":21384,"source_line":846,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ptr::ThinSlicePtr::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ptr/src/lib.rs
//     evidence={"module":383,"record_offset":32844,"parent_record_offset":32820,"ancestor_record_offset":31712,"inlinee":21393,"source_line":500,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] core::bool::impl$0::then
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\bool.rs
//     evidence={"module":383,"record_offset":32240,"parent_record_offset":32208,"ancestor_record_offset":31712,"inlinee":4886,"source_line":59,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$24::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":383,"record_offset":32144,"parent_record_offset":32120,"ancestor_record_offset":31712,"inlinee":13915,"source_line":1471,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":383,"record_offset":33212,"parent_record_offset":33184,"ancestor_record_offset":31712,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::impl$2::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":383,"record_offset":32120,"parent_record_offset":32032,"ancestor_record_offset":31712,"inlinee":21388,"source_line":136,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":383,"record_offset":33396,"parent_record_offset":33372,"ancestor_record_offset":31712,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":383,"record_offset":33456,"parent_record_offset":33428,"ancestor_record_offset":31712,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":383,"record_offset":33552,"parent_record_offset":33516,"ancestor_record_offset":31712,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":383,"record_offset":32624,"parent_record_offset":32600,"ancestor_record_offset":31712,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":383,"record_offset":32576,"parent_record_offset":32552,"ancestor_record_offset":31712,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":383,"record_offset":33132,"parent_record_offset":33100,"ancestor_record_offset":31712,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":383,"record_offset":32348,"parent_record_offset":32316,"ancestor_record_offset":31712,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":383,"record_offset":32396,"parent_record_offset":31988,"ancestor_record_offset":31712,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":383,"record_offset":32064,"parent_record_offset":32032,"ancestor_record_offset":31712,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":383,"record_offset":32944,"parent_record_offset":32920,"ancestor_record_offset":31712,"inlinee":5452,"source_line":17,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":383,"record_offset":32088,"parent_record_offset":32064,"ancestor_record_offset":31712,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":383,"record_offset":32968,"parent_record_offset":32944,"ancestor_record_offset":31712,"inlinee":5453,"source_line":262,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::contains
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":383,"record_offset":32208,"parent_record_offset":31988,"ancestor_record_offset":31712,"inlinee":4884,"source_line":392,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::contains_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":383,"record_offset":32292,"parent_record_offset":32268,"ancestor_record_offset":31712,"inlinee":4889,"source_line":408,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::get_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":383,"record_offset":32316,"parent_record_offset":32292,"ancestor_record_offset":31712,"inlinee":4890,"source_line":171,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::impl$2::contains::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":383,"record_offset":32268,"parent_record_offset":32240,"ancestor_record_offset":31712,"inlinee":4888,"source_line":395,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":383,"record_offset":33712,"parent_record_offset":33676,"ancestor_record_offset":31712,"inlinee":14697,"source_line":157,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":383,"record_offset":33676,"parent_record_offset":33640,"ancestor_record_offset":31712,"inlinee":21373,"source_line":380,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_recip
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":383,"record_offset":33640,"parent_record_offset":33612,"ancestor_record_offset":31712,"inlinee":21372,"source_line":399,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nonmax","version":"0.5.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":383,"record_offset":32712,"parent_record_offset":32688,"ancestor_record_offset":31712,"inlinee":4372,"source_line":153,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":383,"record_offset":33748,"parent_record_offset":33712,"ancestor_record_offset":31712,"inlinee":13701,"source_line":441,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::cull_oob_decorators::check_if_dormer_ridge_is_within_roof_bounds::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/cull_oob_decorators.rs
//     evidence={"module":383,"record_offset":31936,"parent_record_offset":31908,"ancestor_record_offset":31712,"inlinee":21757,"source_line":843,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::cull_oob_decorators::is_decorator_location_valid
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":383,"record_offset":21116,"rva":5244992,"va":5373954112}
// 编译实例 RVAs: 0x500840
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: cull_oob_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1207920","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373954112,"raw_hex":"e89091edff","size":5,"soft_fail":false,"target_va":5372749712,"va":5373957627,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14051aa00
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: cull_oob_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t101740","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373954112,"raw_hex":"e86c8d0100","size":5,"soft_fail":false,"target_va":5374061056,"va":5373959311,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140593250
//     owner=unresolved_namespace; count=3
//     target_aliases=15 candidates; complete list: cull_oob_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t600095","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373954112,"raw_hex":"e81f280900","size":5,"soft_fail":false,"target_va":5374554704,"va":5373954604,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140597160
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: cull_oob_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t613495","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373954112,"raw_hex":"e8775c0900","size":5,"soft_fail":false,"target_va":5374570848,"va":5373957348,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140633130
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: cull_oob_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1255152","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373954112,"raw_hex":"e8f0261300","size":5,"soft_fail":false,"target_va":5375209776,"va":5373954619,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1407da780
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: cull_oob_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2988334","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373954112,"raw_hex":"e82e992d00","size":5,"soft_fail":false,"target_va":5376944000,"va":5373955661,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140ceb4e0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: cull_oob_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8299882","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373954112,"raw_hex":"e86aa57e00","size":5,"soft_fail":false,"target_va":5382255840,"va":5373955953,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140ceb560
//     owner=unresolved_namespace; count=7
//     target_aliases=2 candidates; complete list: cull_oob_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8300913","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373954112,"raw_hex":"e871a97e00","size":5,"soft_fail":false,"target_va":5382255968,"va":5373955050,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140ceb6a0
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: cull_oob_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8296866","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373954112,"raw_hex":"e8a2997e00","size":5,"soft_fail":false,"target_va":5382256288,"va":5373959417,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140cf31e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: cull_oob_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8328903","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373954112,"raw_hex":"e8c7167f00","size":5,"soft_fail":false,"target_va":5382287840,"va":5373958932,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d06720
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: cull_oob_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8412354","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373954112,"raw_hex":"e8c25c8000","size":5,"soft_fail":false,"target_va":5382367008,"va":5373954649,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d06920
//     owner=unresolved_namespace; count=3
//     target_aliases=3 candidates; complete list: cull_oob_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8412347","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373954112,"raw_hex":"e8bb5c8000","size":5,"soft_fail":false,"target_va":5382367520,"va":5373955168,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d06c10
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: cull_oob_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8409782","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373954112,"raw_hex":"e8b6528000","size":5,"soft_fail":false,"target_va":5382368272,"va":5373958485,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d06e90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: cull_oob_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8410511","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373954112,"raw_hex":"e88f558000","size":5,"soft_fail":false,"target_va":5382368912,"va":5373958396,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d07010
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: cull_oob_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8410910","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373954112,"raw_hex":"e81e578000","size":5,"soft_fail":false,"target_va":5382369296,"va":5373958381,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::query::state::QueryState$LT$D$C$F$GT$::get_unchecked_manual
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t8418873","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373954112,"raw_hex":"e839768000","size":5,"soft_fail":false,"target_va":5382377600,"va":5373958722,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=5
//     evidence={"assembly":"\tcallq\t29017992","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373954112,"raw_hex":"e888c7ba01","size":5,"soft_fail":false,"target_va":5402978880,"va":5373960883,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t29018240","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373954112,"raw_hex":"e880c8ba01","size":5,"soft_fail":false,"target_va":5402979152,"va":5373960907,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=8
//     evidence={"assembly":"\tcallq\t29018723","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373954112,"raw_hex":"e863caba01","size":5,"soft_fail":false,"target_va":5402979344,"va":5373960616,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t29017903","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373954112,"raw_hex":"e82fc7ba01","size":5,"soft_fail":false,"target_va":5402978944,"va":5373961036,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t29019016","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373954112,"raw_hex":"e888cbba01","size":5,"soft_fail":false,"target_va":5402980128,"va":5373961107,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::gothic_window_profile::gothic_half_dormer_profile_curve
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t535321","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373954112,"raw_hex":"e8192b0800","size":5,"soft_fail":false,"target_va":5374492896,"va":5373957570,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::roofs::roof_shape::Roof::gable_sides
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t1806210","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373954112,"raw_hex":"e8828f1b00","size":5,"soft_fail":false,"target_va":5375763616,"va":5373957401,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::roofs::roof_shape::Roof::is_flat
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t1806215","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373954112,"raw_hex":"e8878f1b00","size":5,"soft_fail":false,"target_va":5375763552,"va":5373957332,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::roofs::roof_shape::Roof::is_gable
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t1806203","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373954112,"raw_hex":"e87b8f1b00","size":5,"soft_fail":false,"target_va":5375763568,"va":5373957360,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms::transform
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t407984","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373954112,"raw_hex":"e8b0390600","size":5,"soft_fail":false,"target_va":5374362576,"va":5373954587,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator::DecoratorType::is_window
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t610990","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373954112,"raw_hex":"e8ae520900","size":5,"soft_fail":false,"target_va":5374570816,"va":5373959821,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator_storage::DecoratorStorageAddr::as_wall
//     owner=unresolved_namespace; count=7
//     evidence={"assembly":"\tcallq\t762357","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373954112,"raw_hex":"e8f5a10b00","size":5,"soft_fail":false,"target_va":5374716704,"va":5373954342,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo::subtype
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t1258138","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373954112,"raw_hex":"e89a321300","size":5,"soft_fail":false,"target_va":5375212688,"va":5373954545,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_wall_state::PublicWallState::is_rectangle
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t7973930","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373954112,"raw_hex":"e82aac7900","size":5,"soft_fail":false,"target_va":5381933120,"va":5373959185,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_wall_state::PublicWallState::max_y
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t7973152","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373954112,"raw_hex":"e820a97900","size":5,"soft_fail":false,"target_va":5381932864,"va":5373959707,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_wall_state::PublicWallState::wall_space
//     owner=unresolved_namespace; count=12
//     evidence={"assembly":"\tcallq\t7978048","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373954112,"raw_hex":"e840bc7900","size":5,"soft_fail":false,"target_va":5381933056,"va":5373955003,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_walls::PublicWalls::get
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1479988","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373954112,"raw_hex":"e834951600","size":5,"soft_fail":false,"target_va":5375434976,"va":5373954983,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_walls::PublicWalls::roof_entity
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t1481674","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373954112,"raw_hex":"e8ca9b1600","size":5,"soft_fail":false,"target_va":5375436032,"va":5373954353,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::wall::valid_enclosure::ShapeParameters::as_rectangle
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1765424","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373954112,"raw_hex":"e830f01a00","size":5,"soft_fail":false,"target_va":5375724992,"va":5373959563,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_decorator::cull_oob_decorators::check_if_dormer_ridge_is_within_roof_bounds
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t5606","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373954112,"raw_hex":"e8e6150000","size":5,"soft_fail":false,"target_va":5373961520,"va":5373955909,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_decorator::decorator_visual::cottage_dormers::cottage_dormer::is_out_of_roof_bounds
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t8345380","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373954112,"raw_hex":"e824577f00","size":5,"soft_fail":false,"target_va":5382301920,"va":5373956535,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_decorator::ui_systems::decorator_interaction_intent::snap_balcony_door
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t8428183","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373954112,"raw_hex":"e8979a8000","size":5,"soft_fail":false,"target_va":5382387328,"va":5373959140,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::get_closest_edge_id
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4001497","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373954112,"raw_hex":"e8d90e3d00","size":5,"soft_fail":false,"target_va":5377961136,"va":5373959634,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::get_point_u
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t3996582","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373954112,"raw_hex":"e8a6fb3c00","size":5,"soft_fail":false,"target_va":5377955840,"va":5373959253,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::wall_space::WallSpaceCoord::to_world_space
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t8300894","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373954112,"raw_hex":"e85ea97e00","size":5,"soft_fail":false,"target_va":5382256592,"va":5373955693,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::system::query::Query::get
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/query.rs
//     evidence={"module":383,"record_offset":21316,"parent_record_offset":21116,"ancestor_record_offset":21116,"inlinee":21384,"source_line":846,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_direct] core::array::iter::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\array\iter.rs
//     evidence={"module":383,"record_offset":31332,"parent_record_offset":21116,"ancestor_record_offset":21116,"inlinee":6222,"source_line":239,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_direct] core::ops::range::Range::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\range.rs
//     evidence={"module":383,"record_offset":31204,"parent_record_offset":21116,"ancestor_record_offset":21116,"inlinee":10010,"source_line":120,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":383,"record_offset":31168,"parent_record_offset":21116,"ancestor_record_offset":21116,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":383,"record_offset":31500,"parent_record_offset":21116,"ancestor_record_offset":21116,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo::bbx_ls
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":383,"record_offset":22260,"parent_record_offset":21116,"ancestor_record_offset":21116,"inlinee":21725,"source_line":267,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::quat::Quat::mul_vec3
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/quat.rs
//     evidence={"module":383,"record_offset":29460,"parent_record_offset":21116,"ancestor_record_offset":21116,"inlinee":14676,"source_line":659,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::normalize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":383,"record_offset":30948,"parent_record_offset":21116,"ancestor_record_offset":21116,"inlinee":21371,"source_line":450,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::normalize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":383,"record_offset":30108,"parent_record_offset":21116,"ancestor_record_offset":21116,"inlinee":21580,"source_line":508,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$12::add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":383,"record_offset":29200,"parent_record_offset":21116,"ancestor_record_offset":21116,"inlinee":14692,"source_line":1081,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$9::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":383,"record_offset":29144,"parent_record_offset":21116,"ancestor_record_offset":21116,"inlinee":14672,"source_line":1048,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::cull_oob_decorators::is_wall_decorator_location_valid
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/cull_oob_decorators.rs
//     evidence={"module":383,"record_offset":22396,"parent_record_offset":21116,"ancestor_record_offset":21116,"inlinee":21729,"source_line":457,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle3d::height
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":383,"record_offset":29116,"parent_record_offset":21116,"ancestor_record_offset":21116,"inlinee":21736,"source_line":65,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":383,"record_offset":26068,"parent_record_offset":26032,"ancestor_record_offset":21116,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":383,"record_offset":26032,"parent_record_offset":25888,"ancestor_record_offset":21116,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":383,"record_offset":25932,"parent_record_offset":25888,"ancestor_record_offset":21116,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":383,"record_offset":25888,"parent_record_offset":25856,"ancestor_record_offset":21116,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$12::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":383,"record_offset":24148,"parent_record_offset":24116,"ancestor_record_offset":21116,"inlinee":5450,"source_line":2828,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::impl$24::eq
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/archetype.rs
//     evidence={"module":383,"record_offset":21532,"parent_record_offset":21388,"ancestor_record_offset":21116,"inlinee":21390,"source_line":74,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::Entities::get
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/entity/mod.rs
//     evidence={"module":383,"record_offset":21388,"parent_record_offset":21344,"ancestor_record_offset":21116,"inlinee":21387,"source_line":739,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$21::fetch
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":383,"record_offset":22168,"parent_record_offset":21344,"ancestor_record_offset":21116,"inlinee":14627,"source_line":963,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$21::set_archetype
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":383,"record_offset":21800,"parent_record_offset":21344,"ancestor_record_offset":21116,"inlinee":21391,"source_line":933,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$21::set_table
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":383,"record_offset":21824,"parent_record_offset":21800,"ancestor_record_offset":21116,"inlinee":14620,"source_line":948,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::state::QueryState::get_unchecked_manual
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/state.rs
//     evidence={"module":383,"record_offset":21344,"parent_record_offset":21316,"ancestor_record_offset":21116,"inlinee":21385,"source_line":859,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_slice
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":383,"record_offset":22128,"parent_record_offset":22104,"ancestor_record_offset":21116,"inlinee":14643,"source_line":410,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseArray::get
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":383,"record_offset":21900,"parent_record_offset":21876,"ancestor_record_offset":21116,"inlinee":6338,"source_line":56,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":383,"record_offset":21876,"parent_record_offset":21852,"ancestor_record_offset":21116,"inlinee":6337,"source_line":369,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$11::get::closure$0
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":383,"record_offset":21948,"parent_record_offset":21924,"ancestor_record_offset":21116,"inlinee":6340,"source_line":58,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$16::get::closure$0
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":383,"record_offset":22036,"parent_record_offset":22012,"ancestor_record_offset":21116,"inlinee":6342,"source_line":370,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Column::get_data_slice
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":383,"record_offset":22104,"parent_record_offset":21824,"ancestor_record_offset":21116,"inlinee":14642,"source_line":321,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_column
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":383,"record_offset":21852,"parent_record_offset":21824,"ancestor_record_offset":21116,"inlinee":14621,"source_line":692,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":383,"record_offset":21772,"parent_record_offset":21344,"ancestor_record_offset":21116,"inlinee":14618,"source_line":834,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/query.rs
//     evidence={"module":383,"record_offset":26428,"parent_record_offset":26404,"ancestor_record_offset":21116,"inlinee":21384,"source_line":846,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ptr::ThinSlicePtr::get
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ptr/src/lib.rs
//     evidence={"module":383,"record_offset":22192,"parent_record_offset":22168,"ancestor_record_offset":21116,"inlinee":21393,"source_line":500,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::impl$2::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\array\iter.rs
//     evidence={"module":383,"record_offset":31380,"parent_record_offset":31356,"ancestor_record_offset":21116,"inlinee":11996,"source_line":245,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] core::bool::impl$0::then
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\bool.rs
//     evidence={"module":383,"record_offset":21592,"parent_record_offset":21564,"ancestor_record_offset":21116,"inlinee":4886,"source_line":59,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":383,"record_offset":31260,"parent_record_offset":31232,"ancestor_record_offset":21116,"inlinee":6743,"source_line":1676,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$21::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":383,"record_offset":27192,"parent_record_offset":27168,"ancestor_record_offset":21116,"inlinee":19598,"source_line":1469,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$24::ne
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":383,"record_offset":21500,"parent_record_offset":21476,"ancestor_record_offset":21116,"inlinee":13915,"source_line":1471,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$50::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":383,"record_offset":31288,"parent_record_offset":31260,"ancestor_record_offset":21116,"inlinee":5484,"source_line":1517,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":383,"record_offset":29900,"parent_record_offset":29484,"ancestor_record_offset":21116,"inlinee":15697,"source_line":31,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ss
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":383,"record_offset":29808,"parent_record_offset":29752,"ancestor_record_offset":21116,"inlinee":21747,"source_line":20,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=17
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":383,"record_offset":29580,"parent_record_offset":29556,"ancestor_record_offset":21116,"inlinee":14684,"source_line":77,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":383,"record_offset":29612,"parent_record_offset":29528,"ancestor_record_offset":21116,"inlinee":14683,"source_line":1011,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_sub_ps
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":383,"record_offset":29672,"parent_record_offset":29484,"ancestor_record_offset":21116,"inlinee":14685,"source_line":54,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":383,"record_offset":23500,"parent_record_offset":23224,"ancestor_record_offset":21116,"inlinee":5741,"source_line":924,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":383,"record_offset":23060,"parent_record_offset":23036,"ancestor_record_offset":21116,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::Map::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":383,"record_offset":26232,"parent_record_offset":26208,"ancestor_record_offset":21116,"inlinee":4246,"source_line":69,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":383,"record_offset":26208,"parent_record_offset":22396,"ancestor_record_offset":21116,"inlinee":4244,"source_line":761,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_uninit::MaybeUninit::assume_init_read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\maybe_uninit.rs
//     evidence={"module":383,"record_offset":31404,"parent_record_offset":31380,"ancestor_record_offset":21116,"inlinee":6097,"source_line":697,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":383,"record_offset":25984,"parent_record_offset":25932,"ancestor_record_offset":21116,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::impl$2::ne
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":383,"record_offset":21476,"parent_record_offset":21388,"ancestor_record_offset":21116,"inlinee":21388,"source_line":136,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":383,"record_offset":24500,"parent_record_offset":24440,"ancestor_record_offset":21116,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":383,"record_offset":24528,"parent_record_offset":24440,"ancestor_record_offset":21116,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":383,"record_offset":24472,"parent_record_offset":24440,"ancestor_record_offset":21116,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\range.rs
//     evidence={"module":383,"record_offset":31232,"parent_record_offset":31204,"ancestor_record_offset":21116,"inlinee":6738,"source_line":817,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":383,"record_offset":26376,"parent_record_offset":22396,"ancestor_record_offset":21116,"inlinee":4151,"source_line":1408,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":383,"record_offset":21972,"parent_record_offset":21948,"ancestor_record_offset":21116,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":383,"record_offset":21924,"parent_record_offset":21900,"ancestor_record_offset":21116,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":383,"record_offset":23188,"parent_record_offset":22396,"ancestor_record_offset":21116,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":383,"record_offset":22324,"parent_record_offset":22296,"ancestor_record_offset":21116,"inlinee":4138,"source_line":972,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$40::branch
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":383,"record_offset":25348,"parent_record_offset":25320,"ancestor_record_offset":21116,"inlinee":5366,"source_line":2485,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":383,"record_offset":31428,"parent_record_offset":31404,"ancestor_record_offset":21116,"inlinee":6098,"source_line":1250,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":383,"record_offset":25812,"parent_record_offset":22396,"ancestor_record_offset":21116,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":383,"record_offset":21696,"parent_record_offset":21664,"ancestor_record_offset":21116,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":383,"record_offset":31452,"parent_record_offset":31428,"ancestor_record_offset":21116,"inlinee":4136,"source_line":1246,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::ok
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":383,"record_offset":26468,"parent_record_offset":26404,"ancestor_record_offset":21116,"inlinee":4772,"source_line":631,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":383,"record_offset":21744,"parent_record_offset":21344,"ancestor_record_offset":21116,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$10::slice_contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":383,"record_offset":27120,"parent_record_offset":27096,"ancestor_record_offset":21116,"inlinee":4965,"source_line":215,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$10::slice_contains::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":383,"record_offset":27168,"parent_record_offset":27144,"ancestor_record_offset":21116,"inlinee":4968,"source_line":216,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":383,"record_offset":27096,"parent_record_offset":22396,"ancestor_record_offset":21116,"inlinee":4963,"source_line":2524,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":383,"record_offset":21420,"parent_record_offset":21388,"ancestor_record_offset":21116,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":383,"record_offset":26124,"parent_record_offset":22396,"ancestor_record_offset":21116,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::last
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":383,"record_offset":25728,"parent_record_offset":22396,"ancestor_record_offset":21116,"inlinee":9339,"source_line":302,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":383,"record_offset":24180,"parent_record_offset":24148,"ancestor_record_offset":21116,"inlinee":5452,"source_line":17,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":383,"record_offset":21444,"parent_record_offset":21420,"ancestor_record_offset":21116,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":383,"record_offset":24212,"parent_record_offset":24180,"ancestor_record_offset":21116,"inlinee":5453,"source_line":262,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":383,"record_offset":26148,"parent_record_offset":26124,"ancestor_record_offset":21116,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":383,"record_offset":27144,"parent_record_offset":27120,"ancestor_record_offset":21116,"inlinee":4966,"source_line":279,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::DecoratorCollision::dims_wall_space
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":383,"record_offset":23096,"parent_record_offset":22976,"ancestor_record_offset":21116,"inlinee":21733,"source_line":102,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary::dims_wall_space
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":383,"record_offset":22976,"parent_record_offset":22932,"ancestor_record_offset":21116,"inlinee":21731,"source_line":217,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary::get_collision_bbx_ls
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":383,"record_offset":22296,"parent_record_offset":22260,"ancestor_record_offset":21116,"inlinee":21727,"source_line":199,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo::dims_wall_space
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":383,"record_offset":22932,"parent_record_offset":22396,"ancestor_record_offset":21116,"inlinee":21730,"source_line":261,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::impl$3::dims_wall_space::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":383,"record_offset":23036,"parent_record_offset":23004,"ancestor_record_offset":21116,"inlinee":21753,"source_line":220,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::impl$3::get_collision_bbx_ls::closure$0
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":383,"record_offset":22356,"parent_record_offset":22324,"ancestor_record_offset":21116,"inlinee":21755,"source_line":202,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::public_wall_state::PublicWallState::is_premade_shape
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_wall_state.rs
//     evidence={"module":383,"record_offset":26348,"parent_record_offset":22396,"ancestor_record_offset":21116,"inlinee":21375,"source_line":96,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::contains
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":383,"record_offset":21564,"parent_record_offset":21344,"ancestor_record_offset":21116,"inlinee":4884,"source_line":392,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::contains_unchecked
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":383,"record_offset":21640,"parent_record_offset":21616,"ancestor_record_offset":21116,"inlinee":4889,"source_line":408,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::get_unchecked
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":383,"record_offset":21664,"parent_record_offset":21640,"ancestor_record_offset":21116,"inlinee":4890,"source_line":171,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::impl$2::contains::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":383,"record_offset":21616,"parent_record_offset":21592,"ancestor_record_offset":21116,"inlinee":4888,"source_line":395,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::mul_vec3a
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/quat.rs
//     evidence={"module":383,"record_offset":29484,"parent_record_offset":29460,"ancestor_record_offset":21116,"inlinee":14681,"source_line":731,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::cross
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec3a.rs
//     evidence={"module":383,"record_offset":29928,"parent_record_offset":29484,"ancestor_record_offset":21116,"inlinee":14682,"source_line":220,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":383,"record_offset":25188,"parent_record_offset":25156,"ancestor_record_offset":21116,"inlinee":14697,"source_line":157,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":383,"record_offset":25156,"parent_record_offset":25124,"ancestor_record_offset":21116,"inlinee":21373,"source_line":380,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_recip
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":383,"record_offset":25124,"parent_record_offset":25100,"ancestor_record_offset":21116,"inlinee":21372,"source_line":399,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::normalize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":383,"record_offset":25100,"parent_record_offset":22396,"ancestor_record_offset":21116,"inlinee":21371,"source_line":450,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::perp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":383,"record_offset":25072,"parent_record_offset":22396,"ancestor_record_offset":21116,"inlinee":14690,"source_line":814,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":383,"record_offset":24932,"parent_record_offset":24900,"ancestor_record_offset":21116,"inlinee":13203,"source_line":195,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":383,"record_offset":24900,"parent_record_offset":24868,"ancestor_record_offset":21116,"inlinee":21582,"source_line":436,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length_recip
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":383,"record_offset":24868,"parent_record_offset":24844,"ancestor_record_offset":21116,"inlinee":21581,"source_line":455,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":383,"record_offset":24416,"parent_record_offset":24392,"ancestor_record_offset":21116,"inlinee":21436,"source_line":742,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::normalize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":383,"record_offset":24844,"parent_record_offset":24820,"ancestor_record_offset":21116,"inlinee":21580,"source_line":508,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$17::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":383,"record_offset":24440,"parent_record_offset":24416,"ancestor_record_offset":21116,"inlinee":21370,"source_line":1135,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_in_x
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/sse2.rs
//     evidence={"module":383,"record_offset":29556,"parent_record_offset":29528,"ancestor_record_offset":21116,"inlinee":21746,"source_line":42,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_into_m128
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/sse2.rs
//     evidence={"module":383,"record_offset":29528,"parent_record_offset":29484,"ancestor_record_offset":21116,"inlinee":21745,"source_line":66,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] itertools::Itertools::tuple_windows
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"itertools","version":"0.10.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/itertools-0.10.5/src/lib.rs
//     evidence={"module":383,"record_offset":26264,"parent_record_offset":22396,"ancestor_record_offset":21116,"inlinee":11990,"source_line":699,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"nonmax","version":"0.5.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":383,"record_offset":22060,"parent_record_offset":22036,"ancestor_record_offset":21116,"inlinee":4372,"source_line":153,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::abs
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":383,"record_offset":26896,"parent_record_offset":26752,"ancestor_record_offset":21116,"inlinee":15239,"source_line":198,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":383,"record_offset":24964,"parent_record_offset":24932,"ancestor_record_offset":21116,"inlinee":13701,"source_line":441,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::cull_oob_decorators::is_wall_decorator_location_valid::closure$0
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/cull_oob_decorators.rs
//     evidence={"module":383,"record_offset":23224,"parent_record_offset":22396,"ancestor_record_offset":21116,"inlinee":21738,"source_line":477,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::cull_oob_decorators::is_wall_decorator_location_valid::closure$3
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/cull_oob_decorators.rs
//     evidence={"module":383,"record_offset":26404,"parent_record_offset":26376,"ancestor_record_offset":21116,"inlinee":21748,"source_line":697,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::cull_oob_decorators::should_snap_to_corner
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/cull_oob_decorators.rs
//     evidence={"module":383,"record_offset":26752,"parent_record_offset":22396,"ancestor_record_offset":21116,"inlinee":21750,"source_line":899,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::get_coord_at_2d_ws_distance
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":383,"record_offset":23288,"parent_record_offset":23224,"ancestor_record_offset":21116,"inlinee":21741,"source_line":944,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::get_pos_at_coord
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":383,"record_offset":24116,"parent_record_offset":22396,"ancestor_record_offset":21116,"inlinee":21575,"source_line":622,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::get_tangent_at_coord
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":383,"record_offset":24572,"parent_record_offset":22396,"ancestor_record_offset":21116,"inlinee":21578,"source_line":637,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::get_unnormalized_tangent_at_coord
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":383,"record_offset":24600,"parent_record_offset":24572,"ancestor_record_offset":21116,"inlinee":21585,"source_line":633,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i1
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":383,"record_offset":24256,"parent_record_offset":24116,"ancestor_record_offset":21116,"inlinee":21359,"source_line":1113,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::CurveSlice::end
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":383,"record_offset":23468,"parent_record_offset":23288,"ancestor_record_offset":21116,"inlinee":21611,"source_line":335,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::CurveSlice::extend_right
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":383,"record_offset":23440,"parent_record_offset":23288,"ancestor_record_offset":21116,"inlinee":21485,"source_line":233,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::CurveSlice::from_curve_range
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":383,"record_offset":23324,"parent_record_offset":23288,"ancestor_record_offset":21116,"inlinee":21742,"source_line":86,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::CurveSlice::from_point_range
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":383,"record_offset":23352,"parent_record_offset":23324,"ancestor_record_offset":21116,"inlinee":21607,"source_line":107,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::CurveSlice::remap
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":383,"record_offset":23408,"parent_record_offset":23384,"ancestor_record_offset":21116,"inlinee":21484,"source_line":193,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::CurveSlice::remap_2d
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":383,"record_offset":23384,"parent_record_offset":23288,"ancestor_record_offset":21116,"inlinee":21483,"source_line":217,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::impl$12::sample_at_coord
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":383,"record_offset":25444,"parent_record_offset":22396,"ancestor_record_offset":21116,"inlinee":21356,"source_line":1168,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle3d::height
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":383,"record_offset":23148,"parent_record_offset":23096,"ancestor_record_offset":21116,"inlinee":21736,"source_line":65,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle3d::width
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":383,"record_offset":23120,"parent_record_offset":23096,"ancestor_record_offset":21116,"inlinee":21735,"source_line":70,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$1::lerp
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":383,"record_offset":24392,"parent_record_offset":24116,"ancestor_record_offset":21116,"inlinee":21577,"source_line":106,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$1::normalize
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":383,"record_offset":24820,"parent_record_offset":24572,"ancestor_record_offset":21116,"inlinee":21579,"source_line":96,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] utils::wall_space::WallSpaceCoord::new
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/wall_space.rs
//     evidence={"module":383,"record_offset":26724,"parent_record_offset":22396,"ancestor_record_offset":21116,"inlinee":21749,"source_line":106,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] utils::wall_space::WallSpaceCoord::subtract
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/wall_space.rs
//     evidence={"module":383,"record_offset":26868,"parent_record_offset":26752,"ancestor_record_offset":21116,"inlinee":21751,"source_line":268,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] utils::wall_space::WallSpaceCoord::with_offset
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/wall_space.rs
//     evidence={"module":383,"record_offset":25320,"parent_record_offset":22396,"ancestor_record_offset":21116,"inlinee":21743,"source_line":197,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] utils::wall_space::WallSpaceCoord::with_offset_clamped
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/wall_space.rs
//     evidence={"module":383,"record_offset":23260,"parent_record_offset":23224,"ancestor_record_offset":21116,"inlinee":21740,"source_line":241,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::cull_oob_decorators::auto_allow_decorator_culling
// 记录形式: compiled_procedure; instances=1
// 源码行: 46
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":383,"record_offset":3312,"rva":5238944,"va":5373948064}
// 编译实例 RVAs: 0x4ff0a0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140cfd3a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: cull_oob_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8380922","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373948064,"raw_hex":"e8fae17f00","size":5,"soft_fail":false,"target_va":5382329248,"va":5373948321,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::history::History::is_recording
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1020556","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373948064,"raw_hex":"e88c920f00","size":5,"soft_fail":false,"target_va":5374968656,"va":5373948095,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator_storage::DecoratorStorage::wall_decorator_ids
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t770101","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373948064,"raw_hex":"e835c00b00","size":5,"soft_fail":false,"target_va":5374718416,"va":5373948310,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":383,"record_offset":4336,"parent_record_offset":3312,"ancestor_record_offset":3312,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::EventReader::read
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":383,"record_offset":3476,"parent_record_offset":3312,"ancestor_record_offset":3312,"inlinee":6366,"source_line":468,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::impl$20::next
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":383,"record_offset":4016,"parent_record_offset":3312,"ancestor_record_offset":3312,"inlinee":9712,"source_line":770,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator_archivist::DecoratorArchivist::decorator_cull_allow
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_archivist.rs
//     evidence={"module":383,"record_offset":4392,"parent_record_offset":3312,"ancestor_record_offset":3312,"inlinee":21699,"source_line":243,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$29::set_changed
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":383,"record_offset":4360,"parent_record_offset":4336,"ancestor_record_offset":3312,"inlinee":5812,"source_line":309,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::EventIteratorWithId::new
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":383,"record_offset":3564,"parent_record_offset":3540,"ancestor_record_offset":3312,"inlinee":6371,"source_line":810,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::ManualEventReader::read_with_id
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":383,"record_offset":3540,"parent_record_offset":3500,"ancestor_record_offset":3312,"inlinee":6369,"source_line":722,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::impl$23::next
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":383,"record_offset":4044,"parent_record_offset":4016,"ancestor_record_offset":3312,"inlinee":9713,"source_line":842,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::system_param::impl$14::deref_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/system_param.rs
//     evidence={"module":383,"record_offset":3500,"parent_record_offset":3476,"ancestor_record_offset":3312,"inlinee":7509,"source_line":805,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::and_then_or_clear
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\chain.rs
//     evidence={"module":383,"record_offset":4100,"parent_record_offset":4076,"ancestor_record_offset":3312,"inlinee":8622,"source_line":299,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\chain.rs
//     evidence={"module":383,"record_offset":4076,"parent_record_offset":4044,"ancestor_record_offset":3312,"inlinee":8620,"source_line":50,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\chain.rs
//     evidence={"module":383,"record_offset":4180,"parent_record_offset":4156,"ancestor_record_offset":3312,"inlinee":8625,"source_line":51,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":383,"record_offset":3608,"parent_record_offset":3564,"ancestor_record_offset":3312,"inlinee":5655,"source_line":1547,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":383,"record_offset":4124,"parent_record_offset":4100,"ancestor_record_offset":3312,"inlinee":4215,"source_line":704,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":383,"record_offset":4156,"parent_record_offset":4076,"ancestor_record_offset":3312,"inlinee":5273,"source_line":1507,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":383,"record_offset":3908,"parent_record_offset":3884,"ancestor_record_offset":3312,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":383,"record_offset":4280,"parent_record_offset":4256,"ancestor_record_offset":3312,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":383,"record_offset":3672,"parent_record_offset":3564,"ancestor_record_offset":3312,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":383,"record_offset":3860,"parent_record_offset":3564,"ancestor_record_offset":3312,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":383,"record_offset":3732,"parent_record_offset":3708,"ancestor_record_offset":3312,"inlinee":7968,"source_line":353,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$6::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":383,"record_offset":3708,"parent_record_offset":3672,"ancestor_record_offset":3312,"inlinee":8098,"source_line":479,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":383,"record_offset":3884,"parent_record_offset":3860,"ancestor_record_offset":3312,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":383,"record_offset":4256,"parent_record_offset":4232,"ancestor_record_offset":3312,"inlinee":5091,"source_line":101,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":383,"record_offset":4232,"parent_record_offset":4180,"ancestor_record_offset":3312,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::impl$11::extend
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\set.rs
//     evidence={"module":383,"record_offset":4440,"parent_record_offset":4416,"ancestor_record_offset":3312,"inlinee":18286,"source_line":1354,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::impl$9::extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\set.rs
//     evidence={"module":383,"record_offset":4416,"parent_record_offset":4392,"ancestor_record_offset":3312,"inlinee":18284,"source_line":1072,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::cull_oob_decorators::cull_oob_decorators
// 记录形式: compiled_procedure; instances=1
// 源码行: 66
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":383,"record_offset":4480,"rva":5239280,"va":5373948400}
// 编译实例 RVAs: 0x4ff1f0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dccd0
//     owner=unresolved_namespace; count=5
//     target_aliases=9 candidates; complete list: cull_oob_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4336158","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373948400,"raw_hex":"e8e2d5bdff","size":5,"soft_fail":false,"target_va":5369613520,"va":5373949673,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403bc080
//     owner=unresolved_namespace; count=2
//     target_aliases=16 candidates; complete list: cull_oob_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1324187","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373948400,"raw_hex":"e865cbebff","size":5,"soft_fail":false,"target_va":5372625024,"va":5373949206,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=5
//     target_aliases=1 candidates; complete list: cull_oob_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1202234","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373948400,"raw_hex":"e8c6a7edff","size":5,"soft_fail":false,"target_va":5372749712,"va":5373951941,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14051d490
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: cull_oob_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t120073","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373948400,"raw_hex":"e809d50100","size":5,"soft_fail":false,"target_va":5374071952,"va":5373951874,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405b9b40
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: cull_oob_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t761352","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373948400,"raw_hex":"e8089e0b00","size":5,"soft_fail":false,"target_va":5374712640,"va":5373951283,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405b9c30
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: cull_oob_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t763735","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373948400,"raw_hex":"e857a70b00","size":5,"soft_fail":false,"target_va":5374712880,"va":5373949140,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405bab80
//     owner=unresolved_namespace; count=2
//     target_aliases=4 candidates; complete list: cull_oob_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t765318","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373948400,"raw_hex":"e886ad0b00","size":5,"soft_fail":false,"target_va":5374716800,"va":5373951477,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140633eb0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: cull_oob_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1262479","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373948400,"raw_hex":"e88f431300","size":5,"soft_fail":false,"target_va":5375213232,"va":5373950748,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406d7130
//     owner=unresolved_namespace; count=2
//     target_aliases=14 candidates; complete list: cull_oob_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1928179","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373948400,"raw_hex":"e8f36b1d00","size":5,"soft_fail":false,"target_va":5375881520,"va":5373953336,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a6f6d0
//     owner=unresolved_namespace; count=3
//     target_aliases=7 candidates; complete list: cull_oob_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5701709","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373948400,"raw_hex":"e84d005700","size":5,"soft_fail":false,"target_va":5379651280,"va":5373949566,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140cf9940
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: cull_oob_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8364956","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373948400,"raw_hex":"e89ca37f00","size":5,"soft_fail":false,"target_va":5382314304,"va":5373949343,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140cfa350
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: cull_oob_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8364132","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373948400,"raw_hex":"e864a07f00","size":5,"soft_fail":false,"target_va":5382316880,"va":5373952743,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140cfd0b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: cull_oob_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8376481","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373948400,"raw_hex":"e8a1d07f00","size":5,"soft_fail":false,"target_va":5382328496,"va":5373952010,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140cfd440
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: cull_oob_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8377173","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373948400,"raw_hex":"e855d37f00","size":5,"soft_fail":false,"target_va":5382329408,"va":5373952230,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140cfebb0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: cull_oob_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8383275","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373948400,"raw_hex":"e82beb7f00","size":5,"soft_fail":false,"target_va":5382335408,"va":5373952128,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d01fb0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: cull_oob_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8394985","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373948400,"raw_hex":"e8e9188000","size":5,"soft_fail":false,"target_va":5382348720,"va":5373953730,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d04630
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: cull_oob_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8405754","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373948400,"raw_hex":"e8fa428000","size":5,"soft_fail":false,"target_va":5382358576,"va":5373952817,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: cull_oob_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t29640567","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373948400,"raw_hex":"e87747c401","size":5,"soft_fail":false,"target_va":5403593360,"va":5373952788,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t29025519","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373948400,"raw_hex":"e8efe4ba01","size":5,"soft_fail":false,"target_va":5402978688,"va":5373953164,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t29025751","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373948400,"raw_hex":"e8d7e5ba01","size":5,"soft_fail":false,"target_va":5402978880,"va":5373953124,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t29025997","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373948400,"raw_hex":"e8cde6ba01","size":5,"soft_fail":false,"target_va":5402979152,"va":5373953150,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<core::option::Option<puffin::ProfilerScope> > >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-6411","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373948400,"raw_hex":"e8f5e6ffff","size":5,"soft_fail":false,"target_va":5373946640,"va":5373953046,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t29031203","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373948400,"raw_hex":"e823fbba01","size":5,"soft_fail":false,"target_va":5402980128,"va":5373948920,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-1674","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373948400,"raw_hex":"e876f9ffff","size":5,"soft_fail":false,"target_va":5373946832,"va":5373948501,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t29032073","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373948400,"raw_hex":"e889feba01","size":5,"soft_fail":false,"target_va":5402980656,"va":5373948578,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::history::History::is_recording
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t1016611","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373948400,"raw_hex":"e823830f00","size":5,"soft_fail":false,"target_va":5374968656,"va":5373952040,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms::position
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t410023","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373948400,"raw_hex":"e8a7410600","size":5,"soft_fail":false,"target_va":5374362208,"va":5373952180,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator_archivist::DecoratorArchivist::is_decorator_cull_allowed
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t761808","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373948400,"raw_hex":"e8d09f0b00","size":5,"soft_fail":false,"target_va":5374713264,"va":5373951451,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2407821","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373948400,"raw_hex":"e88dbd2400","size":5,"soft_fail":false,"target_va":5376356912,"va":5373949086,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::__getit::__KEY$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t2411707","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373948400,"raw_hex":"e8bbcc2400","size":5,"soft_fail":false,"target_va":5376360560,"va":5373948848,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2407657","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373948400,"raw_hex":"e8e9bc2400","size":5,"soft_fail":false,"target_va":5376356144,"va":5373948482,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::hash::random::RandomState::new::KEYS::__getit::__KEY$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t3417035","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373948400,"raw_hex":"e8cb233400","size":5,"soft_fail":false,"target_va":5377366224,"va":5373949184,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::thread_local::fast_local::Key$LT$T$GT$::try_initialize::hcc99216871ec738f (.llvm.16000913382889057913)
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t8340935","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373948400,"raw_hex":"e8c7457f00","size":5,"soft_fail":false,"target_va":5382289808,"va":5373948868,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_decorator::apply_decorator_op::apply_decorator_op
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t8412595","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373948400,"raw_hex":"e8b35d8000","size":5,"soft_fail":false,"target_va":5382365568,"va":5373952968,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_decorator::cull_oob_decorators::is_decorator_location_valid
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2536","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373948400,"raw_hex":"e8e8090000","size":5,"soft_fail":false,"target_va":5373954112,"va":5373951571,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":383,"record_offset":18508,"parent_record_offset":4480,"ancestor_record_offset":4480,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::EventReader::read
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":383,"record_offset":8272,"parent_record_offset":4480,"ancestor_record_offset":4480,"inlinee":6366,"source_line":468,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::impl$20::next
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":383,"record_offset":8872,"parent_record_offset":4480,"ancestor_record_offset":4480,"inlinee":9712,"source_line":770,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":383,"record_offset":17116,"parent_record_offset":4480,"ancestor_record_offset":4480,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":383,"record_offset":7420,"parent_record_offset":4480,"ancestor_record_offset":4480,"inlinee":4249,"source_line":1999,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::filter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":383,"record_offset":7364,"parent_record_offset":4480,"ancestor_record_offset":4480,"inlinee":21075,"source_line":883,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_none
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":383,"record_offset":18452,"parent_record_offset":4480,"ancestor_record_offset":4480,"inlinee":4710,"source_line":651,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":383,"record_offset":18416,"parent_record_offset":4480,"ancestor_record_offset":4480,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":383,"record_offset":18620,"parent_record_offset":4480,"ancestor_record_offset":4480,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo::new
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":383,"record_offset":14024,"parent_record_offset":4480,"ancestor_record_offset":4480,"inlinee":21526,"source_line":249,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":383,"record_offset":6316,"parent_record_offset":4480,"ancestor_record_offset":4480,"inlinee":7309,"source_line":589,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":383,"record_offset":4728,"parent_record_offset":4480,"ancestor_record_offset":4480,"inlinee":7284,"source_line":626,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":383,"record_offset":5304,"parent_record_offset":4480,"ancestor_record_offset":4480,"inlinee":7291,"source_line":659,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::contains_key
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":383,"record_offset":9184,"parent_record_offset":4480,"ancestor_record_offset":4480,"inlinee":17300,"source_line":1042,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::insert
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":383,"record_offset":18592,"parent_record_offset":4480,"ancestor_record_offset":4480,"inlinee":8309,"source_line":1105,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":383,"record_offset":16136,"parent_record_offset":4480,"ancestor_record_offset":4480,"inlinee":8928,"source_line":518,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":383,"record_offset":14080,"parent_record_offset":4480,"ancestor_record_offset":4480,"inlinee":9059,"source_line":232,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$38::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":383,"record_offset":18820,"parent_record_offset":4480,"ancestor_record_offset":4480,"inlinee":21719,"source_line":2223,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$39::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":383,"record_offset":16468,"parent_record_offset":4480,"ancestor_record_offset":4480,"inlinee":8642,"source_line":2233,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$46::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":383,"record_offset":19536,"parent_record_offset":4480,"ancestor_record_offset":4480,"inlinee":21721,"source_line":2315,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::set::HashSet::insert
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\set.rs
//     evidence={"module":383,"record_offset":8816,"parent_record_offset":4480,"ancestor_record_offset":4480,"inlinee":8705,"source_line":889,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::set::impl$17::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\set.rs
//     evidence={"module":383,"record_offset":14552,"parent_record_offset":4480,"ancestor_record_offset":4480,"inlinee":21703,"source_line":1482,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::set::impl$23::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\set.rs
//     evidence={"module":383,"record_offset":15328,"parent_record_offset":4480,"ancestor_record_offset":4480,"inlinee":21707,"source_line":1541,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::blueprint::DecoratorBlueprintCollider::as_transformed_collider
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/blueprint.rs
//     evidence={"module":383,"record_offset":18200,"parent_record_offset":4480,"ancestor_record_offset":4480,"inlinee":21717,"source_line":120,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::blueprint::DecoratorBlueprints::get
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/blueprint.rs
//     evidence={"module":383,"record_offset":17172,"parent_record_offset":4480,"ancestor_record_offset":4480,"inlinee":21715,"source_line":260,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::blueprint::DecoratorBlueprints::remove
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/blueprint.rs
//     evidence={"module":383,"record_offset":20280,"parent_record_offset":4480,"ancestor_record_offset":4480,"inlinee":21722,"source_line":252,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::cull_oob_decorators::DecoratorLocationCheckResult::is_valid
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/cull_oob_decorators.rs
//     evidence={"module":383,"record_offset":17144,"parent_record_offset":4480,"ancestor_record_offset":4480,"inlinee":21713,"source_line":252,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":383,"record_offset":18368,"parent_record_offset":18344,"ancestor_record_offset":4480,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":383,"record_offset":18344,"parent_record_offset":18320,"ancestor_record_offset":4480,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":383,"record_offset":18320,"parent_record_offset":18296,"ancestor_record_offset":4480,"inlinee":4113,"source_line":1240,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":383,"record_offset":18240,"parent_record_offset":18200,"ancestor_record_offset":4480,"inlinee":4586,"source_line":2105,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$29::set_changed
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":383,"record_offset":18532,"parent_record_offset":18508,"ancestor_record_offset":4480,"inlinee":5812,"source_line":309,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::EventIteratorWithId::new
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":383,"record_offset":8360,"parent_record_offset":8336,"ancestor_record_offset":4480,"inlinee":6371,"source_line":810,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::ManualEventReader::read_with_id
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":383,"record_offset":8336,"parent_record_offset":8296,"ancestor_record_offset":4480,"inlinee":6369,"source_line":722,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::impl$23::next
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":383,"record_offset":8896,"parent_record_offset":8872,"ancestor_record_offset":4480,"inlinee":9713,"source_line":842,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::system_param::impl$14::deref_mut
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/system_param.rs
//     evidence={"module":383,"record_offset":8296,"parent_record_offset":8272,"ancestor_record_offset":4480,"inlinee":7509,"source_line":805,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":383,"record_offset":6716,"parent_record_offset":6688,"ancestor_record_offset":4480,"inlinee":4785,"source_line":1782,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":383,"record_offset":7740,"parent_record_offset":7712,"ancestor_record_offset":4480,"inlinee":5551,"source_line":518,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":383,"record_offset":6768,"parent_record_offset":6744,"ancestor_record_offset":4480,"inlinee":4790,"source_line":479,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":383,"record_offset":6744,"parent_record_offset":6716,"ancestor_record_offset":4480,"inlinee":4789,"source_line":410,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":383,"record_offset":6656,"parent_record_offset":6628,"ancestor_record_offset":4480,"inlinee":4782,"source_line":1058,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":383,"record_offset":6688,"parent_record_offset":6656,"ancestor_record_offset":4480,"inlinee":4783,"source_line":1090,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":383,"record_offset":6860,"parent_record_offset":6688,"ancestor_record_offset":4480,"inlinee":5305,"source_line":2122,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$42::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":383,"record_offset":6972,"parent_record_offset":6948,"ancestor_record_offset":4480,"inlinee":4787,"source_line":1773,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":383,"record_offset":9976,"parent_record_offset":9952,"ancestor_record_offset":4480,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":383,"record_offset":14048,"parent_record_offset":14024,"ancestor_record_offset":4480,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":383,"record_offset":14800,"parent_record_offset":14768,"ancestor_record_offset":4480,"inlinee":4898,"source_line":1225,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":383,"record_offset":9444,"parent_record_offset":9420,"ancestor_record_offset":4480,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":383,"record_offset":9528,"parent_record_offset":9504,"ancestor_record_offset":4480,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":383,"record_offset":9468,"parent_record_offset":9444,"ancestor_record_offset":4480,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::and_then_or_clear
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\chain.rs
//     evidence={"module":383,"record_offset":8948,"parent_record_offset":8924,"ancestor_record_offset":4480,"inlinee":8622,"source_line":299,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\chain.rs
//     evidence={"module":383,"record_offset":8924,"parent_record_offset":8896,"ancestor_record_offset":4480,"inlinee":8620,"source_line":50,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next::closure$0
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\chain.rs
//     evidence={"module":383,"record_offset":9028,"parent_record_offset":9004,"ancestor_record_offset":4480,"inlinee":8625,"source_line":51,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::cloned::impl$1::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\cloned.rs
//     evidence={"module":383,"record_offset":8072,"parent_record_offset":8048,"ancestor_record_offset":4480,"inlinee":11907,"source_line":57,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::filter::Filter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\filter.rs
//     evidence={"module":383,"record_offset":7388,"parent_record_offset":7364,"ancestor_record_offset":4480,"inlinee":21077,"source_line":25,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::filter::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\filter.rs
//     evidence={"module":383,"record_offset":8048,"parent_record_offset":8024,"ancestor_record_offset":4480,"inlinee":18206,"source_line":153,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":383,"record_offset":8024,"parent_record_offset":8000,"ancestor_record_offset":4480,"inlinee":4667,"source_line":125,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":383,"record_offset":8000,"parent_record_offset":7976,"ancestor_record_offset":4480,"inlinee":5220,"source_line":807,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":383,"record_offset":6792,"parent_record_offset":6768,"ancestor_record_offset":4480,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":383,"record_offset":15176,"parent_record_offset":15020,"ancestor_record_offset":4480,"inlinee":5724,"source_line":460,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":383,"record_offset":15052,"parent_record_offset":15020,"ancestor_record_offset":4480,"inlinee":5978,"source_line":686,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":383,"record_offset":15140,"parent_record_offset":15052,"ancestor_record_offset":4480,"inlinee":5725,"source_line":1975,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":383,"record_offset":15104,"parent_record_offset":15052,"ancestor_record_offset":4480,"inlinee":5979,"source_line":2173,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":383,"record_offset":8404,"parent_record_offset":8360,"ancestor_record_offset":4480,"inlinee":5655,"source_line":1547,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\mod.rs
//     evidence={"module":383,"record_offset":4940,"parent_record_offset":4844,"ancestor_record_offset":4480,"inlinee":6114,"source_line":1101,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":383,"record_offset":9640,"parent_record_offset":9616,"ancestor_record_offset":4480,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":383,"record_offset":8972,"parent_record_offset":8948,"ancestor_record_offset":4480,"inlinee":4215,"source_line":704,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":383,"record_offset":6552,"parent_record_offset":6524,"ancestor_record_offset":4480,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":383,"record_offset":18476,"parent_record_offset":18452,"ancestor_record_offset":4480,"inlinee":4711,"source_line":607,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":383,"record_offset":5900,"parent_record_offset":5440,"ancestor_record_offset":4480,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::ok_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":383,"record_offset":6600,"parent_record_offset":6424,"ancestor_record_offset":4480,"inlinee":7261,"source_line":1206,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or_else
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":383,"record_offset":9004,"parent_record_offset":8924,"ancestor_record_offset":4480,"inlinee":5273,"source_line":1507,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":383,"record_offset":5252,"parent_record_offset":5224,"ancestor_record_offset":4480,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=17
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":383,"record_offset":6924,"parent_record_offset":6628,"ancestor_record_offset":4480,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":383,"record_offset":8708,"parent_record_offset":8684,"ancestor_record_offset":4480,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":383,"record_offset":9888,"parent_record_offset":9864,"ancestor_record_offset":4480,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":383,"record_offset":16060,"parent_record_offset":16036,"ancestor_record_offset":4480,"inlinee":6972,"source_line":1335,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":383,"record_offset":9780,"parent_record_offset":9756,"ancestor_record_offset":4480,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":383,"record_offset":9128,"parent_record_offset":9104,"ancestor_record_offset":4480,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":383,"record_offset":5676,"parent_record_offset":5652,"ancestor_record_offset":4480,"inlinee":7305,"source_line":531,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":383,"record_offset":5652,"parent_record_offset":5628,"ancestor_record_offset":4480,"inlinee":7304,"source_line":709,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":383,"record_offset":5860,"parent_record_offset":5828,"ancestor_record_offset":4480,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":383,"record_offset":16084,"parent_record_offset":16060,"ancestor_record_offset":4480,"inlinee":4136,"source_line":1246,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":383,"record_offset":6816,"parent_record_offset":6792,"ancestor_record_offset":4480,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":383,"record_offset":7324,"parent_record_offset":6388,"ancestor_record_offset":4480,"inlinee":5301,"source_line":1028,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":383,"record_offset":4876,"parent_record_offset":4844,"ancestor_record_offset":4480,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":383,"record_offset":8660,"parent_record_offset":8360,"ancestor_record_offset":4480,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":383,"record_offset":4904,"parent_record_offset":4876,"ancestor_record_offset":4480,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::get
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":383,"record_offset":8532,"parent_record_offset":8508,"ancestor_record_offset":4480,"inlinee":7968,"source_line":353,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$6::get
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":383,"record_offset":8508,"parent_record_offset":8468,"ancestor_record_offset":4480,"inlinee":8098,"source_line":479,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":383,"record_offset":8684,"parent_record_offset":8660,"ancestor_record_offset":4480,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":383,"record_offset":9104,"parent_record_offset":9080,"ancestor_record_offset":4480,"inlinee":5091,"source_line":101,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":383,"record_offset":5628,"parent_record_offset":5604,"ancestor_record_offset":4480,"inlinee":7303,"source_line":119,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":383,"record_offset":5956,"parent_record_offset":5932,"ancestor_record_offset":4480,"inlinee":4966,"source_line":279,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":383,"record_offset":9080,"parent_record_offset":9028,"ancestor_record_offset":4480,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":383,"record_offset":5604,"parent_record_offset":5496,"ancestor_record_offset":4480,"inlinee":7302,"source_line":392,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":383,"record_offset":4844,"parent_record_offset":4816,"ancestor_record_offset":4480,"inlinee":6113,"source_line":217,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":383,"record_offset":5336,"parent_record_offset":5304,"ancestor_record_offset":4480,"inlinee":7292,"source_line":1310,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":383,"record_offset":5468,"parent_record_offset":5440,"ancestor_record_offset":4480,"inlinee":7272,"source_line":131,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":383,"record_offset":5440,"parent_record_offset":5408,"ancestor_record_offset":4480,"inlinee":7300,"source_line":221,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":383,"record_offset":5384,"parent_record_offset":5360,"ancestor_record_offset":4480,"inlinee":7296,"source_line":301,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":383,"record_offset":5408,"parent_record_offset":5384,"ancestor_record_offset":4480,"inlinee":7298,"source_line":690,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":383,"record_offset":5360,"parent_record_offset":5336,"ancestor_record_offset":4480,"inlinee":7294,"source_line":779,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":383,"record_offset":5932,"parent_record_offset":5408,"ancestor_record_offset":4480,"inlinee":7307,"source_line":640,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":383,"record_offset":5040,"parent_record_offset":5008,"ancestor_record_offset":4480,"inlinee":7287,"source_line":402,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":383,"record_offset":5200,"parent_record_offset":5040,"ancestor_record_offset":4480,"inlinee":7288,"source_line":422,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":383,"record_offset":5008,"parent_record_offset":4984,"ancestor_record_offset":4480,"inlinee":7286,"source_line":434,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":383,"record_offset":4760,"parent_record_offset":4728,"ancestor_record_offset":4480,"inlinee":6109,"source_line":60,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":383,"record_offset":5224,"parent_record_offset":5200,"ancestor_record_offset":4480,"inlinee":7290,"source_line":192,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$9::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":383,"record_offset":4816,"parent_record_offset":4784,"ancestor_record_offset":4480,"inlinee":6112,"source_line":333,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$9::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":383,"record_offset":4784,"parent_record_offset":4760,"ancestor_record_offset":4480,"inlinee":6111,"source_line":363,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":383,"record_offset":5496,"parent_record_offset":5468,"ancestor_record_offset":4480,"inlinee":7273,"source_line":79,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":383,"record_offset":5744,"parent_record_offset":5496,"ancestor_record_offset":4480,"inlinee":4936,"source_line":17,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":383,"record_offset":5716,"parent_record_offset":5496,"ancestor_record_offset":4480,"inlinee":4935,"source_line":11,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":383,"record_offset":5548,"parent_record_offset":5496,"ancestor_record_offset":4480,"inlinee":7274,"source_line":24,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator::impl$25::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":383,"record_offset":18004,"parent_record_offset":17980,"ancestor_record_offset":4480,"inlinee":21651,"source_line":84,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::impl$9::eq
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":383,"record_offset":10000,"parent_record_offset":9976,"ancestor_record_offset":4480,"inlinee":8043,"source_line":65,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\lib.rs
//     evidence={"module":383,"record_offset":9952,"parent_record_offset":9928,"ancestor_record_offset":4480,"inlinee":4854,"source_line":170,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::contains_key
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":383,"record_offset":9212,"parent_record_offset":9184,"ancestor_record_offset":4480,"inlinee":8704,"source_line":1418,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":383,"record_offset":17220,"parent_record_offset":17196,"ancestor_record_offset":4480,"inlinee":5561,"source_line":1308,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":383,"record_offset":9240,"parent_record_offset":9212,"ancestor_record_offset":4480,"inlinee":5562,"source_line":1351,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::iter
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":383,"record_offset":16160,"parent_record_offset":16136,"ancestor_record_offset":4480,"inlinee":8929,"source_line":756,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::remove
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":383,"record_offset":20328,"parent_record_offset":20304,"ancestor_record_offset":4480,"inlinee":7346,"source_line":1895,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::remove_entry
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":383,"record_offset":20356,"parent_record_offset":20328,"ancestor_record_offset":4480,"inlinee":7347,"source_line":1934,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":383,"record_offset":9928,"parent_record_offset":9708,"ancestor_record_offset":4480,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$59::into_iter
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":383,"record_offset":14608,"parent_record_offset":14580,"ancestor_record_offset":4480,"inlinee":19625,"source_line":4716,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$60::fold
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":383,"record_offset":8168,"parent_record_offset":8144,"ancestor_record_offset":4480,"inlinee":16170,"source_line":4742,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$60::next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":383,"record_offset":16496,"parent_record_offset":16468,"ancestor_record_offset":4480,"inlinee":8644,"source_line":4727,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$67::next
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":383,"record_offset":15384,"parent_record_offset":15356,"ancestor_record_offset":4480,"inlinee":21711,"source_line":4814,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$94::extend
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":383,"record_offset":7976,"parent_record_offset":7952,"ancestor_record_offset":4480,"inlinee":18288,"source_line":6499,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=unresolved_namespace; count=5
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":383,"record_offset":9272,"parent_record_offset":9240,"ancestor_record_offset":4480,"inlinee":5617,"source_line":257,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":383,"record_offset":9840,"parent_record_offset":9816,"ancestor_record_offset":4480,"inlinee":5640,"source_line":513,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":383,"record_offset":9816,"parent_record_offset":9708,"ancestor_record_offset":4480,"inlinee":5639,"source_line":682,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":383,"record_offset":9756,"parent_record_offset":9732,"ancestor_record_offset":4480,"inlinee":5638,"source_line":355,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::next_n
//     owner=unresolved_namespace; count=6
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":383,"record_offset":15832,"parent_record_offset":15476,"ancestor_record_offset":4480,"inlinee":8358,"source_line":561,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::read
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":383,"record_offset":16036,"parent_record_offset":15412,"ancestor_record_offset":4480,"inlinee":11804,"source_line":610,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":383,"record_offset":10136,"parent_record_offset":9348,"ancestor_record_offset":4480,"inlinee":5645,"source_line":175,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::new
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":383,"record_offset":14732,"parent_record_offset":14700,"ancestor_record_offset":4480,"inlinee":6425,"source_line":3760,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::next_impl
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":383,"record_offset":15476,"parent_record_offset":15440,"ancestor_record_offset":4480,"inlinee":8357,"source_line":3827,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":383,"record_offset":9732,"parent_record_offset":9708,"ancestor_record_offset":4480,"inlinee":5636,"source_line":997,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":383,"record_offset":9324,"parent_record_offset":9300,"ancestor_record_offset":4480,"inlinee":5620,"source_line":1456,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":383,"record_offset":9300,"parent_record_offset":9240,"ancestor_record_offset":4480,"inlinee":7330,"source_line":1479,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::into_allocation
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":383,"record_offset":14960,"parent_record_offset":14936,"ancestor_record_offset":4480,"inlinee":19590,"source_line":1667,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::into_iter_from
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":383,"record_offset":14936,"parent_record_offset":14636,"ancestor_record_offset":4480,"inlinee":19589,"source_line":1653,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::iter
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":383,"record_offset":14668,"parent_record_offset":14636,"ancestor_record_offset":4480,"inlinee":6420,"source_line":1596,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::buckets
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":383,"record_offset":14992,"parent_record_offset":14960,"ancestor_record_offset":4480,"inlinee":6423,"source_line":2806,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::data_end
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":383,"record_offset":16240,"parent_record_offset":16208,"ancestor_record_offset":4480,"inlinee":6422,"source_line":2597,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":383,"record_offset":9348,"parent_record_offset":9324,"ancestor_record_offset":4480,"inlinee":5622,"source_line":2168,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":383,"record_offset":14700,"parent_record_offset":14668,"ancestor_record_offset":4480,"inlinee":6421,"source_line":2297,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::TableLayout::calculate_layout_for
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":383,"record_offset":15020,"parent_record_offset":14960,"ancestor_record_offset":4480,"inlinee":6234,"source_line":256,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=7
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":383,"record_offset":9588,"parent_record_offset":9560,"ancestor_record_offset":4480,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=7
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":383,"record_offset":9616,"parent_record_offset":9588,"ancestor_record_offset":4480,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::remove_lowest_bit
//     owner=unresolved_namespace; count=7
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":383,"record_offset":9676,"parent_record_offset":9560,"ancestor_record_offset":4480,"inlinee":5633,"source_line":37,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=7
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":383,"record_offset":9560,"parent_record_offset":9348,"ancestor_record_offset":4480,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::h2
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":383,"record_offset":9392,"parent_record_offset":9348,"ancestor_record_offset":4480,"inlinee":5624,"source_line":150,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$19::into_iter
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":383,"record_offset":14636,"parent_record_offset":14608,"ancestor_record_offset":4480,"inlinee":19588,"source_line":3706,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$28::fold
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":383,"record_offset":8192,"parent_record_offset":8168,"ancestor_record_offset":4480,"inlinee":16171,"source_line":4151,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$28::next
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":383,"record_offset":15440,"parent_record_offset":15412,"ancestor_record_offset":4480,"inlinee":8355,"source_line":4127,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$38::drop
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":383,"record_offset":18716,"parent_record_offset":18692,"ancestor_record_offset":4480,"inlinee":19627,"source_line":4312,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$39::next
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":383,"record_offset":15412,"parent_record_offset":15384,"ancestor_record_offset":4480,"inlinee":19629,"source_line":4344,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":383,"record_offset":9708,"parent_record_offset":9348,"ancestor_record_offset":4480,"inlinee":5635,"source_line":1466,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":383,"record_offset":9420,"parent_record_offset":9348,"ancestor_record_offset":4480,"inlinee":5626,"source_line":52,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load_aligned
//     owner=unresolved_namespace; count=6
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":383,"record_offset":14768,"parent_record_offset":14732,"ancestor_record_offset":4480,"inlinee":6426,"source_line":60,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_byte
//     owner=unresolved_namespace; count=8
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":383,"record_offset":9504,"parent_record_offset":9348,"ancestor_record_offset":4480,"inlinee":5627,"source_line":79,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":383,"record_offset":10044,"parent_record_offset":9348,"ancestor_record_offset":4480,"inlinee":5643,"source_line":97,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty_or_deleted
//     owner=unresolved_namespace; count=6
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":383,"record_offset":14828,"parent_record_offset":14768,"ancestor_record_offset":4480,"inlinee":6427,"source_line":104,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::HashSet::insert
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\set.rs
//     evidence={"module":383,"record_offset":8840,"parent_record_offset":8816,"ancestor_record_offset":4480,"inlinee":6708,"source_line":1114,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::impl$11::extend
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\set.rs
//     evidence={"module":383,"record_offset":7952,"parent_record_offset":7928,"ancestor_record_offset":4480,"inlinee":18286,"source_line":1354,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::impl$19::into_iter
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\set.rs
//     evidence={"module":383,"record_offset":14580,"parent_record_offset":14552,"ancestor_record_offset":4480,"inlinee":21705,"source_line":1673,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::impl$25::next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\set.rs
//     evidence={"module":383,"record_offset":15356,"parent_record_offset":15328,"ancestor_record_offset":4480,"inlinee":21709,"source_line":1726,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":383,"record_offset":6352,"parent_record_offset":6316,"ancestor_record_offset":4480,"inlinee":7259,"source_line":425,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::__getit
//     owner=unresolved_namespace; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":383,"record_offset":6464,"parent_record_offset":6424,"ancestor_record_offset":4480,"inlinee":7260,"source_line":85,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":383,"record_offset":6628,"parent_record_offset":6424,"ancestor_record_offset":4480,"inlinee":7263,"source_line":429,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":383,"record_offset":6896,"parent_record_offset":6628,"ancestor_record_offset":4480,"inlinee":7311,"source_line":591,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":383,"record_offset":17196,"parent_record_offset":17172,"ancestor_record_offset":4480,"inlinee":7329,"source_line":877,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::remove
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":383,"record_offset":20304,"parent_record_offset":20280,"ancestor_record_offset":4480,"inlinee":7345,"source_line":1160,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$39::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":383,"record_offset":8144,"parent_record_offset":8120,"ancestor_record_offset":4480,"inlinee":16169,"source_line":2245,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$50::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":383,"record_offset":8120,"parent_record_offset":8096,"ancestor_record_offset":4480,"inlinee":21701,"source_line":2369,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$8::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":383,"record_offset":14104,"parent_record_offset":14080,"ancestor_record_offset":4480,"inlinee":5535,"source_line":1322,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::HashSet::with_hasher
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\set.rs
//     evidence={"module":383,"record_offset":7900,"parent_record_offset":7452,"ancestor_record_offset":4480,"inlinee":18282,"source_line":373,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::impl$7::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\set.rs
//     evidence={"module":383,"record_offset":7452,"parent_record_offset":7420,"ancestor_record_offset":4480,"inlinee":18281,"source_line":1027,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::impl$9::extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\set.rs
//     evidence={"module":383,"record_offset":7928,"parent_record_offset":7452,"ancestor_record_offset":4480,"inlinee":18284,"source_line":1072,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":383,"record_offset":7516,"parent_record_offset":7492,"ancestor_record_offset":4480,"inlinee":5539,"source_line":55,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::__getit
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":383,"record_offset":7592,"parent_record_offset":7564,"ancestor_record_offset":4480,"inlinee":5544,"source_line":85,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":383,"record_offset":7712,"parent_record_offset":7564,"ancestor_record_offset":4480,"inlinee":5550,"source_line":71,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":383,"record_offset":7492,"parent_record_offset":7452,"ancestor_record_offset":4480,"inlinee":5537,"source_line":151,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::fast_local::Key::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":383,"record_offset":6492,"parent_record_offset":6464,"ancestor_record_offset":4480,"inlinee":5546,"source_line":165,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::lazy::LazyKeyInner::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\mod.rs
//     evidence={"module":383,"record_offset":6524,"parent_record_offset":6492,"ancestor_record_offset":4480,"inlinee":5548,"source_line":41,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":383,"record_offset":6424,"parent_record_offset":6388,"ancestor_record_offset":4480,"inlinee":5542,"source_line":278,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":383,"record_offset":6388,"parent_record_offset":6352,"ancestor_record_offset":4480,"inlinee":5541,"source_line":256,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::cull_oob_decorators::cull_oob_decorators::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 90
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xEB20
// 首条位置: {"module":1276,"inlinee":60192}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] std::collections::hash::map::HashMap::contains_key
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1276,"record_offset":83492,"parent_record_offset":83468,"ancestor_record_offset":83468,"inlinee":17300,"source_line":1042,"archive":"_pdb/A/modules/01276.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1276,"record_offset":84272,"parent_record_offset":84248,"ancestor_record_offset":83468,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/01276.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1276,"record_offset":83740,"parent_record_offset":83716,"ancestor_record_offset":83468,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/01276.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1276,"record_offset":83824,"parent_record_offset":83800,"ancestor_record_offset":83468,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/01276.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":1276,"record_offset":83764,"parent_record_offset":83740,"ancestor_record_offset":83468,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/01276.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":1276,"record_offset":83936,"parent_record_offset":83912,"ancestor_record_offset":83468,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/01276.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1276,"record_offset":84184,"parent_record_offset":84160,"ancestor_record_offset":83468,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/01276.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1276,"record_offset":84076,"parent_record_offset":84052,"ancestor_record_offset":83468,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/01276.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::impl$9::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":1276,"record_offset":84296,"parent_record_offset":84272,"ancestor_record_offset":83468,"inlinee":8043,"source_line":65,"archive":"_pdb/A/modules/01276.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\lib.rs
//     evidence={"module":1276,"record_offset":84248,"parent_record_offset":84224,"ancestor_record_offset":83468,"inlinee":4854,"source_line":170,"archive":"_pdb/A/modules/01276.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::contains_key
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1276,"record_offset":83516,"parent_record_offset":83492,"ancestor_record_offset":83468,"inlinee":8704,"source_line":1418,"archive":"_pdb/A/modules/01276.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1276,"record_offset":83540,"parent_record_offset":83516,"ancestor_record_offset":83468,"inlinee":5562,"source_line":1351,"archive":"_pdb/A/modules/01276.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1276,"record_offset":84224,"parent_record_offset":84004,"ancestor_record_offset":83468,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/01276.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1276,"record_offset":83568,"parent_record_offset":83540,"ancestor_record_offset":83468,"inlinee":5617,"source_line":257,"archive":"_pdb/A/modules/01276.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1276,"record_offset":84136,"parent_record_offset":84112,"ancestor_record_offset":83468,"inlinee":5640,"source_line":513,"archive":"_pdb/A/modules/01276.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1276,"record_offset":84112,"parent_record_offset":84004,"ancestor_record_offset":83468,"inlinee":5639,"source_line":682,"archive":"_pdb/A/modules/01276.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1276,"record_offset":84052,"parent_record_offset":84028,"ancestor_record_offset":83468,"inlinee":5638,"source_line":355,"archive":"_pdb/A/modules/01276.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1276,"record_offset":84432,"parent_record_offset":83644,"ancestor_record_offset":83468,"inlinee":5645,"source_line":175,"archive":"_pdb/A/modules/01276.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1276,"record_offset":84028,"parent_record_offset":84004,"ancestor_record_offset":83468,"inlinee":5636,"source_line":997,"archive":"_pdb/A/modules/01276.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1276,"record_offset":83620,"parent_record_offset":83596,"ancestor_record_offset":83468,"inlinee":5620,"source_line":1456,"archive":"_pdb/A/modules/01276.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1276,"record_offset":83596,"parent_record_offset":83540,"ancestor_record_offset":83468,"inlinee":7330,"source_line":1479,"archive":"_pdb/A/modules/01276.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1276,"record_offset":83644,"parent_record_offset":83620,"ancestor_record_offset":83468,"inlinee":5622,"source_line":2168,"archive":"_pdb/A/modules/01276.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1276,"record_offset":83884,"parent_record_offset":83856,"ancestor_record_offset":83468,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/01276.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1276,"record_offset":83912,"parent_record_offset":83884,"ancestor_record_offset":83468,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/01276.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::remove_lowest_bit
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1276,"record_offset":83972,"parent_record_offset":83856,"ancestor_record_offset":83468,"inlinee":5633,"source_line":37,"archive":"_pdb/A/modules/01276.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1276,"record_offset":83856,"parent_record_offset":83644,"ancestor_record_offset":83468,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/01276.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::h2
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1276,"record_offset":83688,"parent_record_offset":83644,"ancestor_record_offset":83468,"inlinee":5624,"source_line":150,"archive":"_pdb/A/modules/01276.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1276,"record_offset":84004,"parent_record_offset":83644,"ancestor_record_offset":83468,"inlinee":5635,"source_line":1466,"archive":"_pdb/A/modules/01276.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":1276,"record_offset":83716,"parent_record_offset":83644,"ancestor_record_offset":83468,"inlinee":5626,"source_line":52,"archive":"_pdb/A/modules/01276.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_byte
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":1276,"record_offset":83800,"parent_record_offset":83644,"ancestor_record_offset":83468,"inlinee":5627,"source_line":79,"archive":"_pdb/A/modules/01276.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":1276,"record_offset":84340,"parent_record_offset":83644,"ancestor_record_offset":83468,"inlinee":5643,"source_line":97,"archive":"_pdb/A/modules/01276.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::cull_oob_decorators::DecoratorLocationCheckResult::is_valid
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 252
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x54D1
// 首条位置: {"module":383,"inlinee":21713}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::cull_oob_decorators::DecoratorLocationCheckResult::get_invalid_reason
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 256
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5500
// 首条位置: {"module":3832,"inlinee":21760}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] system_decorator::cull_oob_decorators::impl$1::clone
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/cull_oob_decorators.rs
//     evidence={"module":3832,"record_offset":3624,"parent_record_offset":3572,"ancestor_record_offset":3572,"inlinee":21724,"source_line":265,"archive":"_pdb/A/modules/03832.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::cull_oob_decorators::impl$1::clone
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 265
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x54DC
// 首条位置: {"module":383,"inlinee":21724}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::cull_oob_decorators::is_wall_decorator_location_valid
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 457
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x54E1
// 首条位置: {"module":383,"inlinee":21729}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":383,"record_offset":26208,"parent_record_offset":22396,"ancestor_record_offset":22396,"inlinee":4244,"source_line":761,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::and_then
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":383,"record_offset":26376,"parent_record_offset":22396,"ancestor_record_offset":22396,"inlinee":4151,"source_line":1408,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":383,"record_offset":23188,"parent_record_offset":22396,"ancestor_record_offset":22396,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":383,"record_offset":25812,"parent_record_offset":22396,"ancestor_record_offset":22396,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":383,"record_offset":27096,"parent_record_offset":22396,"ancestor_record_offset":22396,"inlinee":4963,"source_line":2524,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":383,"record_offset":26124,"parent_record_offset":22396,"ancestor_record_offset":22396,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::last
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":383,"record_offset":25728,"parent_record_offset":22396,"ancestor_record_offset":22396,"inlinee":9339,"source_line":302,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo::dims_wall_space
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":383,"record_offset":22932,"parent_record_offset":22396,"ancestor_record_offset":22396,"inlinee":21730,"source_line":261,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::public_wall_state::PublicWallState::is_premade_shape
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_wall_state.rs
//     evidence={"module":383,"record_offset":26348,"parent_record_offset":22396,"ancestor_record_offset":22396,"inlinee":21375,"source_line":96,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":383,"record_offset":26320,"parent_record_offset":22396,"ancestor_record_offset":22396,"inlinee":14697,"source_line":157,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::normalize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":383,"record_offset":25100,"parent_record_offset":22396,"ancestor_record_offset":22396,"inlinee":21371,"source_line":450,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::perp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":383,"record_offset":25072,"parent_record_offset":22396,"ancestor_record_offset":22396,"inlinee":14690,"source_line":814,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_direct] itertools::Itertools::tuple_windows
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"itertools","version":"0.10.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/itertools-0.10.5/src/lib.rs
//     evidence={"module":383,"record_offset":26264,"parent_record_offset":22396,"ancestor_record_offset":22396,"inlinee":11990,"source_line":699,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::cull_oob_decorators::is_wall_decorator_location_valid::closure$0
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/cull_oob_decorators.rs
//     evidence={"module":383,"record_offset":23224,"parent_record_offset":22396,"ancestor_record_offset":22396,"inlinee":21738,"source_line":477,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::cull_oob_decorators::should_snap_to_corner
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/cull_oob_decorators.rs
//     evidence={"module":383,"record_offset":26752,"parent_record_offset":22396,"ancestor_record_offset":22396,"inlinee":21750,"source_line":899,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::get_pos_at_coord
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":383,"record_offset":24116,"parent_record_offset":22396,"ancestor_record_offset":22396,"inlinee":21575,"source_line":622,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::get_tangent_at_coord
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":383,"record_offset":24572,"parent_record_offset":22396,"ancestor_record_offset":22396,"inlinee":21578,"source_line":637,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::impl$12::sample_at_coord
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":383,"record_offset":25444,"parent_record_offset":22396,"ancestor_record_offset":22396,"inlinee":21356,"source_line":1168,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_direct] utils::wall_space::WallSpaceCoord::new
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/wall_space.rs
//     evidence={"module":383,"record_offset":26724,"parent_record_offset":22396,"ancestor_record_offset":22396,"inlinee":21749,"source_line":106,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_direct] utils::wall_space::WallSpaceCoord::with_offset
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/wall_space.rs
//     evidence={"module":383,"record_offset":25320,"parent_record_offset":22396,"ancestor_record_offset":22396,"inlinee":21743,"source_line":197,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":383,"record_offset":26068,"parent_record_offset":26032,"ancestor_record_offset":22396,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":383,"record_offset":26032,"parent_record_offset":25888,"ancestor_record_offset":22396,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":383,"record_offset":25932,"parent_record_offset":25888,"ancestor_record_offset":22396,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":383,"record_offset":25888,"parent_record_offset":25856,"ancestor_record_offset":22396,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$12::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":383,"record_offset":24148,"parent_record_offset":24116,"ancestor_record_offset":22396,"inlinee":5450,"source_line":2828,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/query.rs
//     evidence={"module":383,"record_offset":26428,"parent_record_offset":26404,"ancestor_record_offset":22396,"inlinee":21384,"source_line":846,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$21::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":383,"record_offset":27192,"parent_record_offset":27168,"ancestor_record_offset":22396,"inlinee":19598,"source_line":1469,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":383,"record_offset":23500,"parent_record_offset":23224,"ancestor_record_offset":22396,"inlinee":5741,"source_line":924,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":383,"record_offset":23060,"parent_record_offset":23036,"ancestor_record_offset":22396,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::Map::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":383,"record_offset":26232,"parent_record_offset":26208,"ancestor_record_offset":22396,"inlinee":4246,"source_line":69,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":383,"record_offset":25984,"parent_record_offset":25932,"ancestor_record_offset":22396,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":383,"record_offset":24500,"parent_record_offset":24440,"ancestor_record_offset":22396,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":383,"record_offset":24528,"parent_record_offset":24440,"ancestor_record_offset":22396,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":383,"record_offset":24472,"parent_record_offset":24440,"ancestor_record_offset":22396,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":383,"record_offset":27748,"parent_record_offset":27720,"ancestor_record_offset":22396,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":383,"record_offset":23004,"parent_record_offset":22976,"ancestor_record_offset":22396,"inlinee":4138,"source_line":972,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$40::branch
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":383,"record_offset":25348,"parent_record_offset":25320,"ancestor_record_offset":22396,"inlinee":5366,"source_line":2485,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":383,"record_offset":25856,"parent_record_offset":25812,"ancestor_record_offset":22396,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":383,"record_offset":26172,"parent_record_offset":26148,"ancestor_record_offset":22396,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::ok
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":383,"record_offset":26468,"parent_record_offset":26404,"ancestor_record_offset":22396,"inlinee":4772,"source_line":631,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$10::slice_contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":383,"record_offset":27120,"parent_record_offset":27096,"ancestor_record_offset":22396,"inlinee":4965,"source_line":215,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$10::slice_contains::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":383,"record_offset":27168,"parent_record_offset":27144,"ancestor_record_offset":22396,"inlinee":4968,"source_line":216,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":383,"record_offset":24180,"parent_record_offset":24148,"ancestor_record_offset":22396,"inlinee":5452,"source_line":17,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":383,"record_offset":24212,"parent_record_offset":24180,"ancestor_record_offset":22396,"inlinee":5453,"source_line":262,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":383,"record_offset":26148,"parent_record_offset":26124,"ancestor_record_offset":22396,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":383,"record_offset":27144,"parent_record_offset":27120,"ancestor_record_offset":22396,"inlinee":4966,"source_line":279,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::DecoratorCollision::dims_wall_space
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":383,"record_offset":23096,"parent_record_offset":22976,"ancestor_record_offset":22396,"inlinee":21733,"source_line":102,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary::dims_wall_space
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":383,"record_offset":22976,"parent_record_offset":22932,"ancestor_record_offset":22396,"inlinee":21731,"source_line":217,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::impl$3::dims_wall_space::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":383,"record_offset":23036,"parent_record_offset":23004,"ancestor_record_offset":22396,"inlinee":21753,"source_line":220,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":383,"record_offset":25188,"parent_record_offset":25156,"ancestor_record_offset":22396,"inlinee":14697,"source_line":157,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":383,"record_offset":25156,"parent_record_offset":25124,"ancestor_record_offset":22396,"inlinee":21373,"source_line":380,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_recip
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":383,"record_offset":25124,"parent_record_offset":25100,"ancestor_record_offset":22396,"inlinee":21372,"source_line":399,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":383,"record_offset":24932,"parent_record_offset":24900,"ancestor_record_offset":22396,"inlinee":13203,"source_line":195,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":383,"record_offset":24900,"parent_record_offset":24868,"ancestor_record_offset":22396,"inlinee":21582,"source_line":436,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length_recip
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":383,"record_offset":24868,"parent_record_offset":24844,"ancestor_record_offset":22396,"inlinee":21581,"source_line":455,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":383,"record_offset":24416,"parent_record_offset":24392,"ancestor_record_offset":22396,"inlinee":21436,"source_line":742,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::normalize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":383,"record_offset":24844,"parent_record_offset":24820,"ancestor_record_offset":22396,"inlinee":21580,"source_line":508,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$17::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":383,"record_offset":24440,"parent_record_offset":24416,"ancestor_record_offset":22396,"inlinee":21370,"source_line":1135,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::abs
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":383,"record_offset":26896,"parent_record_offset":26752,"ancestor_record_offset":22396,"inlinee":15239,"source_line":198,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":383,"record_offset":24964,"parent_record_offset":24932,"ancestor_record_offset":22396,"inlinee":13701,"source_line":441,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::cull_oob_decorators::is_wall_decorator_location_valid::closure$3
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/cull_oob_decorators.rs
//     evidence={"module":383,"record_offset":26404,"parent_record_offset":26376,"ancestor_record_offset":22396,"inlinee":21748,"source_line":697,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::get_coord_at_2d_ws_distance
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":383,"record_offset":23288,"parent_record_offset":23224,"ancestor_record_offset":22396,"inlinee":21741,"source_line":944,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::get_unnormalized_tangent_at_coord
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":383,"record_offset":24600,"parent_record_offset":24572,"ancestor_record_offset":22396,"inlinee":21585,"source_line":633,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i1
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":383,"record_offset":24256,"parent_record_offset":24116,"ancestor_record_offset":22396,"inlinee":21359,"source_line":1113,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::CurveSlice::end
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":383,"record_offset":23468,"parent_record_offset":23288,"ancestor_record_offset":22396,"inlinee":21611,"source_line":335,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::CurveSlice::extend_right
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":383,"record_offset":23440,"parent_record_offset":23288,"ancestor_record_offset":22396,"inlinee":21485,"source_line":233,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::CurveSlice::from_curve_range
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":383,"record_offset":23324,"parent_record_offset":23288,"ancestor_record_offset":22396,"inlinee":21742,"source_line":86,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::CurveSlice::from_point_range
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":383,"record_offset":23352,"parent_record_offset":23324,"ancestor_record_offset":22396,"inlinee":21607,"source_line":107,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::CurveSlice::remap
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":383,"record_offset":23408,"parent_record_offset":23384,"ancestor_record_offset":22396,"inlinee":21484,"source_line":193,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::CurveSlice::remap_2d
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":383,"record_offset":23384,"parent_record_offset":23288,"ancestor_record_offset":22396,"inlinee":21483,"source_line":217,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle3d::height
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":383,"record_offset":23148,"parent_record_offset":23096,"ancestor_record_offset":22396,"inlinee":21736,"source_line":65,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle3d::width
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":383,"record_offset":23120,"parent_record_offset":23096,"ancestor_record_offset":22396,"inlinee":21735,"source_line":70,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$1::lerp
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":383,"record_offset":24392,"parent_record_offset":24116,"ancestor_record_offset":22396,"inlinee":21577,"source_line":106,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$1::normalize
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":383,"record_offset":24820,"parent_record_offset":24572,"ancestor_record_offset":22396,"inlinee":21579,"source_line":96,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] utils::wall_space::WallSpaceCoord::subtract
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/wall_space.rs
//     evidence={"module":383,"record_offset":26868,"parent_record_offset":26752,"ancestor_record_offset":22396,"inlinee":21751,"source_line":268,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] utils::wall_space::WallSpaceCoord::with_offset_clamped
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/wall_space.rs
//     evidence={"module":383,"record_offset":23260,"parent_record_offset":23224,"ancestor_record_offset":22396,"inlinee":21740,"source_line":241,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::cull_oob_decorators::is_wall_decorator_location_valid::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=5
// 源码行: 477
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x54EA
// 首条位置: {"module":383,"inlinee":21738}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":383,"record_offset":23500,"parent_record_offset":23224,"ancestor_record_offset":23224,"inlinee":5741,"source_line":924,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::get_coord_at_2d_ws_distance
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":383,"record_offset":23288,"parent_record_offset":23224,"ancestor_record_offset":23224,"inlinee":21741,"source_line":944,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_direct] utils::wall_space::WallSpaceCoord::with_offset_clamped
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/wall_space.rs
//     evidence={"module":383,"record_offset":23260,"parent_record_offset":23224,"ancestor_record_offset":23224,"inlinee":21740,"source_line":241,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":383,"record_offset":27748,"parent_record_offset":27720,"ancestor_record_offset":27648,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::CurveSlice::end
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":383,"record_offset":23468,"parent_record_offset":23288,"ancestor_record_offset":23224,"inlinee":21611,"source_line":335,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::CurveSlice::extend_right
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":383,"record_offset":23440,"parent_record_offset":23288,"ancestor_record_offset":23224,"inlinee":21485,"source_line":233,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::CurveSlice::from_curve_range
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":383,"record_offset":23324,"parent_record_offset":23288,"ancestor_record_offset":23224,"inlinee":21742,"source_line":86,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::CurveSlice::from_point_range
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":383,"record_offset":23352,"parent_record_offset":23324,"ancestor_record_offset":23224,"inlinee":21607,"source_line":107,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::CurveSlice::remap
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":383,"record_offset":23408,"parent_record_offset":23384,"ancestor_record_offset":23224,"inlinee":21484,"source_line":193,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::CurveSlice::remap_2d
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":383,"record_offset":23384,"parent_record_offset":23288,"ancestor_record_offset":23224,"inlinee":21483,"source_line":217,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::cull_oob_decorators::is_wall_decorator_location_valid::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=10
// 源码行: 587
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xEB62
// 首条位置: {"module":1284,"inlinee":60258}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] utils::curve::Curve::get_pos_at_coord
//     owner=private_source_ownership_candidate; count=7
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1284,"record_offset":1188,"parent_record_offset":1156,"ancestor_record_offset":1156,"inlinee":21575,"source_line":622,"archive":"_pdb/A/modules/01284.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$12::index
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1284,"record_offset":1224,"parent_record_offset":1188,"ancestor_record_offset":1156,"inlinee":5450,"source_line":2828,"archive":"_pdb/A/modules/01284.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1284,"record_offset":1600,"parent_record_offset":1524,"ancestor_record_offset":1156,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/01284.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1284,"record_offset":1628,"parent_record_offset":1524,"ancestor_record_offset":1156,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/01284.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1284,"record_offset":1572,"parent_record_offset":1524,"ancestor_record_offset":1156,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/01284.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1284,"record_offset":1256,"parent_record_offset":1224,"ancestor_record_offset":1156,"inlinee":5452,"source_line":17,"archive":"_pdb/A/modules/01284.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1284,"record_offset":1288,"parent_record_offset":1256,"ancestor_record_offset":1156,"inlinee":5453,"source_line":262,"archive":"_pdb/A/modules/01284.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::lerp
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1284,"record_offset":1496,"parent_record_offset":1468,"ancestor_record_offset":1156,"inlinee":21436,"source_line":742,"archive":"_pdb/A/modules/01284.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$17::sub
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1284,"record_offset":1524,"parent_record_offset":1496,"ancestor_record_offset":1156,"inlinee":21370,"source_line":1135,"archive":"_pdb/A/modules/01284.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i1
//     owner=private_source_ownership_candidate; count=6
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1284,"record_offset":1332,"parent_record_offset":1188,"ancestor_record_offset":1156,"inlinee":21359,"source_line":1113,"archive":"_pdb/A/modules/01284.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$1::lerp
//     owner=private_source_ownership_candidate; count=6
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":1284,"record_offset":1468,"parent_record_offset":1188,"ancestor_record_offset":1156,"inlinee":21577,"source_line":106,"archive":"_pdb/A/modules/01284.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::cull_oob_decorators::is_wall_decorator_location_valid::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=7
// 源码行: 589
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xEB63
// 首条位置: {"module":1284,"inlinee":60259}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec2::Vec2::normalize
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1284,"record_offset":2100,"parent_record_offset":1988,"ancestor_record_offset":1988,"inlinee":21371,"source_line":450,"archive":"_pdb/A/modules/01284.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$17::sub
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1284,"record_offset":2012,"parent_record_offset":1988,"ancestor_record_offset":1988,"inlinee":21370,"source_line":1135,"archive":"_pdb/A/modules/01284.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1284,"record_offset":2256,"parent_record_offset":2124,"ancestor_record_offset":1988,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/01284.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1284,"record_offset":2040,"parent_record_offset":2012,"ancestor_record_offset":1988,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/01284.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1284,"record_offset":2188,"parent_record_offset":2156,"ancestor_record_offset":1988,"inlinee":14697,"source_line":157,"archive":"_pdb/A/modules/01284.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1284,"record_offset":2156,"parent_record_offset":2124,"ancestor_record_offset":1988,"inlinee":21373,"source_line":380,"archive":"_pdb/A/modules/01284.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_recip
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1284,"record_offset":2124,"parent_record_offset":2100,"ancestor_record_offset":1988,"inlinee":21372,"source_line":399,"archive":"_pdb/A/modules/01284.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":1284,"record_offset":2220,"parent_record_offset":2188,"ancestor_record_offset":1988,"inlinee":13701,"source_line":441,"archive":"_pdb/A/modules/01284.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::cull_oob_decorators::is_wall_decorator_location_valid::closure$3
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 697
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x54F4
// 首条位置: {"module":383,"inlinee":21748}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] bevy_ecs::system::query::Query::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/query.rs
//     evidence={"module":383,"record_offset":26428,"parent_record_offset":26404,"ancestor_record_offset":26404,"inlinee":21384,"source_line":846,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::ok
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":383,"record_offset":26468,"parent_record_offset":26404,"ancestor_record_offset":26404,"inlinee":4772,"source_line":631,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::cull_oob_decorators::check_if_dormer_ridge_is_within_roof_bounds::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 843
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x54FD
// 首条位置: {"module":383,"inlinee":21757}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] bevy_ecs::system::query::Query::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/query.rs
//     evidence={"module":383,"record_offset":31960,"parent_record_offset":31936,"ancestor_record_offset":31936,"inlinee":21384,"source_line":846,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::impl$24::eq
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/archetype.rs
//     evidence={"module":383,"record_offset":32176,"parent_record_offset":32032,"ancestor_record_offset":31936,"inlinee":21390,"source_line":74,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::Entities::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/entity/mod.rs
//     evidence={"module":383,"record_offset":32032,"parent_record_offset":31988,"ancestor_record_offset":31936,"inlinee":21387,"source_line":739,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$21::fetch
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":383,"record_offset":32820,"parent_record_offset":31988,"ancestor_record_offset":31936,"inlinee":14627,"source_line":963,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$21::set_archetype
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":383,"record_offset":32452,"parent_record_offset":31988,"ancestor_record_offset":31936,"inlinee":21391,"source_line":933,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$21::set_table
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":383,"record_offset":32476,"parent_record_offset":32452,"ancestor_record_offset":31936,"inlinee":14620,"source_line":948,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::state::QueryState::get_unchecked_manual
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/state.rs
//     evidence={"module":383,"record_offset":31988,"parent_record_offset":31960,"ancestor_record_offset":31936,"inlinee":21385,"source_line":859,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_slice
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":383,"record_offset":32780,"parent_record_offset":32756,"ancestor_record_offset":31936,"inlinee":14643,"source_line":410,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseArray::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":383,"record_offset":32552,"parent_record_offset":32528,"ancestor_record_offset":31936,"inlinee":6338,"source_line":56,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":383,"record_offset":32528,"parent_record_offset":32504,"ancestor_record_offset":31936,"inlinee":6337,"source_line":369,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$11::get::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":383,"record_offset":32600,"parent_record_offset":32576,"ancestor_record_offset":31936,"inlinee":6340,"source_line":58,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$16::get::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":383,"record_offset":32688,"parent_record_offset":32664,"ancestor_record_offset":31936,"inlinee":6342,"source_line":370,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Column::get_data_slice
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":383,"record_offset":32756,"parent_record_offset":32476,"ancestor_record_offset":31936,"inlinee":14642,"source_line":321,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_column
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":383,"record_offset":32504,"parent_record_offset":32476,"ancestor_record_offset":31936,"inlinee":14621,"source_line":692,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":383,"record_offset":32424,"parent_record_offset":31988,"ancestor_record_offset":31936,"inlinee":14618,"source_line":834,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ptr::ThinSlicePtr::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ptr/src/lib.rs
//     evidence={"module":383,"record_offset":32844,"parent_record_offset":32820,"ancestor_record_offset":31936,"inlinee":21393,"source_line":500,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] core::bool::impl$0::then
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\bool.rs
//     evidence={"module":383,"record_offset":32240,"parent_record_offset":32208,"ancestor_record_offset":31936,"inlinee":4886,"source_line":59,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$24::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":383,"record_offset":32144,"parent_record_offset":32120,"ancestor_record_offset":31936,"inlinee":13915,"source_line":1471,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::impl$2::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":383,"record_offset":32120,"parent_record_offset":32032,"ancestor_record_offset":31936,"inlinee":21388,"source_line":136,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":383,"record_offset":32624,"parent_record_offset":32600,"ancestor_record_offset":31936,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":383,"record_offset":32576,"parent_record_offset":32552,"ancestor_record_offset":31936,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":383,"record_offset":32348,"parent_record_offset":32316,"ancestor_record_offset":31936,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":383,"record_offset":32396,"parent_record_offset":31988,"ancestor_record_offset":31936,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":383,"record_offset":32064,"parent_record_offset":32032,"ancestor_record_offset":31936,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":383,"record_offset":32088,"parent_record_offset":32064,"ancestor_record_offset":31936,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::contains
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":383,"record_offset":32208,"parent_record_offset":31988,"ancestor_record_offset":31936,"inlinee":4884,"source_line":392,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::contains_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":383,"record_offset":32292,"parent_record_offset":32268,"ancestor_record_offset":31936,"inlinee":4889,"source_line":408,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::get_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":383,"record_offset":32316,"parent_record_offset":32292,"ancestor_record_offset":31936,"inlinee":4890,"source_line":171,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::impl$2::contains::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":383,"record_offset":32268,"parent_record_offset":32240,"ancestor_record_offset":31936,"inlinee":4888,"source_line":395,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nonmax","version":"0.5.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":383,"record_offset":32712,"parent_record_offset":32688,"ancestor_record_offset":31936,"inlinee":4372,"source_line":153,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::cull_oob_decorators::should_snap_to_corner
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=7
// 源码行: 899
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x54F6
// 首条位置: {"module":383,"inlinee":21750}
// 编译实例 RVAs: 0x5029f0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14051aa00
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: cull_oob_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t98262","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373962736,"raw_hex":"e8d67f0100","size":5,"soft_fail":false,"target_va":5374061056,"va":5373962789,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140ceb6a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: cull_oob_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8293436","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373962736,"raw_hex":"e83c8c7e00","size":5,"soft_fail":false,"target_va":5382256288,"va":5373962847,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_wall_state::PublicWallState::wall_space
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t7970230","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373962736,"raw_hex":"e8b69d7900","size":5,"soft_fail":false,"target_va":5381933056,"va":5373962821,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] std::f32::impl$0::abs
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":383,"record_offset":26896,"parent_record_offset":26752,"ancestor_record_offset":26752,"inlinee":15239,"source_line":198,"archive":"_pdb/A/modules/00383.jsonl.gz"}
//   [pdb_inline_direct] utils::wall_space::WallSpaceCoord::subtract
//     owner=private_source_ownership_candidate; count=8
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/wall_space.rs
//     evidence={"module":383,"record_offset":26868,"parent_record_offset":26752,"ancestor_record_offset":26752,"inlinee":21751,"source_line":268,"archive":"_pdb/A/modules/00383.jsonl.gz"}
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
// 完整结构及原始类型记录: cull_oob_decorators.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] system_decorator::cull_oob_decorators::DecoratorLocationCheckResult
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 3; 过程引用数: 0
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] system_decorator::cull_oob_decorators::cull_oob_decorators::DecoratorRemoveInfo
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 3
// [PDB 泛型关系，非字段关系] {"type": "tuple$<country_core::resources::walls::decorator::DecoratorLocator,system_decorator::cull_oob_decorators::cull_oob_decorators::DecoratorRemoveInfo>", "constructor": "tuple$", "arguments": ["country_core::resources::walls::decorator::DecoratorLocator", "system_decorator::cull_oob_decorators::cull_oob_decorators::DecoratorRemoveInfo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::make_hasher::closure_env$0<country_core::resources::walls::decorator::DecoratorLocator,system_decorator::cull_oob_decorators::cull_oob_decorators::DecoratorRemoveInfo,std::hash::random::RandomState>", "constructor": "hashbrown::map::make_hasher::closure_env$0", "arguments": ["country_core::resources::walls::decorator::DecoratorLocator", "system_decorator::cull_oob_decorators::cull_oob_decorators::DecoratorRemoveInfo", "std::hash::random::RandomState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTable::reserve_rehash<tuple$<country_core::resources::walls::decorator::DecoratorLocator,system_decorator::cull_oob_decorators::cull_oob_decorators::DecoratorRemoveInfo>,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<country_core::resources::walls::decorator::DecoratorLocator,system_decorator::cull_oob_decorators::cull_oob_decorators::DecoratorRemoveInfo,std::hash::random::RandomState> >", "constructor": "hashbrown::raw::RawTable::reserve_rehash", "arguments": ["tuple$<country_core::resources::walls::decorator::DecoratorLocator,system_decorator::cull_oob_decorators::cull_oob_decorators::DecoratorRemoveInfo>", "alloc::alloc::Global", "hashbrown::map::make_hasher::closure_env$0<country_core::resources::walls::decorator::DecoratorLocator,system_decorator::cull_oob_decorators::cull_oob_decorators::DecoratorRemoveInfo,std::hash::random::RandomState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::HashMap::insert<country_core::resources::walls::decorator::DecoratorLocator,system_decorator::cull_oob_decorators::cull_oob_decorators::DecoratorRemoveInfo,std::hash::random::RandomState,alloc::alloc::Global>", "constructor": "hashbrown::map::HashMap::insert", "arguments": ["country_core::resources::walls::decorator::DecoratorLocator", "system_decorator::cull_oob_decorators::cull_oob_decorators::DecoratorRemoveInfo", "std::hash::random::RandomState", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::collections::hash::map::HashMap<country_core::resources::walls::decorator::DecoratorLocator,system_decorator::cull_oob_decorators::cull_oob_decorators::DecoratorRemoveInfo,std::hash::random::RandomState>", "constructor": "std::collections::hash::map::HashMap", "arguments": ["country_core::resources::walls::decorator::DecoratorLocator", "system_decorator::cull_oob_decorators::cull_oob_decorators::DecoratorRemoveInfo", "std::hash::random::RandomState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<std::collections::hash::map::HashMap<country_core::resources::walls::decorator::DecoratorLocator,system_decorator::cull_oob_decorators::cull_oob_decorators::DecoratorRemoveInfo,std::hash::random::RandomState> >", "constructor": "core::ptr::drop_in_place", "arguments": ["std::collections::hash::map::HashMap<country_core::resources::walls::decorator::DecoratorLocator,system_decorator::cull_oob_decorators::cull_oob_decorators::DecoratorRemoveInfo,std::hash::random::RandomState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: cull_oob_decorators.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: cull_oob_decorators.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 6; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] glam::f32::vec2::Vec2 (*)(glam::f32::vec2::Vec2)
// 候选参数类型: ["glam::f32::vec2::Vec2"]
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator::DeferredDecoratorOpEvents>,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::event::EventReader<country_core::systems::wall::adjust_wall_height::adjust_wall_height_impl::AdjustWallHeightCmd>,bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>,bevy_ecs::event::EventReader<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::change_detection::Res<country_core::resources::history::History>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>", "bevy_ecs::change_detection::Res<country_core::resources::walls::decorator::DeferredDecoratorOpEvents>", "bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>", "bevy_ecs::event::EventReader<country_core::systems::wall::adjust_wall_height::adjust_wall_height_impl::AdjustWallHeightCmd>", "bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>", "bevy_ecs::event::EventReader<country_core::resources::roofs::commands::EditRoofCmd>", "bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >", "country_core::resources::walls::decorator_archivist::DecoratorArchivalResources", "country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources", "bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::history::History>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::decorator::DeferredDecoratorOpEvents>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::systems::wall::adjust_wall_height::adjust_wall_height_impl::AdjustWallHeightCmd>
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::resources::roofs::commands::EditRoofCmd>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [文件级接口候选，具体函数归属 unknown] glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)
// 候选参数类型: ["usize", "ref$<glam::f32::vec3::Vec3>"]
// [文件级接口候选，具体函数归属 unknown] glam::f32::vec2::Vec2 (*)(glam::f32::vec3::Vec3)
// 候选参数类型: ["glam::f32::vec3::Vec3"]
// [文件级接口候选，具体函数归属 unknown] country_core::resources::walls::WallId (*)(ref$<country_core::resources::walls::WallId>)
// 候选参数类型: ["ref$<country_core::resources::walls::WallId>"]
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::event::EventReader<country_core::systems::wall::adjust_wall_height::adjust_wall_height_impl::AdjustWallHeightCmd>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::history::History>", "bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>", "bevy_ecs::event::EventReader<country_core::systems::wall::adjust_wall_height::adjust_wall_height_impl::AdjustWallHeightCmd>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::history::History>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::systems::wall::adjust_wall_height::adjust_wall_height_impl::AdjustWallHeightCmd>

// ------------------------------------------------------------------
// 函数: system_decorator::cull_oob_decorators::DecoratorLocationCheckResult::get_invalid_reason
// 信息 ID: fd32dadfa2c6683dbd30c4a2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::iter::adapters::copied::impl$1::try_fold<std::collections::hash::set::Iter<country_core::resources::walls::decorator::DecoratorLocator>,country_core::resources::walls::decorator::DecoratorLocator,tuple$<>,core::iter::traits::iterator::Iterator::all::check::closure_env$0<country_core::resources::walls::decorator::DecoratorLocator,system_decorator::blueprint::validate_blueprints::closure_env$0>,enum2$<core::ops::control_flow::ControlFlow<tuple$<>,tuple$<> > > >; module=3832, inline_record=3572; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::cull_oob_decorators::DecoratorLocationCheckResult::is_valid
// 信息 ID: d2bafb1ff891664017e8fc1e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::cull_oob_decorators::cull_oob_decorators; module=383, inline_record=17144; 不是独立调用边。
// [PDB 内联宿主] core::iter::adapters::copied::impl$1::try_fold<std::collections::hash::set::Iter<country_core::resources::walls::decorator::DecoratorLocator>,country_core::resources::walls::decorator::DecoratorLocator,tuple$<>,core::iter::traits::iterator::Iterator::all::check::closure_env$0<country_core::resources::walls::decorator::DecoratorLocator,system_decorator::blueprint::validate_blueprints::closure_env$0>,enum2$<core::ops::control_flow::ControlFlow<tuple$<>,tuple$<> > > >; module=3832, inline_record=3544; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::cull_oob_decorators::auto_allow_decorator_culling
// 信息 ID: 6b725b82e461aedffd85ad82
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] f7aea9d762ff5149f0789b86: void (*)(bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::event::EventReader<country_core::systems::wall::adjust_wall_height::adjust_wall_height_impl::AdjustWallHeightCmd>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::event::EventReader<country_core::systems::wall::adjust_wall_height::adjust_wall_height_impl::AdjustWallHeightCmd>),void (*)(bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::event::EventReader<country_core::systems::wall::adjust_wall_height::adjust_wall_height_impl::AdjustWallHeightCmd>)> @ 0x3182 -> 0x4ff0a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::event::EventReader<country_core::systems::wall::adjust_wall_height::adjust_wall_height_impl::AdjustWallHeightCmd>),void (*)(bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::event::EventReader<country_core::systems::wall::adjust_wall_height::adjust_wall_height_impl::AdjustWallHeightCmd>)> > @ 0xd5fa4 -> 0x4ff0a0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::cull_oob_decorators::check_if_dormer_ridge_is_within_roof_bounds
// 信息 ID: 544be6c53c10232790db4019
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::cull_oob_decorators::is_decorator_location_valid @ 0x5014c2 -> 0x502530 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::cull_oob_decorators::is_decorator_location_valid @ 0x500f45 -> 0x502530 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::cull_oob_decorators::check_if_dormer_ridge_is_within_roof_bounds::closure$0
// 信息 ID: 6752ce815c0b81f73cc53d19
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_decorator::cull_oob_decorators::check_if_dormer_ridge_is_within_roof_bounds", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::cull_oob_decorators::check_if_dormer_ridge_is_within_roof_bounds; module=383, inline_record=31936; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::cull_oob_decorators::cull_oob_decorators
// 信息 ID: e90705d21dbd2b91447ec98c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 224ce8d0a9dec896758294fd: void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator::DeferredDecoratorOpEvents>,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::event::EventReader<country_core::systems::wall::adjust_wall_height::adjust_wall_height_impl::AdjustWallHeightCmd>,bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>,bevy_ecs::event::EventReader<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator::DeferredDecoratorOpEvents>,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::event::EventReader<country_core::systems::wall::adjust_wall_height::adjust_wall_height_impl::AdjustWallHeightCmd>,bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>,bevy_ecs::event::EventReader<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>),void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator::DeferredDecoratorOpEvents>,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::event::EventReader<country_core::systems::wall::adjust_wall_height::adjust_wall_height_impl::AdjustWallHeightCmd>,bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>,bevy_ecs::event::EventReader<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)> > @ 0x17c96fe -> 0x4ff1f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator::DeferredDecoratorOpEvents>,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::event::EventReader<country_core::systems::wall::adjust_wall_height::adjust_wall_height_impl::AdjustWallHeightCmd>,bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>,bevy_ecs::event::EventReader<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>),void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator::DeferredDecoratorOpEvents>,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::event::EventReader<country_core::systems::wall::adjust_wall_height::adjust_wall_height_impl::AdjustWallHeightCmd>,bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>,bevy_ecs::event::EventReader<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)> @ 0x17ee925 -> 0x4ff1f0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::cull_oob_decorators::cull_oob_decorators::closure$0
// 信息 ID: 94ca05bb0b2bf848bff870dc
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 2ce84ca3267526f881835269: country_core::resources::walls::WallId (*)(ref$<country_core::resources::walls::WallId>)
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_decorator::cull_oob_decorators::cull_oob_decorators", "captures": "unknown"}
// [闭包环境类型引用] system_decorator::cull_oob_decorators::cull_oob_decorators::closure_env$0 <- hashbrown::raw::RawIterRange::fold_impl<tuple$<country_core::resources::walls::WallId,country_core::resources::walls::decorator_storage::Decorators>,hashbrown::map::impl$60::fold::closure_env$0<country_core::resources::walls::WallId,country_core::resources::walls::decorator_storage::Decorators,tuple$<>,std::collections::hash::map::impl$50::fold::closure_env$0<country_core::resources::walls::WallId,country_core::resources::walls::decorator_storage::Decorators,tuple$<>,core::iter::adapters::map::map_fold::closure_env$0<ref$<country_core::resources::walls::WallId>,country_core::resources::walls::WallId,tuple$<>,country_core::resources::walls::WallId (*)(ref$<country_core::resources::walls::WallId>),core::iter::adapters::filter::filter_fold::closure_env$0<country_core::resources::walls::WallId,tuple$<>,system_decorator::cull_oob_decorators::cull_oob_decorators::closure_env$0,core::iter::adapters::map::map_fold::closure_env$0<country_core::resources::walls::WallId,tuple$<country_core::resources::walls::WallId,tuple$<> >,tuple$<>,hashbrown::set::impl$11::extend::closure_env$0<country_core::resources::walls::WallId,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::filter::Filter<core::iter::adapters::cloned::Cloned<std::collections::hash::map::Keys<country_core::resources::walls::WallId,country_core::resources::walls::decorator_storage::Decorators> >,system_decorator::cull_oob_decorators::cull_oob_decorators::closure_env$0> >,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<country_core::resources::walls::WallId,tuple$<> >,hashbrown::map::impl$94::extend::closure_env$0<country_core::resources::walls::WallId,tuple$<>,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::filter::Filter<core::iter::adapters::cloned::Cloned<std::collections::hash::map::Keys<country_core::resources::walls::WallId,country_core::resources::walls::decorator_storage::Decorators> >,system_decorator::cull_oob_decorators::cull_oob_decorators::closure_env$0>,hashbrown::set::impl$11::extend::closure_env$0<country_core::resources::walls::WallId,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::filter::Filter<core::iter::adapters::cloned::Cloned<std::collections::hash::map::Keys<country_core::resources::walls::WallId,country_core::resources::walls::decorator_storage::Decorators> >,system_decorator::cull_oob_decorators::cull_oob_decorators::closure_env$0> > > > > > > > > >,tuple$<> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_decorator::cull_oob_decorators::cull_oob_decorators::closure_env$0 <- hashbrown::raw::RawIterRange::fold_impl<tuple$<country_core::resources::walls::WallId,country_core::resources::walls::decorator_storage::Decorators>,hashbrown::map::impl$60::fold::closure_env$0<country_core::resources::walls::WallId,country_core::resources::walls::decorator_storage::Decorators,tuple$<>,std::collections::hash::map::impl$50::fold::closure_env$0<country_core::resources::walls::WallId,country_core::resources::walls::decorator_storage::Decorators,tuple$<>,core::iter::adapters::map::map_fold::closure_env$0<ref$<country_core::resources::walls::WallId>,country_core::resources::walls::WallId,tuple$<>,country_core::resources::walls::WallId (*)(ref$<country_core::resources::walls::WallId>),core::iter::adapters::filter::filter_fold::closure_env$0<country_core::resources::walls::WallId,tuple$<>,system_decorator::cull_oob_decorators::cull_oob_decorators::closure_env$0,core::iter::adapters::map::map_fold::closure_env$0<country_core::resources::walls::WallId,tuple$<country_core::resources::walls::WallId,tuple$<> >,tuple$<>,hashbrown::set::impl$11::extend::closure_env$0<country_core::resources::walls::WallId,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::filter::Filter<core::iter::adapters::cloned::Cloned<std::collections::hash::map::Keys<country_core::resources::walls::WallId,country_core::resources::walls::decorator_storage::Decorators> >,system_decorator::cull_oob_decorators::cull_oob_decorators::closure_env$0> >,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<country_core::resources::walls::WallId,tuple$<> >,hashbrown::map::impl$94::extend::closure_env$0<country_core::resources::walls::WallId,tuple$<>,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::filter::Filter<core::iter::adapters::cloned::Cloned<std::collections::hash::map::Keys<country_core::resources::walls::WallId,country_core::resources::walls::decorator_storage::Decorators> >,system_decorator::cull_oob_decorators::cull_oob_decorators::closure_env$0>,hashbrown::set::impl$11::extend::closure_env$0<country_core::resources::walls::WallId,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::filter::Filter<core::iter::adapters::cloned::Cloned<std::collections::hash::map::Keys<country_core::resources::walls::WallId,country_core::resources::walls::decorator_storage::Decorators> >,system_decorator::cull_oob_decorators::cull_oob_decorators::closure_env$0> > > > > > > > > >,tuple$<> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_decorator::cull_oob_decorators::cull_oob_decorators::closure_env$0 <- hashbrown::raw::RawIterRange::fold_impl<tuple$<country_core::resources::walls::WallId,country_core::resources::walls::decorator_storage::Decorators>,hashbrown::map::impl$60::fold::closure_env$0<country_core::resources::walls::WallId,country_core::resources::walls::decorator_storage::Decorators,tuple$<>,std::collections::hash::map::impl$50::fold::closure_env$0<country_core::resources::walls::WallId,country_core::resources::walls::decorator_storage::Decorators,tuple$<>,core::iter::adapters::map::map_fold::closure_env$0<ref$<country_core::resources::walls::WallId>,country_core::resources::walls::WallId,tuple$<>,country_core::resources::walls::WallId (*)(ref$<country_core::resources::walls::WallId>),core::iter::adapters::filter::filter_fold::closure_env$0<country_core::resources::walls::WallId,tuple$<>,system_decorator::cull_oob_decorators::cull_oob_decorators::closure_env$0,core::iter::adapters::map::map_fold::closure_env$0<country_core::resources::walls::WallId,tuple$<country_core::resources::walls::WallId,tuple$<> >,tuple$<>,hashbrown::set::impl$11::extend::closure_env$0<country_core::resources::walls::WallId,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::filter::Filter<core::iter::adapters::cloned::Cloned<std::collections::hash::map::Keys<country_core::resources::walls::WallId,country_core::resources::walls::decorator_storage::Decorators> >,system_decorator::cull_oob_decorators::cull_oob_decorators::closure_env$0> >,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<country_core::resources::walls::WallId,tuple$<> >,hashbrown::map::impl$94::extend::closure_env$0<country_core::resources::walls::WallId,tuple$<>,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::filter::Filter<core::iter::adapters::cloned::Cloned<std::collections::hash::map::Keys<country_core::resources::walls::WallId,country_core::resources::walls::decorator_storage::Decorators> >,system_decorator::cull_oob_decorators::cull_oob_decorators::closure_env$0>,hashbrown::set::impl$11::extend::closure_env$0<country_core::resources::walls::WallId,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::filter::Filter<core::iter::adapters::cloned::Cloned<std::collections::hash::map::Keys<country_core::resources::walls::WallId,country_core::resources::walls::decorator_storage::Decorators> >,system_decorator::cull_oob_decorators::cull_oob_decorators::closure_env$0> > > > > > > > > >,tuple$<> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_decorator::cull_oob_decorators::cull_oob_decorators::closure_env$0 <- hashbrown::raw::RawIterRange::fold_impl<tuple$<country_core::resources::walls::WallId,country_core::resources::walls::decorator_storage::Decorators>,hashbrown::map::impl$60::fold::closure_env$0<country_core::resources::walls::WallId,country_core::resources::walls::decorator_storage::Decorators,tuple$<>,std::collections::hash::map::impl$50::fold::closure_env$0<country_core::resources::walls::WallId,country_core::resources::walls::decorator_storage::Decorators,tuple$<>,core::iter::adapters::map::map_fold::closure_env$0<ref$<country_core::resources::walls::WallId>,country_core::resources::walls::WallId,tuple$<>,country_core::resources::walls::WallId (*)(ref$<country_core::resources::walls::WallId>),core::iter::adapters::filter::filter_fold::closure_env$0<country_core::resources::walls::WallId,tuple$<>,system_decorator::cull_oob_decorators::cull_oob_decorators::closure_env$0,core::iter::adapters::map::map_fold::closure_env$0<country_core::resources::walls::WallId,tuple$<country_core::resources::walls::WallId,tuple$<> >,tuple$<>,hashbrown::set::impl$11::extend::closure_env$0<country_core::resources::walls::WallId,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::filter::Filter<core::iter::adapters::cloned::Cloned<std::collections::hash::map::Keys<country_core::resources::walls::WallId,country_core::resources::walls::decorator_storage::Decorators> >,system_decorator::cull_oob_decorators::cull_oob_decorators::closure_env$0> >,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<country_core::resources::walls::WallId,tuple$<> >,hashbrown::map::impl$94::extend::closure_env$0<country_core::resources::walls::WallId,tuple$<>,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::filter::Filter<core::iter::adapters::cloned::Cloned<std::collections::hash::map::Keys<country_core::resources::walls::WallId,country_core::resources::walls::decorator_storage::Decorators> >,system_decorator::cull_oob_decorators::cull_oob_decorators::closure_env$0>,hashbrown::set::impl$11::extend::closure_env$0<country_core::resources::walls::WallId,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::filter::Filter<core::iter::adapters::cloned::Cloned<std::collections::hash::map::Keys<country_core::resources::walls::WallId,country_core::resources::walls::decorator_storage::Decorators> >,system_decorator::cull_oob_decorators::cull_oob_decorators::closure_env$0> > > > > > > > > >,tuple$<> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 4 条。
// [PDB 内联宿主] hashbrown::raw::RawIterRange::fold_impl<tuple$<country_core::resources::walls::WallId,country_core::resources::walls::decorator_storage::Decorators>,hashbrown::map::impl$60::fold::closure_env$0<country_core::resources::walls::WallId,country_core::resources::walls::decorator_storage::Decorators,tuple$<>,std::collections::hash::map::impl$50::fold::closure_env$0<country_core::resources::walls::WallId,country_core::resources::walls::decorator_storage::Decorators,tuple$<>,core::iter::adapters::map::map_fold::closure_env$0<ref$<country_core::resources::walls::WallId>,country_core::resources::walls::WallId,tuple$<>,country_core::resources::walls::WallId (*)(ref$<country_core::resources::walls::WallId>),core::iter::adapters::filter::filter_fold::closure_env$0<country_core::resources::walls::WallId,tuple$<>,system_decorator::cull_oob_decorators::cull_oob_decorators::closure_env$0,core::iter::adapters::map::map_fold::closure_env$0<country_core::resources::walls::WallId,tuple$<country_core::resources::walls::WallId,tuple$<> >,tuple$<>,hashbrown::set::impl$11::extend::closure_env$0<country_core::resources::walls::WallId,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::filter::Filter<core::iter::adapters::cloned::Cloned<std::collections::hash::map::Keys<country_core::resources::walls::WallId,country_core::resources::walls::decorator_storage::Decorators> >,system_decorator::cull_oob_decorators::cull_oob_decorators::closure_env$0> >,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<country_core::resources::walls::WallId,tuple$<> >,hashbrown::map::impl$94::extend::closure_env$0<country_core::resources::walls::WallId,tuple$<>,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::filter::Filter<core::iter::adapters::cloned::Cloned<std::collections::hash::map::Keys<country_core::resources::walls::WallId,country_core::resources::walls::decorator_storage::Decorators> >,system_decorator::cull_oob_decorators::cull_oob_decorators::closure_env$0>,hashbrown::set::impl$11::extend::closure_env$0<country_core::resources::walls::WallId,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::filter::Filter<core::iter::adapters::cloned::Cloned<std::collections::hash::map::Keys<country_core::resources::walls::WallId,country_core::resources::walls::decorator_storage::Decorators> >,system_decorator::cull_oob_decorators::cull_oob_decorators::closure_env$0> > > > > > > > > >,tuple$<> >; module=1276, inline_record=83468; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::cull_oob_decorators::impl$1::clone
// 信息 ID: aa4aa8ac2c45f0318fa7dd26
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::iter::adapters::copied::impl$1::try_fold<std::collections::hash::set::Iter<country_core::resources::walls::decorator::DecoratorLocator>,country_core::resources::walls::decorator::DecoratorLocator,tuple$<>,core::iter::traits::iterator::Iterator::all::check::closure_env$0<country_core::resources::walls::decorator::DecoratorLocator,system_decorator::blueprint::validate_blueprints::closure_env$0>,enum2$<core::ops::control_flow::ControlFlow<tuple$<>,tuple$<> > > >; module=3832, inline_record=3624; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::cull_oob_decorators::is_decorator_location_valid
// 信息 ID: 6e7242a572dc8accd70a1950
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::cull_oob_decorators::cull_oob_decorators @ 0x4ffe53 -> 0x500840 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::iter::adapters::copied::impl$1::try_fold<std::collections::hash::set::Iter<country_core::resources::walls::decorator::DecoratorLocator>,country_core::resources::walls::decorator::DecoratorLocator,tuple$<>,core::iter::traits::iterator::Iterator::all::check::closure_env$0<country_core::resources::walls::decorator::DecoratorLocator,system_decorator::blueprint::validate_blueprints::closure_env$0>,enum2$<core::ops::control_flow::ControlFlow<tuple$<>,tuple$<> > > > @ 0x1c350ca -> 0x500840 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::cull_oob_decorators::is_wall_decorator_location_valid
// 信息 ID: 1bd7b6bcdeffa7f549259522
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::cull_oob_decorators::is_decorator_location_valid; module=383, inline_record=22396; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::cull_oob_decorators::is_wall_decorator_location_valid::closure$0
// 信息 ID: 38cf65a725704f1889cefdd9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_decorator::cull_oob_decorators::is_wall_decorator_location_valid", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::cull_oob_decorators::is_decorator_location_valid; module=383, inline_record=23224; 不是独立调用边。
// [PDB 内联宿主] system_decorator::cull_oob_decorators::is_decorator_location_valid; module=383, inline_record=23532; 不是独立调用边。
// [PDB 内联宿主] system_decorator::cull_oob_decorators::is_decorator_location_valid; module=383, inline_record=23840; 不是独立调用边。
// [PDB 内联宿主] system_decorator::cull_oob_decorators::is_decorator_location_valid; module=383, inline_record=27648; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::cull_oob_decorators::is_wall_decorator_location_valid::closure$1
// 信息 ID: 6764f5c57a9cf5044be0e5f3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "system_decorator::cull_oob_decorators::is_wall_decorator_location_valid", "captures": "unknown"}
// [闭包环境类型引用] system_decorator::cull_oob_decorators::is_wall_decorator_location_valid::closure_env$1 <- itertools::tuple_impl::impl$6::next<core::iter::adapters::map::Map<itertools::tuple_impl::TupleWindows<core::iter::adapters::map::Map<core::slice::iter::Iter<utils::curve::CurveInternalCoord<utils::curve::CurveCoordSemanticThresholded3D> >,system_decorator::cull_oob_decorators::is_wall_decorator_location_valid::closure_env$1>,tuple$<glam::f32::vec3::Vec3,glam::f32::vec3::Vec3> >,system_decorator::cull_oob_decorators::is_wall_decorator_location_valid::closure_env$2>,tuple$<glam::f32::vec2::Vec2,glam::f32::vec2::Vec2> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_decorator::cull_oob_decorators::is_wall_decorator_location_valid::closure_env$1 <- itertools::tuple_impl::impl$6::next<core::iter::adapters::map::Map<core::slice::iter::Iter<utils::curve::CurveInternalCoord<utils::curve::CurveCoordSemanticThresholded3D> >,system_decorator::cull_oob_decorators::is_wall_decorator_location_valid::closure_env$1>,tuple$<glam::f32::vec3::Vec3,glam::f32::vec3::Vec3> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_decorator::cull_oob_decorators::is_wall_decorator_location_valid::closure_env$1 <- itertools::tuple_impl::tuple_windows<core::iter::adapters::map::Map<itertools::tuple_impl::TupleWindows<core::iter::adapters::map::Map<core::slice::iter::Iter<utils::curve::CurveInternalCoord<utils::curve::CurveCoordSemanticThresholded3D> >,system_decorator::cull_oob_decorators::is_wall_decorator_location_valid::closure_env$1>,tuple$<glam::f32::vec3::Vec3,glam::f32::vec3::Vec3> >,system_decorator::cull_oob_decorators::is_wall_decorator_location_valid::closure_env$2>,tuple$<glam::f32::vec2::Vec2,glam::f32::vec2::Vec2> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_decorator::cull_oob_decorators::is_wall_decorator_location_valid::closure_env$1 <- itertools::tuple_impl::tuple_windows<core::iter::adapters::map::Map<core::slice::iter::Iter<utils::curve::CurveInternalCoord<utils::curve::CurveCoordSemanticThresholded3D> >,system_decorator::cull_oob_decorators::is_wall_decorator_location_valid::closure_env$1>,tuple$<glam::f32::vec3::Vec3,glam::f32::vec3::Vec3> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_decorator::cull_oob_decorators::is_wall_decorator_location_valid::closure_env$1 <- itertools::tuple_impl::impl$26::collect_from_iter_no_buf<glam::f32::vec2::Vec2,core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::sources::once::Once<glam::f32::vec2::Vec2>,core::iter::sources::once::Once<glam::f32::vec2::Vec2> >,ref_mut$<core::iter::adapters::map::Map<itertools::tuple_impl::TupleWindows<core::iter::adapters::map::Map<core::slice::iter::Iter<utils::curve::CurveInternalCoord<utils::curve::CurveCoordSemanticThresholded3D> >,system_decorator::cull_oob_decorators::is_wall_decorator_location_valid::closure_env$1>,tuple$<glam::f32::vec3::Vec3,glam::f32::vec3::Vec3> >,system_decorator::cull_oob_decorators::is_wall_decorator_location_valid::closure_env$2> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_decorator::cull_oob_decorators::is_wall_decorator_location_valid::closure_env$1 <- itertools::tuple_impl::impl$26::collect_from_iter_no_buf<glam::f32::vec3::Vec3,core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::sources::once::Once<glam::f32::vec3::Vec3>,core::iter::sources::once::Once<glam::f32::vec3::Vec3> >,ref_mut$<core::iter::adapters::map::Map<core::slice::iter::Iter<utils::curve::CurveInternalCoord<utils::curve::CurveCoordSemanticThresholded3D> >,system_decorator::cull_oob_decorators::is_wall_decorator_location_valid::closure_env$1> > > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 7 条；闭包环境类型引用 6 条。
// [PDB 内联宿主] itertools::tuple_impl::impl$6::next<core::iter::adapters::map::Map<itertools::tuple_impl::TupleWindows<core::iter::adapters::map::Map<core::slice::iter::Iter<utils::curve::CurveInternalCoord<utils::curve::CurveCoordSemanticThresholded3D> >,system_decorator::cull_oob_decorators::is_wall_decorator_location_valid::closure_env$1>,tuple$<glam::f32::vec3::Vec3,glam::f32::vec3::Vec3> >,system_decorator::cull_oob_decorators::is_wall_decorator_location_valid::closure_env$2>,tuple$<glam::f32::vec2::Vec2,glam::f32::vec2::Vec2> >; module=1284, inline_record=1156; 不是独立调用边。
// [PDB 内联宿主] itertools::tuple_impl::impl$6::next<core::iter::adapters::map::Map<core::slice::iter::Iter<utils::curve::CurveInternalCoord<utils::curve::CurveCoordSemanticThresholded3D> >,system_decorator::cull_oob_decorators::is_wall_decorator_location_valid::closure_env$1>,tuple$<glam::f32::vec3::Vec3,glam::f32::vec3::Vec3> >; module=1284, inline_record=3104; 不是独立调用边。
// [PDB 内联宿主] itertools::tuple_impl::tuple_windows<core::iter::adapters::map::Map<itertools::tuple_impl::TupleWindows<core::iter::adapters::map::Map<core::slice::iter::Iter<utils::curve::CurveInternalCoord<utils::curve::CurveCoordSemanticThresholded3D> >,system_decorator::cull_oob_decorators::is_wall_decorator_location_valid::closure_env$1>,tuple$<glam::f32::vec3::Vec3,glam::f32::vec3::Vec3> >,system_decorator::cull_oob_decorators::is_wall_decorator_location_valid::closure_env$2>,tuple$<glam::f32::vec2::Vec2,glam::f32::vec2::Vec2> >; module=1284, inline_record=4872; 不是独立调用边。
// [PDB 内联宿主] itertools::tuple_impl::tuple_windows<core::iter::adapters::map::Map<core::slice::iter::Iter<utils::curve::CurveInternalCoord<utils::curve::CurveCoordSemanticThresholded3D> >,system_decorator::cull_oob_decorators::is_wall_decorator_location_valid::closure_env$1>,tuple$<glam::f32::vec3::Vec3,glam::f32::vec3::Vec3> >; module=1284, inline_record=6732; 不是独立调用边。
// [PDB 内联宿主] itertools::tuple_impl::impl$26::collect_from_iter_no_buf<glam::f32::vec3::Vec3,core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::sources::once::Once<glam::f32::vec3::Vec3>,core::iter::sources::once::Once<glam::f32::vec3::Vec3> >,ref_mut$<core::iter::adapters::map::Map<core::slice::iter::Iter<utils::curve::CurveInternalCoord<utils::curve::CurveCoordSemanticThresholded3D> >,system_decorator::cull_oob_decorators::is_wall_decorator_location_valid::closure_env$1> > > >; module=2767, inline_record=3884; 不是独立调用边。
// [PDB 内联宿主] itertools::tuple_impl::impl$26::collect_from_iter_no_buf<glam::f32::vec3::Vec3,core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::sources::once::Once<glam::f32::vec3::Vec3>,core::iter::sources::once::Once<glam::f32::vec3::Vec3> >,ref_mut$<core::iter::adapters::map::Map<core::slice::iter::Iter<utils::curve::CurveInternalCoord<utils::curve::CurveCoordSemanticThresholded3D> >,system_decorator::cull_oob_decorators::is_wall_decorator_location_valid::closure_env$1> > > >; module=2767, inline_record=5272; 不是独立调用边。
// [PDB 内联宿主] itertools::tuple_impl::impl$26::collect_from_iter_no_buf<glam::f32::vec3::Vec3,core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::sources::once::Once<glam::f32::vec3::Vec3>,core::iter::sources::once::Once<glam::f32::vec3::Vec3> >,ref_mut$<core::iter::adapters::map::Map<core::slice::iter::Iter<utils::curve::CurveInternalCoord<utils::curve::CurveCoordSemanticThresholded3D> >,system_decorator::cull_oob_decorators::is_wall_decorator_location_valid::closure_env$1> > > >; module=2767, inline_record=6040; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::cull_oob_decorators::is_wall_decorator_location_valid::closure$2
// 信息 ID: c0ed1c27905dd24e88065e94
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "system_decorator::cull_oob_decorators::is_wall_decorator_location_valid", "captures": "unknown"}
// [闭包环境类型引用] system_decorator::cull_oob_decorators::is_wall_decorator_location_valid::closure_env$2 <- itertools::tuple_impl::impl$6::next<core::iter::adapters::map::Map<itertools::tuple_impl::TupleWindows<core::iter::adapters::map::Map<core::slice::iter::Iter<utils::curve::CurveInternalCoord<utils::curve::CurveCoordSemanticThresholded3D> >,system_decorator::cull_oob_decorators::is_wall_decorator_location_valid::closure_env$1>,tuple$<glam::f32::vec3::Vec3,glam::f32::vec3::Vec3> >,system_decorator::cull_oob_decorators::is_wall_decorator_location_valid::closure_env$2>,tuple$<glam::f32::vec2::Vec2,glam::f32::vec2::Vec2> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_decorator::cull_oob_decorators::is_wall_decorator_location_valid::closure_env$2 <- itertools::tuple_impl::tuple_windows<core::iter::adapters::map::Map<itertools::tuple_impl::TupleWindows<core::iter::adapters::map::Map<core::slice::iter::Iter<utils::curve::CurveInternalCoord<utils::curve::CurveCoordSemanticThresholded3D> >,system_decorator::cull_oob_decorators::is_wall_decorator_location_valid::closure_env$1>,tuple$<glam::f32::vec3::Vec3,glam::f32::vec3::Vec3> >,system_decorator::cull_oob_decorators::is_wall_decorator_location_valid::closure_env$2>,tuple$<glam::f32::vec2::Vec2,glam::f32::vec2::Vec2> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_decorator::cull_oob_decorators::is_wall_decorator_location_valid::closure_env$2 <- itertools::tuple_impl::impl$26::collect_from_iter_no_buf<glam::f32::vec2::Vec2,core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::sources::once::Once<glam::f32::vec2::Vec2>,core::iter::sources::once::Once<glam::f32::vec2::Vec2> >,ref_mut$<core::iter::adapters::map::Map<itertools::tuple_impl::TupleWindows<core::iter::adapters::map::Map<core::slice::iter::Iter<utils::curve::CurveInternalCoord<utils::curve::CurveCoordSemanticThresholded3D> >,system_decorator::cull_oob_decorators::is_wall_decorator_location_valid::closure_env$1>,tuple$<glam::f32::vec3::Vec3,glam::f32::vec3::Vec3> >,system_decorator::cull_oob_decorators::is_wall_decorator_location_valid::closure_env$2> > > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 3 条。
// [PDB 内联宿主] itertools::tuple_impl::impl$6::next<core::iter::adapters::map::Map<itertools::tuple_impl::TupleWindows<core::iter::adapters::map::Map<core::slice::iter::Iter<utils::curve::CurveInternalCoord<utils::curve::CurveCoordSemanticThresholded3D> >,system_decorator::cull_oob_decorators::is_wall_decorator_location_valid::closure_env$1>,tuple$<glam::f32::vec3::Vec3,glam::f32::vec3::Vec3> >,system_decorator::cull_oob_decorators::is_wall_decorator_location_valid::closure_env$2>,tuple$<glam::f32::vec2::Vec2,glam::f32::vec2::Vec2> >; module=1284, inline_record=1988; 不是独立调用边。
// [PDB 内联宿主] itertools::tuple_impl::tuple_windows<core::iter::adapters::map::Map<itertools::tuple_impl::TupleWindows<core::iter::adapters::map::Map<core::slice::iter::Iter<utils::curve::CurveInternalCoord<utils::curve::CurveCoordSemanticThresholded3D> >,system_decorator::cull_oob_decorators::is_wall_decorator_location_valid::closure_env$1>,tuple$<glam::f32::vec3::Vec3,glam::f32::vec3::Vec3> >,system_decorator::cull_oob_decorators::is_wall_decorator_location_valid::closure_env$2>,tuple$<glam::f32::vec2::Vec2,glam::f32::vec2::Vec2> >; module=1284, inline_record=5704; 不是独立调用边。
// [PDB 内联宿主] itertools::tuple_impl::impl$26::collect_from_iter_no_buf<glam::f32::vec2::Vec2,core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::sources::once::Once<glam::f32::vec2::Vec2>,core::iter::sources::once::Once<glam::f32::vec2::Vec2> >,ref_mut$<core::iter::adapters::map::Map<itertools::tuple_impl::TupleWindows<core::iter::adapters::map::Map<core::slice::iter::Iter<utils::curve::CurveInternalCoord<utils::curve::CurveCoordSemanticThresholded3D> >,system_decorator::cull_oob_decorators::is_wall_decorator_location_valid::closure_env$1>,tuple$<glam::f32::vec3::Vec3,glam::f32::vec3::Vec3> >,system_decorator::cull_oob_decorators::is_wall_decorator_location_valid::closure_env$2> > > >; module=2742, inline_record=1880; 不是独立调用边。
// [PDB 内联宿主] itertools::tuple_impl::impl$26::collect_from_iter_no_buf<glam::f32::vec2::Vec2,core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::sources::once::Once<glam::f32::vec2::Vec2>,core::iter::sources::once::Once<glam::f32::vec2::Vec2> >,ref_mut$<core::iter::adapters::map::Map<itertools::tuple_impl::TupleWindows<core::iter::adapters::map::Map<core::slice::iter::Iter<utils::curve::CurveInternalCoord<utils::curve::CurveCoordSemanticThresholded3D> >,system_decorator::cull_oob_decorators::is_wall_decorator_location_valid::closure_env$1>,tuple$<glam::f32::vec3::Vec3,glam::f32::vec3::Vec3> >,system_decorator::cull_oob_decorators::is_wall_decorator_location_valid::closure_env$2> > > >; module=2742, inline_record=2920; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::cull_oob_decorators::is_wall_decorator_location_valid::closure$3
// 信息 ID: 79c445d49c41c8ccf4be7c05
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 3, "lexical_owner": "system_decorator::cull_oob_decorators::is_wall_decorator_location_valid", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::cull_oob_decorators::is_decorator_location_valid; module=383, inline_record=26404; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::cull_oob_decorators::should_snap_to_corner
// 信息 ID: 699a9909d0f4a770eb6bb6c5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_interaction_intent @ 0xd0ac6b -> 0x5029f0 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] system_decorator::cull_oob_decorators::is_decorator_location_valid; module=383, inline_record=26752; 不是独立调用边。
// [PDB 内联宿主] system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_interaction_intent; module=1286, inline_record=4032; 不是独立调用边。
// [PDB 内联宿主] system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_interaction_intent; module=1286, inline_record=4160; 不是独立调用边。
// [PDB 内联宿主] system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_interaction_intent; module=1286, inline_record=4292; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
