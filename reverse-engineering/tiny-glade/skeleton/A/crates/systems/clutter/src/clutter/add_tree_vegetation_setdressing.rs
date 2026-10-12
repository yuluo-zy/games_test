// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/systems/clutter/src/clutter/add_tree_vegetation_setdressing.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 8dcb52decef5f6c410f9876b094aca8b98060375a2714174a9d78f8dcb0263d2
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: add_tree_vegetation_setdressing.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 3514
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
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"PDB_registry_path_only","version":"0.27.0"}]
// use lerp; // namespace candidate
//   package identities: [{"name":"lerp","source":"PDB_registry_path_only","version":"0.4.0"}]
// use std; // namespace candidate
//   package identities: []
// use utils; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_clutter::clutter::add_tree_vegetation_setdressing::add_tree_setdressing
// 记录形式: compiled_procedure; instances=1
// 源码行: 19
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3514,"record_offset":252,"rva":28130144,"va":5396839264}
// 编译实例 RVAs: 0x1ad3b60
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14043c290
//     owner=unresolved_namespace; count=1
//     target_aliases=53 candidates; complete list: add_tree_vegetation_setdressing.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-23689561","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396839264,"raw_hex":"e8a78696fe","size":5,"soft_fail":false,"target_va":5373149840,"va":5396839396,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14054a000
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: add_tree_vegetation_setdressing.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-22584865","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396839264,"raw_hex":"e8df61a7fe","size":5,"soft_fail":false,"target_va":5374255104,"va":5396839964,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1407da760
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: add_tree_vegetation_setdressing.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-19897060","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396839264,"raw_hex":"e81c65d0fe","size":5,"soft_fail":false,"target_va":5376943968,"va":5396841023,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141adee40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: add_tree_vegetation_setdressing.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t43960","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396839264,"raw_hex":"e8b8ab0000","size":5,"soft_fail":false,"target_va":5396885056,"va":5396841091,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144da0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: add_tree_vegetation_setdressing.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t6753859","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396839264,"raw_hex":"e8430e6700","size":5,"soft_fail":false,"target_va":5403594144,"va":5396840280,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-25532723","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396839264,"raw_hex":"e8cd667afe","size":5,"soft_fail":false,"target_va":5371308800,"va":5396841518,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t6137383","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396839264,"raw_hex":"e827a65d00","size":5,"soft_fail":false,"target_va":5402979152,"va":5396841764,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t6137199","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396839264,"raw_hex":"e86fa55d00","size":5,"soft_fail":false,"target_va":5402978944,"va":5396841740,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::placement_data::PlacementDataSsbo::position
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-22559845","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396839264,"raw_hex":"e89bc3a7fe","size":5,"soft_fail":false,"target_va":5374280016,"va":5396839856,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::session::GladeType::is_flowery
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-22584744","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396839264,"raw_hex":"e85862a7fe","size":5,"soft_fail":false,"target_va":5374255088,"va":5396839827,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::session::GladeType::is_winter
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t-22584867","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396839264,"raw_hex":"e8dd61a7fe","size":5,"soft_fail":false,"target_va":5374255120,"va":5396839982,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] fastrand::Rng::f32
//     owner=unresolved_namespace; count=6
//     evidence={"assembly":"\tcallq\t-10990222","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396839264,"raw_hex":"e8724d58ff","size":5,"soft_fail":false,"target_va":5385850032,"va":5396840249,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::circle::Circle2d::get_pos_at_u
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-18872439","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396839264,"raw_hex":"e88907e0fe","size":5,"soft_fail":false,"target_va":5377967920,"va":5396840354,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::circle::Circle2d::new
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-18872194","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396839264,"raw_hex":"e87e08e0fe","size":5,"soft_fail":false,"target_va":5377967856,"va":5396840045,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::lerp
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t-19900944","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396839264,"raw_hex":"e8f055d0fe","size":5,"soft_fail":false,"target_va":5376939952,"va":5396840891,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::rng::init_rng
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-14584402","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396839264,"raw_hex":"e8ae7521ff","size":5,"soft_fail":false,"target_va":5382255664,"va":5396840061,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3514,"record_offset":1384,"parent_record_offset":252,"ancestor_record_offset":252,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/03514.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":3514,"record_offset":3360,"parent_record_offset":252,"ancestor_record_offset":252,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/03514.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::query::iter::impl$1::next
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs
//     evidence={"module":3514,"record_offset":704,"parent_record_offset":252,"ancestor_record_offset":252,"inlinee":14613,"source_line":881,"archive":"_pdb/A/modules/03514.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::query::Query::iter
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/query.rs
//     evidence={"module":3514,"record_offset":504,"parent_record_offset":252,"ancestor_record_offset":252,"inlinee":14606,"source_line":435,"archive":"_pdb/A/modules/03514.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":3514,"record_offset":1736,"parent_record_offset":252,"ancestor_record_offset":252,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/03514.jsonl.gz"}
//   [pdb_inline_direct] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":3514,"record_offset":1524,"parent_record_offset":252,"ancestor_record_offset":252,"inlinee":4233,"source_line":844,"archive":"_pdb/A/modules/03514.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::session::ActiveSession::is_autumn
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":3514,"record_offset":1328,"parent_record_offset":252,"ancestor_record_offset":252,"inlinee":22564,"source_line":210,"archive":"_pdb/A/modules/03514.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::session::ActiveSession::is_flowery
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":3514,"record_offset":1300,"parent_record_offset":252,"ancestor_record_offset":252,"inlinee":27887,"source_line":220,"archive":"_pdb/A/modules/03514.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::session::ActiveSession::is_winter
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":3514,"record_offset":1356,"parent_record_offset":252,"ancestor_record_offset":252,"inlinee":12658,"source_line":205,"archive":"_pdb/A/modules/03514.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::world_raster::WorldRaster::as_float_grid
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/world_raster.rs
//     evidence={"module":3514,"record_offset":476,"parent_record_offset":252,"ancestor_record_offset":252,"inlinee":21417,"source_line":126,"archive":"_pdb/A/modules/03514.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::normalize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":3514,"record_offset":3064,"parent_record_offset":252,"ancestor_record_offset":252,"inlinee":21371,"source_line":450,"archive":"_pdb/A/modules/03514.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$13::add_assign
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":3514,"record_offset":2124,"parent_record_offset":252,"ancestor_record_offset":252,"inlinee":14662,"source_line":1012,"archive":"_pdb/A/modules/03514.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$9::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":3514,"record_offset":2068,"parent_record_offset":252,"ancestor_record_offset":252,"inlinee":14701,"source_line":972,"archive":"_pdb/A/modules/03514.jsonl.gz"}
//   [pdb_inline_direct] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lerp","version":"0.4.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lerp-0.4.0/src/lib.rs
//     evidence={"module":3514,"record_offset":3416,"parent_record_offset":252,"ancestor_record_offset":252,"inlinee":10380,"source_line":171,"archive":"_pdb/A/modules/03514.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u_circular::CurveUCircular::new
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u_circular.rs
//     evidence={"module":3514,"record_offset":1608,"parent_record_offset":252,"ancestor_record_offset":252,"inlinee":21570,"source_line":31,"archive":"_pdb/A/modules/03514.jsonl.gz"}
//   [pdb_inline_direct] utils::float_grid::impl$0::bilinear_sample_at_world_xz
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/float_grid.rs
//     evidence={"module":3514,"record_offset":2180,"parent_record_offset":252,"ancestor_record_offset":252,"inlinee":21423,"source_line":26,"archive":"_pdb/A/modules/03514.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3514,"record_offset":1460,"parent_record_offset":1428,"ancestor_record_offset":252,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/03514.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3514,"record_offset":1428,"parent_record_offset":1384,"ancestor_record_offset":252,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/03514.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$29::set_changed
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":3514,"record_offset":3384,"parent_record_offset":3360,"ancestor_record_offset":252,"inlinee":5812,"source_line":309,"archive":"_pdb/A/modules/03514.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$21::fetch
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":3514,"record_offset":1148,"parent_record_offset":1124,"ancestor_record_offset":252,"inlinee":14627,"source_line":963,"archive":"_pdb/A/modules/03514.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$21::set_table
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":3514,"record_offset":1036,"parent_record_offset":1012,"ancestor_record_offset":252,"inlinee":14620,"source_line":948,"archive":"_pdb/A/modules/03514.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIter::new
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs
//     evidence={"module":3514,"record_offset":556,"parent_record_offset":532,"ancestor_record_offset":252,"inlinee":14609,"source_line":36,"archive":"_pdb/A/modules/03514.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::init
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs
//     evidence={"module":3514,"record_offset":580,"parent_record_offset":556,"ancestor_record_offset":252,"inlinee":14611,"source_line":1520,"archive":"_pdb/A/modules/03514.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::next
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs
//     evidence={"module":3514,"record_offset":728,"parent_record_offset":704,"ancestor_record_offset":252,"inlinee":14614,"source_line":1587,"archive":"_pdb/A/modules/03514.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::state::QueryState::iter_unchecked_manual
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/state.rs
//     evidence={"module":3514,"record_offset":532,"parent_record_offset":504,"ancestor_record_offset":252,"inlinee":14607,"source_line":1240,"archive":"_pdb/A/modules/03514.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$2::fetch
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/world_query.rs
//     evidence={"module":3514,"record_offset":1124,"parent_record_offset":728,"ancestor_record_offset":252,"inlinee":14626,"source_line":201,"archive":"_pdb/A/modules/03514.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$2::set_table
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/world_query.rs
//     evidence={"module":3514,"record_offset":1012,"parent_record_offset":728,"ancestor_record_offset":252,"inlinee":14619,"source_line":192,"archive":"_pdb/A/modules/03514.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_slice
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":3514,"record_offset":1084,"parent_record_offset":1060,"ancestor_record_offset":252,"inlinee":14643,"source_line":410,"archive":"_pdb/A/modules/03514.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Column::get_data_slice
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":3514,"record_offset":1060,"parent_record_offset":1036,"ancestor_record_offset":252,"inlinee":14642,"source_line":321,"archive":"_pdb/A/modules/03514.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::entities
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":3514,"record_offset":984,"parent_record_offset":728,"ancestor_record_offset":252,"inlinee":14616,"source_line":568,"archive":"_pdb/A/modules/03514.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::TableRow::as_usize
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":3514,"record_offset":1172,"parent_record_offset":1148,"ancestor_record_offset":252,"inlinee":14629,"source_line":125,"archive":"_pdb/A/modules/03514.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":3514,"record_offset":900,"parent_record_offset":728,"ancestor_record_offset":252,"inlinee":14618,"source_line":834,"archive":"_pdb/A/modules/03514.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ptr::ThinSlicePtr::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ptr/src/lib.rs
//     evidence={"module":3514,"record_offset":1228,"parent_record_offset":1204,"ancestor_record_offset":252,"inlinee":21393,"source_line":500,"archive":"_pdb/A/modules/03514.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":3514,"record_offset":1880,"parent_record_offset":1856,"ancestor_record_offset":252,"inlinee":6743,"source_line":1676,"archive":"_pdb/A/modules/03514.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$50::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":3514,"record_offset":1904,"parent_record_offset":1880,"ancestor_record_offset":252,"inlinee":5484,"source_line":1517,"archive":"_pdb/A/modules/03514.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$54::lt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":3514,"record_offset":1572,"parent_record_offset":1548,"ancestor_record_offset":252,"inlinee":4237,"source_line":1553,"archive":"_pdb/A/modules/03514.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":3514,"record_offset":2584,"parent_record_offset":2180,"ancestor_record_offset":252,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/03514.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::is_nan
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":3514,"record_offset":1644,"parent_record_offset":1608,"ancestor_record_offset":252,"inlinee":5051,"source_line":502,"archive":"_pdb/A/modules/03514.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":3514,"record_offset":2428,"parent_record_offset":2400,"ancestor_record_offset":252,"inlinee":5741,"source_line":924,"archive":"_pdb/A/modules/03514.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::min
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":3514,"record_offset":2456,"parent_record_offset":2400,"ancestor_record_offset":252,"inlinee":4665,"source_line":944,"archive":"_pdb/A/modules/03514.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":3514,"record_offset":1972,"parent_record_offset":1792,"ancestor_record_offset":252,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/03514.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":3514,"record_offset":1548,"parent_record_offset":1524,"ancestor_record_offset":252,"inlinee":4235,"source_line":753,"archive":"_pdb/A/modules/03514.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":3514,"record_offset":2092,"parent_record_offset":2068,"ancestor_record_offset":252,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/03514.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":3514,"record_offset":2344,"parent_record_offset":2320,"ancestor_record_offset":252,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/03514.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$216::div
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":3514,"record_offset":2288,"parent_record_offset":2264,"ancestor_record_offset":252,"inlinee":5488,"source_line":502,"archive":"_pdb/A/modules/03514.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$308::add_assign
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":3514,"record_offset":2148,"parent_record_offset":2124,"ancestor_record_offset":252,"inlinee":14664,"source_line":761,"archive":"_pdb/A/modules/03514.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":3514,"record_offset":3000,"parent_record_offset":2972,"ancestor_record_offset":252,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/03514.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\range.rs
//     evidence={"module":3514,"record_offset":1856,"parent_record_offset":1832,"ancestor_record_offset":252,"inlinee":6738,"source_line":817,"archive":"_pdb/A/modules/03514.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\range.rs
//     evidence={"module":3514,"record_offset":1832,"parent_record_offset":1792,"ancestor_record_offset":252,"inlinee":7456,"source_line":509,"archive":"_pdb/A/modules/03514.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3514,"record_offset":652,"parent_record_offset":628,"ancestor_record_offset":252,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/03514.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":3514,"record_offset":864,"parent_record_offset":840,"ancestor_record_offset":252,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/03514.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":3514,"record_offset":812,"parent_record_offset":776,"ancestor_record_offset":252,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/03514.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3514,"record_offset":1492,"parent_record_offset":1384,"ancestor_record_offset":252,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/03514.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":3514,"record_offset":924,"parent_record_offset":900,"ancestor_record_offset":252,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/03514.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":3514,"record_offset":604,"parent_record_offset":580,"ancestor_record_offset":252,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/03514.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":3514,"record_offset":948,"parent_record_offset":924,"ancestor_record_offset":252,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/03514.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":3514,"record_offset":628,"parent_record_offset":604,"ancestor_record_offset":252,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/03514.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":3514,"record_offset":840,"parent_record_offset":776,"ancestor_record_offset":252,"inlinee":5091,"source_line":101,"archive":"_pdb/A/modules/03514.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":3514,"record_offset":776,"parent_record_offset":728,"ancestor_record_offset":252,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/03514.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::world_raster::impl$3::load_index_unchecked
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/world_raster.rs
//     evidence={"module":3514,"record_offset":2704,"parent_record_offset":2668,"ancestor_record_offset":252,"inlinee":21432,"source_line":524,"archive":"_pdb/A/modules/03514.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::as_uvec2
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":3514,"record_offset":2548,"parent_record_offset":2180,"ancestor_record_offset":252,"inlinee":21428,"source_line":875,"archive":"_pdb/A/modules/03514.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::clamp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":3514,"record_offset":2376,"parent_record_offset":2180,"ancestor_record_offset":252,"inlinee":21426,"source_line":201,"archive":"_pdb/A/modules/03514.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":3514,"record_offset":3164,"parent_record_offset":3128,"ancestor_record_offset":252,"inlinee":14697,"source_line":157,"archive":"_pdb/A/modules/03514.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":3514,"record_offset":3128,"parent_record_offset":3088,"ancestor_record_offset":252,"inlinee":21373,"source_line":380,"archive":"_pdb/A/modules/03514.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_recip
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":3514,"record_offset":3088,"parent_record_offset":3064,"ancestor_record_offset":252,"inlinee":21372,"source_line":399,"archive":"_pdb/A/modules/03514.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::max
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":3514,"record_offset":2400,"parent_record_offset":2376,"ancestor_record_offset":252,"inlinee":21427,"source_line":185,"archive":"_pdb/A/modules/03514.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$14::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":3514,"record_offset":2320,"parent_record_offset":2180,"ancestor_record_offset":252,"inlinee":17347,"source_line":1021,"archive":"_pdb/A/modules/03514.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$17::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":3514,"record_offset":2972,"parent_record_offset":2180,"ancestor_record_offset":252,"inlinee":14656,"source_line":1051,"archive":"_pdb/A/modules/03514.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$2::div
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":3514,"record_offset":2264,"parent_record_offset":2180,"ancestor_record_offset":252,"inlinee":21425,"source_line":904,"archive":"_pdb/A/modules/03514.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$7::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":3514,"record_offset":2492,"parent_record_offset":2180,"ancestor_record_offset":252,"inlinee":17349,"source_line":953,"archive":"_pdb/A/modules/03514.jsonl.gz"}
//   [pdb_inline_transitive] glam::u32::uvec2::UVec2::as_vec2
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/u32/uvec2.rs
//     evidence={"module":3514,"record_offset":2640,"parent_record_offset":2180,"ancestor_record_offset":252,"inlinee":17196,"source_line":302,"archive":"_pdb/A/modules/03514.jsonl.gz"}
//   [pdb_inline_transitive] glam::u32::uvec2::impl$65::from
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/u32/uvec2.rs
//     evidence={"module":3514,"record_offset":2608,"parent_record_offset":2584,"ancestor_record_offset":252,"inlinee":22963,"source_line":1123,"archive":"_pdb/A/modules/03514.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":3514,"record_offset":3200,"parent_record_offset":3164,"ancestor_record_offset":252,"inlinee":13701,"source_line":441,"archive":"_pdb/A/modules/03514.jsonl.gz"}
//   [pdb_inline_transitive] utils::comparative_space::impl$3::project_unbounded
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/comparative_space.rs
//     evidence={"module":3514,"record_offset":1696,"parent_record_offset":1672,"ancestor_record_offset":252,"inlinee":7458,"source_line":154,"archive":"_pdb/A/modules/03514.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u::CurveU::new
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":3514,"record_offset":1792,"parent_record_offset":1764,"ancestor_record_offset":252,"inlinee":14654,"source_line":87,"archive":"_pdb/A/modules/03514.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u_circular::CurveUCircular::wrap_around
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u_circular.rs
//     evidence={"module":3514,"record_offset":1672,"parent_record_offset":1608,"ancestor_record_offset":252,"inlinee":21571,"source_line":90,"archive":"_pdb/A/modules/03514.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u_circular::impl$3::from
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u_circular.rs
//     evidence={"module":3514,"record_offset":1764,"parent_record_offset":1736,"ancestor_record_offset":252,"inlinee":26825,"source_line":115,"archive":"_pdb/A/modules/03514.jsonl.gz"}
//   [pdb_inline_transitive] utils::float_grid::FloatGrid::load_unchecked
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/float_grid.rs
//     evidence={"module":3514,"record_offset":2668,"parent_record_offset":2180,"ancestor_record_offset":252,"inlinee":21430,"source_line":8,"archive":"_pdb/A/modules/03514.jsonl.gz"}
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
// 完整结构及原始类型记录: add_tree_vegetation_setdressing.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: add_tree_vegetation_setdressing.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: add_tree_vegetation_setdressing.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<system_clutter::clutter::ClutterState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<system_clutter::clutter::load_clutter_meshes::ClutterMeshes>,bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::components::TreeComponent>,ref$<country_core::systems::prefab::PrefabInstanceData> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<system_clutter::clutter::ClutterState>", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>", "bevy_ecs::change_detection::Res<system_clutter::clutter::load_clutter_meshes::ClutterMeshes>", "bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>", "bevy_ecs::system::query::Query<tuple$<ref$<country_core::components::TreeComponent>,ref$<country_core::systems::prefab::PrefabInstanceData> >,tuple$<> >", "bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_clutter::clutter::ClutterState>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<system_clutter::clutter::load_clutter_meshes::ClutterMeshes>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<ref$<country_core::components::TreeComponent>,ref$<country_core::systems::prefab::PrefabInstanceData> >,tuple$<> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>

// ------------------------------------------------------------------
// 函数: system_clutter::clutter::add_tree_vegetation_setdressing::add_tree_setdressing
// 信息 ID: a253db35e2633717e7e9d463
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 9ee48561a8dfe825700bc46e: void (*)(bevy_ecs::change_detection::ResMut<system_clutter::clutter::ClutterState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<system_clutter::clutter::load_clutter_meshes::ClutterMeshes>,bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::components::TreeComponent>,ref$<country_core::systems::prefab::PrefabInstanceData> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<system_clutter::clutter::ClutterState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<system_clutter::clutter::load_clutter_meshes::ClutterMeshes>,bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::components::TreeComponent>,ref$<country_core::systems::prefab::PrefabInstanceData> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>),void (*)(bevy_ecs::change_detection::ResMut<system_clutter::clutter::ClutterState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<system_clutter::clutter::load_clutter_meshes::ClutterMeshes>,bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::components::TreeComponent>,ref$<country_core::systems::prefab::PrefabInstanceData> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>)> > @ 0x14c0ff6 -> 0x1ad3b60 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::change_detection::ResMut<system_clutter::clutter::ClutterState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<system_clutter::clutter::load_clutter_meshes::ClutterMeshes>,bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::components::TreeComponent>,ref$<country_core::systems::prefab::PrefabInstanceData> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>),void (*)(bevy_ecs::change_detection::ResMut<system_clutter::clutter::ClutterState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<system_clutter::clutter::load_clutter_meshes::ClutterMeshes>,bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::components::TreeComponent>,ref$<country_core::systems::prefab::PrefabInstanceData> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>)> @ 0x14e8f7e -> 0x1ad3b60 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
