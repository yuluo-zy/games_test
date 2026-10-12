// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/systems/critters/src/sheep/sheep_react_to_being_inside_buildings.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: c8e9d04013a2e7af7afc48f311b685defea1db7dd30a9d3c0e7b7d0b2620ce70
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: sheep_react_to_being_inside_buildings.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 3887
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use bevy_ecs; // namespace candidate
//   package identities: [{"name":"bevy_ecs","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.0"}]
// use bevy_platform; // namespace candidate
//   package identities: [{"name":"bevy_platform","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.0"}]
// use bevy_ptr; // namespace candidate
//   package identities: [{"name":"bevy_ptr","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.0"}]
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use equivalent; // namespace candidate
//   package identities: [{"name":"equivalent","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.1"}]
// use fastrand; // namespace candidate
//   package identities: [{"name":"fastrand","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.8.0"}]
// use fixedbitset; // namespace candidate
//   package identities: [{"name":"fixedbitset","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.5.7"}]
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.29.3"}]
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.3"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.5"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.15.2"}]
// use nonmax; // namespace candidate
//   package identities: [{"name":"nonmax","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.5.5"}]
// use std; // namespace candidate
//   package identities: []
// use system_critters; // namespace candidate
//   package identities: [{"name":"system-critters","source":"local_or_path","version":"0.1.0"}]
// use utils; // namespace candidate
//   package identities: [{"name":"utils","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_critters::sheep::sheep_react_to_being_inside_buildings::react_to_being_inside_buildings
// 记录形式: compiled_procedure; instances=1
// 源码行: 20
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3887,"record_offset":264,"rva":36828688,"va":5405537808}
// 编译实例 RVAs: 0x231f610
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: sheep_react_to_being_inside_buildings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-30458425","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405537808,"raw_hex":"e8c73d2ffe","size":5,"soft_fail":false,"target_va":5375081488,"va":5405539908,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140ad2950
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: sheep_react_to_being_inside_buildings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-25482076","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405537808,"raw_hex":"e8a42c7bfe","size":5,"soft_fail":false,"target_va":5380057424,"va":5405539495,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: sheep_react_to_being_inside_buildings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t6307214","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405537808,"raw_hex":"e88e3d6000","size":5,"soft_fail":false,"target_va":5411847168,"va":5405539949,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-31183292","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405537808,"raw_hex":"e8442e24fe","size":5,"soft_fail":false,"target_va":5374356528,"va":5405539815,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t6000766","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405537808,"raw_hex":"e87e905b00","size":5,"soft_fail":false,"target_va":5411540963,"va":5405540192,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::system::commands::Commands::spawn
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-5631456","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405537808,"raw_hex":"e82012aaff","size":5,"soft_fail":false,"target_va":5399908624,"va":5405540075,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t6001599","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405537808,"raw_hex":"e8bf935b00","size":5,"soft_fail":false,"target_va":5411541776,"va":5405540172,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::roofs::roof_shape::Roof::height
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-27511341","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405537808,"raw_hex":"e8d3355cfe","size":5,"soft_fail":false,"target_va":5378028128,"va":5405539464,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_wall_state::PublicWallState::get_wall_shape
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-25121398","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405537808,"raw_hex":"e88aad80fe","size":5,"soft_fail":false,"target_va":5380416976,"va":5405538369,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_wall_state::PublicWallState::max_y
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-25122213","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405537808,"raw_hex":"e85baa80fe","size":5,"soft_fail":false,"target_va":5380417056,"va":5405539264,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_walls::PublicWalls::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-25745674","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405537808,"raw_hex":"e8f62677fe","size":5,"soft_fail":false,"target_va":5379792624,"va":5405538293,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_walls::PublicWalls::roof_entity
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-25745713","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405537808,"raw_hex":"e8cf2677fe","size":5,"soft_fail":false,"target_va":5379793584,"va":5405539292,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::rng::init_rng
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-23632549","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405537808,"raw_hex":"e85b6597fe","size":5,"soft_fail":false,"target_va":5381907008,"va":5405539552,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::event::iterators::impl$0::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/iterators.rs
//     evidence={"module":3887,"record_offset":1200,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":12537,"source_line":14,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::reader::EventReader::read
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/reader.rs
//     evidence={"module":3887,"record_offset":488,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":8034,"source_line":27,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::writer::EventWriter::write
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/writer.rs
//     evidence={"module":3887,"record_offset":8664,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":7657,"source_line":74,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::query::iter::impl$1::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs
//     evidence={"module":3887,"record_offset":1952,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":11673,"source_line":884,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::query::Query::iter
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":3887,"record_offset":1604,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":13704,"source_line":670,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":3887,"record_offset":8992,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_direct] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":3887,"record_offset":8268,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":4980,"source_line":509,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::and_then
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3887,"record_offset":6680,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":4518,"source_line":1447,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::map_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3887,"record_offset":8212,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":5577,"source_line":1173,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3887,"record_offset":1540,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::public_wall_state::PublicWallState::is_premade_shape
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_wall_state.rs
//     evidence={"module":3887,"record_offset":1576,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":24411,"source_line":113,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_direct] fastrand::Rng::i32
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fastrand","version":"1.8.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fastrand-1.8.0/src/lib.rs
//     evidence={"module":3887,"record_offset":8384,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":25309,"source_line":237,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":3887,"record_offset":9364,"parent_record_offset":9340,"ancestor_record_offset":264,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":3887,"record_offset":9388,"parent_record_offset":9364,"ancestor_record_offset":264,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":3887,"record_offset":9340,"parent_record_offset":9312,"ancestor_record_offset":264,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3887,"record_offset":1784,"parent_record_offset":1760,"ancestor_record_offset":264,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3887,"record_offset":9252,"parent_record_offset":9224,"ancestor_record_offset":264,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3887,"record_offset":1832,"parent_record_offset":1808,"ancestor_record_offset":264,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3887,"record_offset":1808,"parent_record_offset":1784,"ancestor_record_offset":264,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3887,"record_offset":9312,"parent_record_offset":9280,"ancestor_record_offset":264,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3887,"record_offset":9280,"parent_record_offset":9252,"ancestor_record_offset":264,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":3887,"record_offset":9188,"parent_record_offset":9160,"ancestor_record_offset":264,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":3887,"record_offset":9160,"parent_record_offset":9132,"ancestor_record_offset":264,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":3887,"record_offset":9104,"parent_record_offset":9076,"ancestor_record_offset":264,"inlinee":4247,"source_line":476,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":3887,"record_offset":9132,"parent_record_offset":9104,"ancestor_record_offset":264,"inlinee":4248,"source_line":500,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":3887,"record_offset":9076,"parent_record_offset":9048,"ancestor_record_offset":264,"inlinee":4245,"source_line":887,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\str.rs
//     evidence={"module":3887,"record_offset":9048,"parent_record_offset":9020,"ancestor_record_offset":264,"inlinee":4243,"source_line":210,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$47::from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":3887,"record_offset":9020,"parent_record_offset":8992,"ancestor_record_offset":264,"inlinee":5924,"source_line":2956,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3887,"record_offset":8828,"parent_record_offset":8784,"ancestor_record_offset":264,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3887,"record_offset":1760,"parent_record_offset":1736,"ancestor_record_offset":264,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3887,"record_offset":828,"parent_record_offset":804,"ancestor_record_offset":264,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3887,"record_offset":2332,"parent_record_offset":2308,"ancestor_record_offset":264,"inlinee":4613,"source_line":2705,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3887,"record_offset":2356,"parent_record_offset":2332,"ancestor_record_offset":264,"inlinee":4352,"source_line":2680,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3887,"record_offset":8784,"parent_record_offset":8748,"ancestor_record_offset":264,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3887,"record_offset":9224,"parent_record_offset":9188,"ancestor_record_offset":264,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3887,"record_offset":804,"parent_record_offset":576,"ancestor_record_offset":264,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::entities
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":3887,"record_offset":6028,"parent_record_offset":1976,"ancestor_record_offset":264,"inlinee":11682,"source_line":466,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::is_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":3887,"record_offset":4124,"parent_record_offset":1976,"ancestor_record_offset":264,"inlinee":11679,"source_line":615,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::table_id
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":3887,"record_offset":4208,"parent_record_offset":1976,"ancestor_record_offset":264,"inlinee":5495,"source_line":460,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::ArchetypeEntity::table_row
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":3887,"record_offset":6224,"parent_record_offset":1976,"ancestor_record_offset":264,"inlinee":17012,"source_line":325,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetypes::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":3887,"record_offset":4040,"parent_record_offset":1976,"ancestor_record_offset":264,"inlinee":11678,"source_line":895,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::impl$25::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":3887,"record_offset":7132,"parent_record_offset":6800,"ancestor_record_offset":264,"inlinee":4846,"source_line":76,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":3887,"record_offset":8720,"parent_record_offset":8688,"ancestor_record_offset":264,"inlinee":4459,"source_line":397,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":3887,"record_offset":8688,"parent_record_offset":8664,"ancestor_record_offset":264,"inlinee":4457,"source_line":434,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::Entities::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/entity/mod.rs
//     evidence={"module":3887,"record_offset":6800,"parent_record_offset":6756,"ancestor_record_offset":264,"inlinee":4840,"source_line":842,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::collections::Events::send_with_caller
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/collections.rs
//     evidence={"module":3887,"record_offset":8748,"parent_record_offset":8688,"ancestor_record_offset":264,"inlinee":7659,"source_line":128,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::event_cursor::EventCursor::read_with_id
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/event_cursor.rs
//     evidence={"module":3887,"record_offset":552,"parent_record_offset":512,"ancestor_record_offset":264,"inlinee":8037,"source_line":88,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::iterators::EventIteratorWithId::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/iterators.rs
//     evidence={"module":3887,"record_offset":576,"parent_record_offset":552,"ancestor_record_offset":264,"inlinee":8039,"source_line":54,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::iterators::impl$3::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/iterators.rs
//     evidence={"module":3887,"record_offset":1224,"parent_record_offset":1200,"ancestor_record_offset":264,"inlinee":12538,"source_line":86,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::StorageSwitch::extract
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3887,"record_offset":5936,"parent_record_offset":5912,"ancestor_record_offset":264,"inlinee":4876,"source_line":2472,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$26::set_archetype
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3887,"record_offset":4332,"parent_record_offset":4300,"ancestor_record_offset":264,"inlinee":4856,"source_line":1160,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$26::set_table
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3887,"record_offset":2424,"parent_record_offset":2392,"ancestor_record_offset":264,"inlinee":4857,"source_line":1175,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$27::fetch
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3887,"record_offset":5912,"parent_record_offset":5888,"ancestor_record_offset":264,"inlinee":4874,"source_line":1229,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$27::fetch::closure$0
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3887,"record_offset":5960,"parent_record_offset":5936,"ancestor_record_offset":264,"inlinee":4878,"source_line":1235,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$63::fetch
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3887,"record_offset":5888,"parent_record_offset":1976,"ancestor_record_offset":264,"inlinee":13690,"source_line":2083,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIter::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs
//     evidence={"module":3887,"record_offset":1652,"parent_record_offset":1628,"ancestor_record_offset":264,"inlinee":13709,"source_line":39,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::init
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs
//     evidence={"module":3887,"record_offset":1676,"parent_record_offset":1652,"ancestor_record_offset":264,"inlinee":9292,"source_line":2359,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs
//     evidence={"module":3887,"record_offset":1976,"parent_record_offset":1952,"ancestor_record_offset":264,"inlinee":11674,"source_line":2458,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$3::set_archetype
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/world_query.rs
//     evidence={"module":3887,"record_offset":4300,"parent_record_offset":1976,"ancestor_record_offset":264,"inlinee":11680,"source_line":182,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$3::set_table
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/world_query.rs
//     evidence={"module":3887,"record_offset":2392,"parent_record_offset":1976,"ancestor_record_offset":264,"inlinee":11676,"source_line":195,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_array::BlobArray::get_sub_slice
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_array.rs
//     evidence={"module":3887,"record_offset":2844,"parent_record_offset":2820,"ancestor_record_offset":264,"inlinee":4872,"source_line":143,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseArray::get
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":3887,"record_offset":2524,"parent_record_offset":2500,"ancestor_record_offset":264,"inlinee":4864,"source_line":58,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":3887,"record_offset":2500,"parent_record_offset":2476,"ancestor_record_offset":264,"inlinee":4862,"source_line":395,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$16::get::closure$0
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":3887,"record_offset":2704,"parent_record_offset":2680,"ancestor_record_offset":264,"inlinee":4866,"source_line":396,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_column
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":3887,"record_offset":2476,"parent_record_offset":2448,"ancestor_record_offset":264,"inlinee":4860,"source_line":475,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_data_slice_for
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":3887,"record_offset":2448,"parent_record_offset":2424,"ancestor_record_offset":264,"inlinee":4859,"source_line":363,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::is_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":3887,"record_offset":2308,"parent_record_offset":1976,"ancestor_record_offset":264,"inlinee":11675,"source_line":615,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::TableRow::as_usize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":3887,"record_offset":5984,"parent_record_offset":5960,"ancestor_record_offset":264,"inlinee":5472,"source_line":127,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":3887,"record_offset":2224,"parent_record_offset":1976,"ancestor_record_offset":264,"inlinee":4854,"source_line":721,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::column::ThinColumn::get_data_slice
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/column.rs
//     evidence={"module":3887,"record_offset":2820,"parent_record_offset":2796,"ancestor_record_offset":264,"inlinee":4870,"source_line":303,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::impl$4::get_data_slice_for::closure$0
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":3887,"record_offset":2796,"parent_record_offset":2772,"ancestor_record_offset":264,"inlinee":4868,"source_line":368,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":3887,"record_offset":6732,"parent_record_offset":6708,"ancestor_record_offset":264,"inlinee":4837,"source_line":1385,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::get_inner
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":3887,"record_offset":6756,"parent_record_offset":6732,"ancestor_record_offset":264,"inlinee":4838,"source_line":1591,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::impl$4::into_iter
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":3887,"record_offset":1628,"parent_record_offset":1604,"ancestor_record_offset":264,"inlinee":25723,"source_line":2574,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::system_param::impl$19::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/system_param.rs
//     evidence={"module":3887,"record_offset":512,"parent_record_offset":488,"ancestor_record_offset":264,"inlinee":10713,"source_line":1234,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ptr::ThinSlicePtr::get
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ptr","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ptr-0.16.0/src/lib.rs
//     evidence={"module":3887,"record_offset":6376,"parent_record_offset":6352,"ancestor_record_offset":264,"inlinee":4880,"source_line":526,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] core::bool::impl$0::then
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\bool.rs
//     evidence={"module":3887,"record_offset":7192,"parent_record_offset":7164,"ancestor_record_offset":264,"inlinee":4781,"source_line":61,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":3887,"record_offset":8320,"parent_record_offset":8292,"ancestor_record_offset":264,"inlinee":4984,"source_line":1943,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$24::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":3887,"record_offset":7100,"parent_record_offset":7076,"ancestor_record_offset":264,"inlinee":4844,"source_line":1742,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":3887,"record_offset":8344,"parent_record_offset":8320,"ancestor_record_offset":264,"inlinee":4986,"source_line":1788,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":3887,"record_offset":9464,"parent_record_offset":9440,"ancestor_record_offset":264,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::and_then_or_clear
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":3887,"record_offset":1276,"parent_record_offset":1252,"ancestor_record_offset":264,"inlinee":5541,"source_line":332,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":3887,"record_offset":1252,"parent_record_offset":1224,"ancestor_record_offset":264,"inlinee":4570,"source_line":83,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":3887,"record_offset":1356,"parent_record_offset":1332,"ancestor_record_offset":264,"inlinee":4573,"source_line":84,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":3887,"record_offset":760,"parent_record_offset":732,"ancestor_record_offset":264,"inlinee":4611,"source_line":653,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":3887,"record_offset":620,"parent_record_offset":576,"ancestor_record_offset":264,"inlinee":8040,"source_line":1859,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$8::wrapping_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":3887,"record_offset":8540,"parent_record_offset":8408,"ancestor_record_offset":264,"inlinee":20286,"source_line":2065,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":3887,"record_offset":8504,"parent_record_offset":8472,"ancestor_record_offset":264,"inlinee":6331,"source_line":1980,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::impl$2::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":3887,"record_offset":7076,"parent_record_offset":6800,"ancestor_record_offset":264,"inlinee":4842,"source_line":200,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":3887,"record_offset":2616,"parent_record_offset":2552,"ancestor_record_offset":264,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":3887,"record_offset":8292,"parent_record_offset":8268,"ancestor_record_offset":264,"inlinee":4982,"source_line":817,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3887,"record_offset":7752,"parent_record_offset":7728,"ancestor_record_offset":264,"inlinee":4518,"source_line":1447,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3887,"record_offset":1300,"parent_record_offset":1276,"ancestor_record_offset":264,"inlinee":4130,"source_line":728,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3887,"record_offset":2640,"parent_record_offset":2616,"ancestor_record_offset":264,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3887,"record_offset":2680,"parent_record_offset":2500,"ancestor_record_offset":264,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3887,"record_offset":1332,"parent_record_offset":1252,"ancestor_record_offset":264,"inlinee":4571,"source_line":1546,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":3887,"record_offset":9440,"parent_record_offset":9188,"ancestor_record_offset":264,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3887,"record_offset":1092,"parent_record_offset":1068,"ancestor_record_offset":264,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":3887,"record_offset":1480,"parent_record_offset":1456,"ancestor_record_offset":264,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":3887,"record_offset":2108,"parent_record_offset":2072,"ancestor_record_offset":264,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":3887,"record_offset":8948,"parent_record_offset":8784,"ancestor_record_offset":264,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":3887,"record_offset":676,"parent_record_offset":576,"ancestor_record_offset":264,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":3887,"record_offset":1044,"parent_record_offset":576,"ancestor_record_offset":264,"inlinee":4460,"source_line":1050,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_noubcheck
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":3887,"record_offset":6900,"parent_record_offset":6872,"ancestor_record_offset":264,"inlinee":4164,"source_line":87,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":3887,"record_offset":2272,"parent_record_offset":2248,"ancestor_record_offset":264,"inlinee":4145,"source_line":226,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":3887,"record_offset":732,"parent_record_offset":704,"ancestor_record_offset":264,"inlinee":8042,"source_line":364,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$7::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":3887,"record_offset":704,"parent_record_offset":676,"ancestor_record_offset":264,"inlinee":8041,"source_line":537,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":3887,"record_offset":1068,"parent_record_offset":1044,"ancestor_record_offset":264,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":3887,"record_offset":1456,"parent_record_offset":1432,"ancestor_record_offset":264,"inlinee":4510,"source_line":99,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":3887,"record_offset":1408,"parent_record_offset":1356,"ancestor_record_offset":264,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$186::next_unchecked
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":3887,"record_offset":1432,"parent_record_offset":1408,"ancestor_record_offset":264,"inlinee":4509,"source_line":440,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] fastrand::Rng::gen_mod_u32
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fastrand","version":"1.8.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fastrand-1.8.0/src/lib.rs
//     evidence={"module":3887,"record_offset":8408,"parent_record_offset":8384,"ancestor_record_offset":264,"inlinee":25182,"source_line":143,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] fastrand::Rng::gen_u32
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"fastrand","version":"1.8.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fastrand-1.8.0/src/lib.rs
//     evidence={"module":3887,"record_offset":8448,"parent_record_offset":8408,"ancestor_record_offset":264,"inlinee":25183,"source_line":122,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] fastrand::Rng::gen_u64
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"fastrand","version":"1.8.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fastrand-1.8.0/src/lib.rs
//     evidence={"module":3887,"record_offset":8472,"parent_record_offset":8448,"ancestor_record_offset":264,"inlinee":7384,"source_line":128,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::contains
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":3887,"record_offset":7164,"parent_record_offset":6756,"ancestor_record_offset":264,"inlinee":4848,"source_line":392,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::contains_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":3887,"record_offset":7240,"parent_record_offset":7216,"ancestor_record_offset":264,"inlinee":4851,"source_line":408,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::get_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":3887,"record_offset":7264,"parent_record_offset":7240,"ancestor_record_offset":264,"inlinee":4852,"source_line":171,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::impl$2::contains::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":3887,"record_offset":7216,"parent_record_offset":7192,"ancestor_record_offset":264,"inlinee":4850,"source_line":395,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"nonmax","version":"0.5.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":3887,"record_offset":2728,"parent_record_offset":2704,"ancestor_record_offset":264,"inlinee":4171,"source_line":153,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] system_critters::sheep::sheep_react_to_being_inside_buildings::react_to_being_inside_buildings::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-critters","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/critters/src/sheep/sheep_react_to_being_inside_buildings.rs
//     evidence={"module":3887,"record_offset":6708,"parent_record_offset":6680,"ancestor_record_offset":264,"inlinee":122533,"source_line":46,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] system_critters::sheep::sheep_react_to_being_inside_buildings::react_to_being_inside_buildings::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-critters","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/critters/src/sheep/sheep_react_to_being_inside_buildings.rs
//     evidence={"module":3887,"record_offset":8236,"parent_record_offset":8212,"ancestor_record_offset":264,"inlinee":122534,"source_line":47,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_critters::sheep::sheep_react_to_being_inside_buildings::react_to_being_inside_buildings::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 46
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1DEA5
// 首条位置: {"module":3887,"inlinee":122533}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] bevy_ecs::system::query::Query::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":3887,"record_offset":6732,"parent_record_offset":6708,"ancestor_record_offset":6708,"inlinee":4837,"source_line":1385,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3887,"record_offset":6980,"parent_record_offset":6956,"ancestor_record_offset":6708,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3887,"record_offset":7028,"parent_record_offset":7004,"ancestor_record_offset":6708,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3887,"record_offset":7004,"parent_record_offset":6980,"ancestor_record_offset":6708,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3887,"record_offset":6956,"parent_record_offset":6932,"ancestor_record_offset":6708,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3887,"record_offset":6932,"parent_record_offset":6832,"ancestor_record_offset":6708,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3887,"record_offset":7368,"parent_record_offset":7344,"ancestor_record_offset":6708,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::impl$25::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":3887,"record_offset":7132,"parent_record_offset":6800,"ancestor_record_offset":6708,"inlinee":4846,"source_line":76,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::Entities::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/entity/mod.rs
//     evidence={"module":3887,"record_offset":6800,"parent_record_offset":6756,"ancestor_record_offset":6708,"inlinee":4840,"source_line":842,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::StorageSwitch::extract
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3887,"record_offset":8080,"parent_record_offset":8056,"ancestor_record_offset":6708,"inlinee":4876,"source_line":2472,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$26::set_archetype
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3887,"record_offset":7604,"parent_record_offset":6756,"ancestor_record_offset":6708,"inlinee":4856,"source_line":1160,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$26::set_table
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3887,"record_offset":7628,"parent_record_offset":7604,"ancestor_record_offset":6708,"inlinee":4857,"source_line":1175,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$27::fetch
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3887,"record_offset":8056,"parent_record_offset":6756,"ancestor_record_offset":6708,"inlinee":4874,"source_line":1229,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$27::fetch::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3887,"record_offset":8104,"parent_record_offset":8080,"ancestor_record_offset":6708,"inlinee":4878,"source_line":1235,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_array::BlobArray::get_sub_slice
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_array.rs
//     evidence={"module":3887,"record_offset":8004,"parent_record_offset":7980,"ancestor_record_offset":6708,"inlinee":4872,"source_line":143,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseArray::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":3887,"record_offset":7728,"parent_record_offset":7704,"ancestor_record_offset":6708,"inlinee":4864,"source_line":58,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":3887,"record_offset":7704,"parent_record_offset":7680,"ancestor_record_offset":6708,"inlinee":4862,"source_line":395,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$16::get::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":3887,"record_offset":7864,"parent_record_offset":7840,"ancestor_record_offset":6708,"inlinee":4866,"source_line":396,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_column
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":3887,"record_offset":7680,"parent_record_offset":7652,"ancestor_record_offset":6708,"inlinee":4860,"source_line":475,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_data_slice_for
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":3887,"record_offset":7652,"parent_record_offset":7628,"ancestor_record_offset":6708,"inlinee":4859,"source_line":363,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":3887,"record_offset":7344,"parent_record_offset":6756,"ancestor_record_offset":6708,"inlinee":4854,"source_line":721,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::column::ThinColumn::get_data_slice
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/column.rs
//     evidence={"module":3887,"record_offset":7980,"parent_record_offset":7956,"ancestor_record_offset":6708,"inlinee":4870,"source_line":303,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::impl$4::get_data_slice_for::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":3887,"record_offset":7956,"parent_record_offset":7932,"ancestor_record_offset":6708,"inlinee":4868,"source_line":368,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::get_inner
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":3887,"record_offset":6756,"parent_record_offset":6732,"ancestor_record_offset":6708,"inlinee":4838,"source_line":1591,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ptr::ThinSlicePtr::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ptr","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ptr-0.16.0/src/lib.rs
//     evidence={"module":3887,"record_offset":8128,"parent_record_offset":8104,"ancestor_record_offset":6708,"inlinee":4880,"source_line":526,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] core::bool::impl$0::then
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\bool.rs
//     evidence={"module":3887,"record_offset":7192,"parent_record_offset":7164,"ancestor_record_offset":6708,"inlinee":4781,"source_line":61,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$24::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":3887,"record_offset":7100,"parent_record_offset":7076,"ancestor_record_offset":6708,"inlinee":4844,"source_line":1742,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::impl$2::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":3887,"record_offset":7076,"parent_record_offset":6800,"ancestor_record_offset":6708,"inlinee":4842,"source_line":200,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":3887,"record_offset":7776,"parent_record_offset":7752,"ancestor_record_offset":6708,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3887,"record_offset":7752,"parent_record_offset":7728,"ancestor_record_offset":6708,"inlinee":4518,"source_line":1447,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3887,"record_offset":7800,"parent_record_offset":7776,"ancestor_record_offset":6708,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3887,"record_offset":7840,"parent_record_offset":7704,"ancestor_record_offset":6708,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3887,"record_offset":7296,"parent_record_offset":7264,"ancestor_record_offset":6708,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":3887,"record_offset":6832,"parent_record_offset":6800,"ancestor_record_offset":6708,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_noubcheck
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":3887,"record_offset":6900,"parent_record_offset":6872,"ancestor_record_offset":6708,"inlinee":4164,"source_line":87,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":3887,"record_offset":6872,"parent_record_offset":6832,"ancestor_record_offset":6708,"inlinee":4145,"source_line":226,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::contains
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":3887,"record_offset":7164,"parent_record_offset":6756,"ancestor_record_offset":6708,"inlinee":4848,"source_line":392,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::contains_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":3887,"record_offset":7240,"parent_record_offset":7216,"ancestor_record_offset":6708,"inlinee":4851,"source_line":408,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::get_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":3887,"record_offset":7264,"parent_record_offset":7240,"ancestor_record_offset":6708,"inlinee":4852,"source_line":171,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::impl$2::contains::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":3887,"record_offset":7216,"parent_record_offset":7192,"ancestor_record_offset":6708,"inlinee":4850,"source_line":395,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nonmax","version":"0.5.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":3887,"record_offset":7888,"parent_record_offset":7864,"ancestor_record_offset":6708,"inlinee":4171,"source_line":153,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_critters::sheep::sheep_react_to_being_inside_buildings::react_to_being_inside_buildings::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 47
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1DEA6
// 首条位置: {"module":3887,"inlinee":122534}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_critters::sheep::sheep_react_to_being_inside_buildings::impl$0::register_required_components
// 记录形式: compiled_procedure; instances=1
// 源码行: 101
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3887,"record_offset":14480,"rva":36832384,"va":5405541504}
// 编译实例 RVAs: 0x2320480
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140051340
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: sheep_react_to_being_inside_buildings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-36501032","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405541504,"raw_hex":"e8d809d3fd","size":5,"soft_fail":false,"target_va":5369041728,"va":5405542755,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405f0b50
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: sheep_react_to_being_inside_buildings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-30604461","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405541504,"raw_hex":"e853032dfe","size":5,"soft_fail":false,"target_va":5374937936,"va":5405542392,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: sheep_react_to_being_inside_buildings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-30461323","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405541504,"raw_hex":"e875322ffe","size":5,"soft_fail":false,"target_va":5375081504,"va":5405542822,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141dbd790
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: sheep_react_to_being_inside_buildings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5648429","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405541504,"raw_hex":"e8d3cfa9ff","size":5,"soft_fail":false,"target_va":5399893904,"va":5405542328,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142320480
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: sheep_react_to_being_inside_buildings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1183","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405541504,"raw_hex":"e861fbffff","size":5,"soft_fail":false,"target_va":5405541504,"va":5405542682,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: sheep_react_to_being_inside_buildings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t6304790","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405541504,"raw_hex":"e816346000","size":5,"soft_fail":false,"target_va":5411847168,"va":5405542373,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-24767444","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405541504,"raw_hex":"e82c1486fe","size":5,"soft_fail":false,"target_va":5380775408,"va":5405542847,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::component::ComponentInfo::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-22941485","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405541504,"raw_hex":"e8d3f0a1fe","size":5,"soft_fail":false,"target_va":5382600816,"va":5405542296,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::component::QueuedRegistration::register
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-22941700","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405541504,"raw_hex":"e8fcefa1fe","size":5,"soft_fail":false,"target_va":5382601280,"va":5405542975,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::component::enforce_no_required_components_recursion
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-22935397","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405541504,"raw_hex":"e89b08a2fe","size":5,"soft_fail":false,"target_va":5382606192,"va":5405541584,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] hashbrown::raw::RawTable$LT$T$C$A$GT$::reserve_rehash
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t5969085","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405541504,"raw_hex":"e8bd145b00","size":5,"soft_fail":false,"target_va":5411512256,"va":5405543166,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::pop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3887,"record_offset":20724,"parent_record_offset":14480,"ancestor_record_offset":14480,"inlinee":6530,"source_line":2503,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3887,"record_offset":20528,"parent_record_offset":14480,"ancestor_record_offset":14480,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::component::ComponentsRegistrator::register_component
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/component.rs
//     evidence={"module":3887,"record_offset":14672,"parent_record_offset":14480,"ancestor_record_offset":14480,"inlinee":5611,"source_line":1715,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":3887,"record_offset":20240,"parent_record_offset":20216,"ancestor_record_offset":14480,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":3887,"record_offset":20216,"parent_record_offset":20124,"ancestor_record_offset":14480,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3887,"record_offset":19660,"parent_record_offset":19636,"ancestor_record_offset":14480,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3887,"record_offset":20152,"parent_record_offset":20124,"ancestor_record_offset":14480,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3887,"record_offset":20124,"parent_record_offset":20100,"ancestor_record_offset":14480,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3887,"record_offset":19708,"parent_record_offset":19684,"ancestor_record_offset":14480,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3887,"record_offset":19684,"parent_record_offset":19660,"ancestor_record_offset":14480,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3887,"record_offset":20100,"parent_record_offset":20076,"ancestor_record_offset":14480,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3887,"record_offset":19636,"parent_record_offset":19612,"ancestor_record_offset":14480,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3887,"record_offset":19612,"parent_record_offset":19588,"ancestor_record_offset":14480,"inlinee":5037,"source_line":1593,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3887,"record_offset":14716,"parent_record_offset":14672,"ancestor_record_offset":14480,"inlinee":4597,"source_line":422,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3887,"record_offset":19588,"parent_record_offset":17584,"ancestor_record_offset":14480,"inlinee":5310,"source_line":3273,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::ComponentDescriptor::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/component.rs
//     evidence={"module":3887,"record_offset":17652,"parent_record_offset":17584,"ancestor_record_offset":14480,"inlinee":5647,"source_line":1101,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::ComponentIds::next_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/component.rs
//     evidence={"module":3887,"record_offset":17552,"parent_record_offset":14744,"ancestor_record_offset":14480,"inlinee":5644,"source_line":1334,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::Components::register_component_inner
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/component.rs
//     evidence={"module":3887,"record_offset":17680,"parent_record_offset":17584,"ancestor_record_offset":14480,"inlinee":5648,"source_line":1972,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::ComponentsRegistrator::register_component_checked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/component.rs
//     evidence={"module":3887,"record_offset":14744,"parent_record_offset":14672,"ancestor_record_offset":14480,"inlinee":5612,"source_line":1721,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::ComponentsRegistrator::register_component_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/component.rs
//     evidence={"module":3887,"record_offset":17584,"parent_record_offset":14744,"ancestor_record_offset":14480,"inlinee":5645,"source_line":1755,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::impl$17::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/component.rs
//     evidence={"module":3887,"record_offset":17816,"parent_record_offset":17584,"ancestor_record_offset":14480,"inlinee":13108,"source_line":1614,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::impl$90::default
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/component.rs
//     evidence={"module":3887,"record_offset":19560,"parent_record_offset":17584,"ancestor_record_offset":14480,"inlinee":5662,"source_line":2812,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] bevy_platform::collections::hash_map::HashMap::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_platform","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_platform-0.16.0/src/collections/hash_map.rs
//     evidence={"module":3887,"record_offset":14812,"parent_record_offset":14744,"ancestor_record_offset":14480,"inlinee":5614,"source_line":933,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] bevy_platform::collections::hash_map::HashMap::remove
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_platform","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_platform-0.16.0/src/collections/hash_map.rs
//     evidence={"module":3887,"record_offset":15852,"parent_record_offset":14744,"ancestor_record_offset":14480,"inlinee":5624,"source_line":1141,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$7::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\any.rs
//     evidence={"module":3887,"record_offset":15592,"parent_record_offset":15568,"ancestor_record_offset":14480,"inlinee":5068,"source_line":719,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$25::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":3887,"record_offset":15640,"parent_record_offset":15616,"ancestor_record_offset":14480,"inlinee":5070,"source_line":1740,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_cmpeq_epi8
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":3887,"record_offset":17112,"parent_record_offset":17084,"ancestor_record_offset":14480,"inlinee":5636,"source_line":842,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":3887,"record_offset":19000,"parent_record_offset":18976,"ancestor_record_offset":14480,"inlinee":4832,"source_line":1214,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":3887,"record_offset":15040,"parent_record_offset":15016,"ancestor_record_offset":14480,"inlinee":4484,"source_line":1227,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":3887,"record_offset":15124,"parent_record_offset":15100,"ancestor_record_offset":14480,"inlinee":4486,"source_line":1433,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":3887,"record_offset":15064,"parent_record_offset":15040,"ancestor_record_offset":14480,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":3887,"record_offset":20184,"parent_record_offset":20152,"ancestor_record_offset":14480,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::wrapping_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":3887,"record_offset":16972,"parent_record_offset":16924,"ancestor_record_offset":14480,"inlinee":5005,"source_line":2021,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$7::leading_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":3887,"record_offset":17348,"parent_record_offset":17320,"ancestor_record_offset":14480,"inlinee":5639,"source_line":135,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$7::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":3887,"record_offset":17376,"parent_record_offset":17320,"ancestor_record_offset":14480,"inlinee":5640,"source_line":161,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":3887,"record_offset":15276,"parent_record_offset":15248,"ancestor_record_offset":14480,"inlinee":4503,"source_line":449,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":3887,"record_offset":15248,"parent_record_offset":15220,"ancestor_record_offset":14480,"inlinee":4493,"source_line":601,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_none
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3887,"record_offset":18376,"parent_record_offset":18012,"ancestor_record_offset":14480,"inlinee":5657,"source_line":651,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3887,"record_offset":18400,"parent_record_offset":18376,"ancestor_record_offset":14480,"inlinee":4512,"source_line":607,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::offset_from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":3887,"record_offset":16884,"parent_record_offset":16860,"ancestor_record_offset":14480,"inlinee":5634,"source_line":682,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":3887,"record_offset":17784,"parent_record_offset":17680,"ancestor_record_offset":14480,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3887,"record_offset":17472,"parent_record_offset":17448,"ancestor_record_offset":14480,"inlinee":4834,"source_line":1354,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3887,"record_offset":15508,"parent_record_offset":15484,"ancestor_record_offset":14480,"inlinee":4723,"source_line":1129,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3887,"record_offset":19488,"parent_record_offset":19464,"ancestor_record_offset":14480,"inlinee":5670,"source_line":1505,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":3887,"record_offset":17496,"parent_record_offset":17472,"ancestor_record_offset":14480,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":3887,"record_offset":19512,"parent_record_offset":19488,"ancestor_record_offset":14480,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":3887,"record_offset":17728,"parent_record_offset":17680,"ancestor_record_offset":14480,"inlinee":5649,"source_line":619,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":3887,"record_offset":17752,"parent_record_offset":17728,"ancestor_record_offset":14480,"inlinee":5650,"source_line":232,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":3887,"record_offset":15784,"parent_record_offset":15760,"ancestor_record_offset":14480,"inlinee":5623,"source_line":623,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":3887,"record_offset":15808,"parent_record_offset":15784,"ancestor_record_offset":14480,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] core::tuple::impl$10::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\tuple.rs
//     evidence={"module":3887,"record_offset":15616,"parent_record_offset":15592,"ancestor_record_offset":14480,"inlinee":4662,"source_line":30,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] equivalent::impl$0::equivalent
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"equivalent","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/equivalent-1.0.1/src/lib.rs
//     evidence={"module":3887,"record_offset":15568,"parent_record_offset":15544,"ancestor_record_offset":14480,"inlinee":4660,"source_line":88,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::leading_zeros
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/control/bitmask.rs
//     evidence={"module":3887,"record_offset":17320,"parent_record_offset":16924,"ancestor_record_offset":14480,"inlinee":5637,"source_line":86,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/control/bitmask.rs
//     evidence={"module":3887,"record_offset":15184,"parent_record_offset":15156,"ancestor_record_offset":14480,"inlinee":4490,"source_line":49,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/control/bitmask.rs
//     evidence={"module":3887,"record_offset":15220,"parent_record_offset":15184,"ancestor_record_offset":14480,"inlinee":4491,"source_line":74,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/control/bitmask.rs
//     evidence={"module":3887,"record_offset":15156,"parent_record_offset":14936,"ancestor_record_offset":14480,"inlinee":4488,"source_line":112,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/control/group/sse2.rs
//     evidence={"module":3887,"record_offset":15016,"parent_record_offset":14936,"ancestor_record_offset":14480,"inlinee":4482,"source_line":51,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load_aligned
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/control/group/sse2.rs
//     evidence={"module":3887,"record_offset":18976,"parent_record_offset":18916,"ancestor_record_offset":14480,"inlinee":4831,"source_line":59,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/control/group/sse2.rs
//     evidence={"module":3887,"record_offset":15320,"parent_record_offset":14936,"ancestor_record_offset":14480,"inlinee":4504,"source_line":94,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty_or_deleted
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/control/group/sse2.rs
//     evidence={"module":3887,"record_offset":18464,"parent_record_offset":18436,"ancestor_record_offset":14480,"inlinee":4818,"source_line":101,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/control/group/sse2.rs
//     evidence={"module":3887,"record_offset":15100,"parent_record_offset":14936,"ancestor_record_offset":14480,"inlinee":4485,"source_line":76,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::special_is_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/control/tag.rs
//     evidence={"module":3887,"record_offset":19252,"parent_record_offset":19220,"ancestor_record_offset":14480,"inlinee":5667,"source_line":28,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::find_or_find_insert_slot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/map.rs
//     evidence={"module":3887,"record_offset":17900,"parent_record_offset":17856,"ancestor_record_offset":14480,"inlinee":5653,"source_line":1804,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/map.rs
//     evidence={"module":3887,"record_offset":14836,"parent_record_offset":14812,"ancestor_record_offset":14480,"inlinee":4719,"source_line":1304,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/map.rs
//     evidence={"module":3887,"record_offset":14860,"parent_record_offset":14836,"ancestor_record_offset":14480,"inlinee":4475,"source_line":1347,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::insert
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/map.rs
//     evidence={"module":3887,"record_offset":17856,"parent_record_offset":17816,"ancestor_record_offset":14480,"inlinee":5652,"source_line":1790,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::remove
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/map.rs
//     evidence={"module":3887,"record_offset":15884,"parent_record_offset":15852,"ancestor_record_offset":14480,"inlinee":5625,"source_line":1954,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::remove_entry
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/map.rs
//     evidence={"module":3887,"record_offset":15916,"parent_record_offset":15884,"ancestor_record_offset":14480,"inlinee":5626,"source_line":1993,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/map.rs
//     evidence={"module":3887,"record_offset":15544,"parent_record_offset":15436,"ancestor_record_offset":14480,"inlinee":4497,"source_line":224,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":3887,"record_offset":15484,"parent_record_offset":15460,"ancestor_record_offset":14480,"inlinee":5616,"source_line":261,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::read
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":3887,"record_offset":17448,"parent_record_offset":16756,"ancestor_record_offset":14480,"inlinee":5642,"source_line":488,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::to_base_index
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":3887,"record_offset":16836,"parent_record_offset":16812,"ancestor_record_offset":14480,"inlinee":5631,"source_line":334,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::write
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":3887,"record_offset":19464,"parent_record_offset":19188,"ancestor_record_offset":14480,"inlinee":5669,"source_line":510,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":3887,"record_offset":15404,"parent_record_offset":14936,"ancestor_record_offset":14480,"inlinee":4506,"source_line":81,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":3887,"record_offset":15460,"parent_record_offset":15436,"ancestor_record_offset":14480,"inlinee":5615,"source_line":734,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket_index
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":3887,"record_offset":16812,"parent_record_offset":16784,"ancestor_record_offset":14480,"inlinee":5630,"source_line":705,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::erase_no_drop
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":3887,"record_offset":16784,"parent_record_offset":16756,"ancestor_record_offset":14480,"inlinee":5629,"source_line":766,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":3887,"record_offset":14912,"parent_record_offset":14888,"ancestor_record_offset":14480,"inlinee":4478,"source_line":1156,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find_or_find_insert_slot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":3887,"record_offset":17936,"parent_record_offset":17900,"ancestor_record_offset":14480,"inlinee":5654,"source_line":1109,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":3887,"record_offset":14888,"parent_record_offset":14860,"ancestor_record_offset":14480,"inlinee":4477,"source_line":1179,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::insert_in_slot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":3887,"record_offset":19188,"parent_record_offset":17856,"ancestor_record_offset":14480,"inlinee":5665,"source_line":1145,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::remove
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":3887,"record_offset":16756,"parent_record_offset":15940,"ancestor_record_offset":14480,"inlinee":5628,"source_line":785,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::remove_entry
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":3887,"record_offset":15940,"parent_record_offset":15916,"ancestor_record_offset":14480,"inlinee":5627,"source_line":797,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::reserve
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":3887,"record_offset":17976,"parent_record_offset":17936,"ancestor_record_offset":14480,"inlinee":5655,"source_line":896,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::erase
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":3887,"record_offset":16924,"parent_record_offset":16784,"ancestor_record_offset":14480,"inlinee":5635,"source_line":3062,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":3887,"record_offset":14936,"parent_record_offset":14912,"ancestor_record_offset":14480,"inlinee":4480,"source_line":1862,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_insert_slot_in_group
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":3887,"record_offset":18436,"parent_record_offset":18012,"ancestor_record_offset":14480,"inlinee":5658,"source_line":1600,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_or_find_insert_slot_inner
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":3887,"record_offset":18012,"parent_record_offset":17936,"ancestor_record_offset":14480,"inlinee":5656,"source_line":1648,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::fix_insert_slot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":3887,"record_offset":18916,"parent_record_offset":18012,"ancestor_record_offset":14480,"inlinee":5663,"source_line":1563,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::is_bucket_full
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":3887,"record_offset":18948,"parent_record_offset":18916,"ancestor_record_offset":14480,"inlinee":5664,"source_line":2498,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":3887,"record_offset":14988,"parent_record_offset":14936,"ancestor_record_offset":14480,"inlinee":4515,"source_line":2301,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::record_item_insert_at
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":3887,"record_offset":19220,"parent_record_offset":19188,"ancestor_record_offset":14480,"inlinee":5666,"source_line":2311,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::set_ctrl
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":3887,"record_offset":17408,"parent_record_offset":16924,"ancestor_record_offset":14480,"inlinee":5641,"source_line":2419,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::set_ctrl_hash
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":3887,"record_offset":19288,"parent_record_offset":19220,"ancestor_record_offset":14480,"inlinee":5668,"source_line":2353,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$17::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":3887,"record_offset":19852,"parent_record_offset":19828,"ancestor_record_offset":14480,"inlinee":4995,"source_line":3323,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":3887,"record_offset":15436,"parent_record_offset":14936,"ancestor_record_offset":14480,"inlinee":4495,"source_line":1166,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find_or_find_insert_slot::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":3887,"record_offset":18648,"parent_record_offset":18012,"ancestor_record_offset":14480,"inlinee":5660,"source_line":1127,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::offset_from
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":3887,"record_offset":16860,"parent_record_offset":16836,"ancestor_record_offset":14480,"inlinee":5633,"source_line":18,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::borrow
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison.rs
//     evidence={"module":3887,"record_offset":15736,"parent_record_offset":15712,"ancestor_record_offset":14480,"inlinee":5620,"source_line":116,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison.rs
//     evidence={"module":3887,"record_offset":15760,"parent_record_offset":15736,"ancestor_record_offset":14480,"inlinee":5621,"source_line":144,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::rwlock::RwLock::get_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison\rwlock.rs
//     evidence={"module":3887,"record_offset":15712,"parent_record_offset":14744,"ancestor_record_offset":14480,"inlinee":5618,"source_line":631,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_critters::sheep::sheep_react_to_being_inside_buildings::sheep_reaction_emoji
// 记录形式: compiled_procedure; instances=1
// 源码行: 114
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3887,"record_offset":9532,"rva":36831088,"va":5405540208}
// 编译实例 RVAs: 0x231ff70
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] bevy_ecs::system::commands::EntityCommands::despawn
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-23116876","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405540208,"raw_hex":"e8b4439ffe","size":5,"soft_fail":false,"target_va":5382424432,"va":5405541303,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::window::WindowSize::as_vec2
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-27092117","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405540208,"raw_hex":"e86b9b62fe","size":5,"soft_fail":false,"target_va":5378448400,"va":5405540512,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::utils::world_space_to_screen_space
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-25524544","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405540208,"raw_hex":"e8c0867afe","size":5,"soft_fail":false,"target_va":5380016000,"va":5405540539,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::easing_functions::EasingFunction::apply
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-23635021","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405540208,"raw_hex":"e8b35b97fe","size":5,"soft_fail":false,"target_va":5381906160,"va":5405541176,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$58::deref_mut
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":3887,"record_offset":10128,"parent_record_offset":9532,"ancestor_record_offset":9532,"inlinee":9177,"source_line":434,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::query::iter::impl$1::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs
//     evidence={"module":3887,"record_offset":10184,"parent_record_offset":9532,"ancestor_record_offset":9532,"inlinee":11673,"source_line":884,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::commands::Commands::entity
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs
//     evidence={"module":3887,"record_offset":14248,"parent_record_offset":9532,"ancestor_record_offset":9532,"inlinee":13699,"source_line":407,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::query::Query::iter_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":3887,"record_offset":9720,"parent_record_offset":9532,"ancestor_record_offset":9532,"inlinee":17020,"source_line":701,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":3887,"record_offset":14104,"parent_record_offset":9532,"ancestor_record_offset":9532,"inlinee":6356,"source_line":1409,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":3887,"record_offset":14136,"parent_record_offset":9532,"ancestor_record_offset":9532,"inlinee":4698,"source_line":1198,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3887,"record_offset":9960,"parent_record_offset":9936,"ancestor_record_offset":9532,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3887,"record_offset":10008,"parent_record_offset":9984,"ancestor_record_offset":9532,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3887,"record_offset":9984,"parent_record_offset":9960,"ancestor_record_offset":9532,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3887,"record_offset":9936,"parent_record_offset":9912,"ancestor_record_offset":9532,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3887,"record_offset":9912,"parent_record_offset":9880,"ancestor_record_offset":9532,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3887,"record_offset":10488,"parent_record_offset":10464,"ancestor_record_offset":9532,"inlinee":4613,"source_line":2705,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3887,"record_offset":10512,"parent_record_offset":10488,"ancestor_record_offset":9532,"inlinee":4352,"source_line":2680,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3887,"record_offset":9880,"parent_record_offset":9852,"ancestor_record_offset":9532,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::entities
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":3887,"record_offset":13276,"parent_record_offset":10208,"ancestor_record_offset":9532,"inlinee":11682,"source_line":466,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::is_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":3887,"record_offset":10708,"parent_record_offset":10208,"ancestor_record_offset":9532,"inlinee":11679,"source_line":615,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::table_id
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":3887,"record_offset":12176,"parent_record_offset":10208,"ancestor_record_offset":9532,"inlinee":5495,"source_line":460,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::ArchetypeEntity::table_row
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":3887,"record_offset":13556,"parent_record_offset":10208,"ancestor_record_offset":9532,"inlinee":17012,"source_line":325,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetypes::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":3887,"record_offset":10624,"parent_record_offset":10208,"ancestor_record_offset":9532,"inlinee":11678,"source_line":895,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$57::set_changed
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":3887,"record_offset":10152,"parent_record_offset":10128,"ancestor_record_offset":9532,"inlinee":9179,"source_line":397,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::StorageSwitch::extract
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3887,"record_offset":12084,"parent_record_offset":12060,"ancestor_record_offset":9532,"inlinee":4876,"source_line":2472,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::set_archetype
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3887,"record_offset":12292,"parent_record_offset":12268,"ancestor_record_offset":9532,"inlinee":13655,"source_line":1524,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::set_table
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3887,"record_offset":10820,"parent_record_offset":10792,"ancestor_record_offset":9532,"inlinee":13656,"source_line":1539,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$37::fetch
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3887,"record_offset":12060,"parent_record_offset":12036,"ancestor_record_offset":9532,"inlinee":13686,"source_line":1596,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$37::fetch::closure$0
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3887,"record_offset":12108,"parent_record_offset":12084,"ancestor_record_offset":9532,"inlinee":9533,"source_line":1602,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$63::fetch
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3887,"record_offset":12036,"parent_record_offset":10208,"ancestor_record_offset":9532,"inlinee":13690,"source_line":2083,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIter::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs
//     evidence={"module":3887,"record_offset":9828,"parent_record_offset":9744,"ancestor_record_offset":9532,"inlinee":13709,"source_line":39,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::init
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs
//     evidence={"module":3887,"record_offset":9852,"parent_record_offset":9828,"ancestor_record_offset":9532,"inlinee":9292,"source_line":2359,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs
//     evidence={"module":3887,"record_offset":10208,"parent_record_offset":10184,"ancestor_record_offset":9532,"inlinee":11674,"source_line":2458,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$3::set_archetype
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/world_query.rs
//     evidence={"module":3887,"record_offset":12268,"parent_record_offset":10208,"ancestor_record_offset":9532,"inlinee":11680,"source_line":182,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$3::set_table
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/world_query.rs
//     evidence={"module":3887,"record_offset":10792,"parent_record_offset":10208,"ancestor_record_offset":9532,"inlinee":11676,"source_line":195,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_array::BlobArray::get_sub_slice
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_array.rs
//     evidence={"module":3887,"record_offset":11200,"parent_record_offset":11176,"ancestor_record_offset":9532,"inlinee":4872,"source_line":143,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseArray::get
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":3887,"record_offset":10908,"parent_record_offset":10880,"ancestor_record_offset":9532,"inlinee":4864,"source_line":58,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":3887,"record_offset":10880,"parent_record_offset":10852,"ancestor_record_offset":9532,"inlinee":4862,"source_line":395,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$16::get::closure$0
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":3887,"record_offset":11044,"parent_record_offset":11020,"ancestor_record_offset":9532,"inlinee":4866,"source_line":396,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::entities
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":3887,"record_offset":11756,"parent_record_offset":10208,"ancestor_record_offset":9532,"inlinee":11681,"source_line":211,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_column
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":3887,"record_offset":10852,"parent_record_offset":10820,"ancestor_record_offset":9532,"inlinee":4860,"source_line":475,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::is_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":3887,"record_offset":10464,"parent_record_offset":10208,"ancestor_record_offset":9532,"inlinee":11675,"source_line":615,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::TableRow::as_usize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":3887,"record_offset":12132,"parent_record_offset":12108,"ancestor_record_offset":9532,"inlinee":5472,"source_line":127,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":3887,"record_offset":10380,"parent_record_offset":10208,"ancestor_record_offset":9532,"inlinee":4854,"source_line":721,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::column::ThinColumn::get_changed_ticks_slice
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/column.rs
//     evidence={"module":3887,"record_offset":11232,"parent_record_offset":10820,"ancestor_record_offset":9532,"inlinee":13683,"source_line":319,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::column::ThinColumn::get_data_slice
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/column.rs
//     evidence={"module":3887,"record_offset":11176,"parent_record_offset":10820,"ancestor_record_offset":9532,"inlinee":4870,"source_line":303,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::thin_array_ptr::ThinArrayPtr::as_slice
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/thin_array_ptr.rs
//     evidence={"module":3887,"record_offset":11256,"parent_record_offset":11232,"ancestor_record_offset":9532,"inlinee":13684,"source_line":300,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::copy_unsafe
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":3887,"record_offset":9796,"parent_record_offset":9772,"ancestor_record_offset":9532,"inlinee":13707,"source_line":636,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::reborrow
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":3887,"record_offset":9744,"parent_record_offset":9720,"ancestor_record_offset":9532,"inlinee":17021,"source_line":601,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::reborrow_unsafe
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":3887,"record_offset":9772,"parent_record_offset":9744,"ancestor_record_offset":9532,"inlinee":13706,"source_line":618,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ptr::ThinSlicePtr::get
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ptr","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ptr-0.16.0/src/lib.rs
//     evidence={"module":3887,"record_offset":13684,"parent_record_offset":13656,"ancestor_record_offset":9532,"inlinee":4880,"source_line":526,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":3887,"record_offset":14160,"parent_record_offset":14136,"ancestor_record_offset":9532,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":3887,"record_offset":10956,"parent_record_offset":10932,"ancestor_record_offset":9532,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3887,"record_offset":10932,"parent_record_offset":10908,"ancestor_record_offset":9532,"inlinee":4518,"source_line":1447,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3887,"record_offset":10980,"parent_record_offset":10956,"ancestor_record_offset":9532,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3887,"record_offset":11020,"parent_record_offset":10880,"ancestor_record_offset":9532,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3887,"record_offset":10076,"parent_record_offset":10052,"ancestor_record_offset":9532,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":3887,"record_offset":10344,"parent_record_offset":10308,"ancestor_record_offset":9532,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":3887,"record_offset":10404,"parent_record_offset":10380,"ancestor_record_offset":9532,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":3887,"record_offset":11952,"parent_record_offset":10208,"ancestor_record_offset":9532,"inlinee":5263,"source_line":656,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_noubcheck
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":3887,"record_offset":11124,"parent_record_offset":11100,"ancestor_record_offset":9532,"inlinee":4164,"source_line":87,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":3887,"record_offset":10428,"parent_record_offset":10404,"ancestor_record_offset":9532,"inlinee":4145,"source_line":226,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":3887,"record_offset":11100,"parent_record_offset":11068,"ancestor_record_offset":9532,"inlinee":4162,"source_line":242,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":3887,"record_offset":10052,"parent_record_offset":9880,"ancestor_record_offset":9532,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":3887,"record_offset":10308,"parent_record_offset":10208,"ancestor_record_offset":9532,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/03887.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"nonmax","version":"0.5.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":3887,"record_offset":11068,"parent_record_offset":11044,"ancestor_record_offset":9532,"inlinee":4171,"source_line":153,"archive":"_pdb/B/modules/03887.jsonl.gz"}
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
// 完整结构及原始类型记录: sheep_react_to_being_inside_buildings.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] system_critters::sheep::sheep_react_to_being_inside_buildings::SheepReactionEmoji
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 48
// [PDB 泛型关系，非字段关系] {"type": "ref_mut$<system_critters::sheep::sheep_react_to_being_inside_buildings::SheepReactionEmoji>", "constructor": "ref_mut$", "arguments": ["system_critters::sheep::sheep_react_to_being_inside_buildings::SheepReactionEmoji"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<ref_mut$<system_critters::sheep::sheep_react_to_being_inside_buildings::SheepReactionEmoji>,ref_mut$<country_core::components::Ui2dExtent>,bevy_ecs::entity::Entity>", "constructor": "tuple$", "arguments": ["ref_mut$<system_critters::sheep::sheep_react_to_being_inside_buildings::SheepReactionEmoji>", "ref_mut$<country_core::components::Ui2dExtent>", "bevy_ecs::entity::Entity"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::query::Query<tuple$<ref_mut$<system_critters::sheep::sheep_react_to_being_inside_buildings::SheepReactionEmoji>,ref_mut$<country_core::components::Ui2dExtent>,bevy_ecs::entity::Entity>,tuple$<> >", "constructor": "bevy_ecs::system::query::Query", "arguments": ["tuple$<ref_mut$<system_critters::sheep::sheep_react_to_being_inside_buildings::SheepReactionEmoji>,ref_mut$<country_core::components::Ui2dExtent>,bevy_ecs::entity::Entity>", "tuple$<>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<void (*)(bevy_ecs::system::query::Query<tuple$<enum2$<core::option::Option<ref_mut$<utils::components::transform::Transform> > >,ref_mut$<country_core::resources::critters::sheep::Sheep> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::critters::sheep::Sheep>,ref$<utils::components::transform::Transform>,ref_mut$<country_core::resources::critters::sheep::SheepAudio> >,tuple$<> >,bevy_ecs::event::writer::EventWriter<system_critters::sheep::sheep_pet_animation::PlaySheepPetAnimation>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::system::system_param::Local<system_critters::sheep::ui_pet_sheep::IntersectionDepth>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>,bevy_ecs::event::reader::EventReader<country_core::resources::window::CursorMovedScreenSpace>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::sheep_audio::PlaySheepReactSound>),void (*)(bevy_ecs::system::query::Query<ref_mut$<country_core::resources::critters::sheep::Sheep>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::event::reader::EventReader<system_critters::sheep::sheep_pet_animation::PlaySheepPetAnimation>),void (*)(bevy_ecs::event::reader::EventReader<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::critters::sheep::Sheep>,ref$<utils::components::transform::Transform>,ref$<country_core::resources::critters::sheep::SheepAudio> >,tuple$<> >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::sheep_audio::PlaySheepReactSound>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<system_critters::sheep::sheep_react_to_being_inside_buildings::SheepReactionEmoji>,ref_mut$<country_core::components::Ui2dExtent>,bevy_ecs::entity::Entity>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::system::commands::Commands)>", "constructor": "tuple$", "arguments": ["void (*)(bevy_ecs::system::query::Query<tuple$<enum2$<core::option::Option<ref_mut$<utils::components::transform::Transform> > >,ref_mut$<country_core::resources::critters::sheep::Sheep> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)", "void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::critters::sheep::Sheep>,ref$<utils::components::transform::Transform>,ref_mut$<country_core::resources::critters::sheep::SheepAudio> >,tuple$<> >,bevy_ecs::event::writer::EventWriter<system_critters::sheep::sheep_pet_animation::PlaySheepPetAnimation>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::system::system_param::Local<system_critters::sheep::ui_pet_sheep::IntersectionDepth>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>,bevy_ecs::event::reader::EventReader<country_core::resources::window::CursorMovedScreenSpace>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::sheep_audio::PlaySheepReactSound>)", "void (*)(bevy_ecs::system::query::Query<ref_mut$<country_core::resources::critters::sheep::Sheep>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::event::reader::EventReader<system_critters::sheep::sheep_pet_animation::PlaySheepPetAnimation>)", "void (*)(bevy_ecs::event::reader::EventReader<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::critters::sheep::Sheep>,ref$<utils::components::transform::Transform>,ref$<country_core::resources::critters::sheep::SheepAudio> >,tuple$<> >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::sheep_audio::PlaySheepReactSound>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)", "void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<system_critters::sheep::sheep_react_to_being_inside_buildings::SheepReactionEmoji>,ref_mut$<country_core::components::Ui2dExtent>,bevy_ecs::entity::Entity>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::system::commands::Commands)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<utils::components::transform::Transform>,ref_mut$<country_core::resources::critters::sheep::Sheep>,ref_mut$<country_core::resources::critters::CritterGoal> >,tuple$<> >,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::gui::EguiContextResource> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::DebugGui>,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<bevy_input::button_input::ButtonInput<enum2$<bevy_input::keyboard::KeyCode> > >,bevy_ecs::event::reader::EventReader<country_core::resources::critters::sheep::CmdUnfreezeSheep>),tuple$<void (*)(bevy_ecs::system::query::Query<tuple$<enum2$<core::option::Option<ref_mut$<utils::components::transform::Transform> > >,ref_mut$<country_core::resources::critters::sheep::Sheep> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::critters::sheep::Sheep>,ref$<utils::components::transform::Transform>,ref_mut$<country_core::resources::critters::sheep::SheepAudio> >,tuple$<> >,bevy_ecs::event::writer::EventWriter<system_critters::sheep::sheep_pet_animation::PlaySheepPetAnimation>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::system::system_param::Local<system_critters::sheep::ui_pet_sheep::IntersectionDepth>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>,bevy_ecs::event::reader::EventReader<country_core::resources::window::CursorMovedScreenSpace>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::sheep_audio::PlaySheepReactSound>),void (*)(bevy_ecs::system::query::Query<ref_mut$<country_core::resources::critters::sheep::Sheep>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::event::reader::EventReader<system_critters::sheep::sheep_pet_animation::PlaySheepPetAnimation>),void (*)(bevy_ecs::event::reader::EventReader<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::critters::sheep::Sheep>,ref$<utils::components::transform::Transform>,ref$<country_core::resources::critters::sheep::SheepAudio> >,tuple$<> >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::sheep_audio::PlaySheepReactSound>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<system_critters::sheep::sheep_react_to_being_inside_buildings::SheepReactionEmoji>,ref_mut$<country_core::components::Ui2dExtent>,bevy_ecs::entity::Entity>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::system::commands::Commands)> >", "constructor": "tuple$", "arguments": ["void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<utils::components::transform::Transform>,ref_mut$<country_core::resources::critters::sheep::Sheep>,ref_mut$<country_core::resources::critters::CritterGoal> >,tuple$<> >,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::gui::EguiContextResource> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::DebugGui>,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<bevy_input::button_input::ButtonInput<enum2$<bevy_input::keyboard::KeyCode> > >,bevy_ecs::event::reader::EventReader<country_core::resources::critters::sheep::CmdUnfreezeSheep>)", "tuple$<void (*)(bevy_ecs::system::query::Query<tuple$<enum2$<core::option::Option<ref_mut$<utils::components::transform::Transform> > >,ref_mut$<country_core::resources::critters::sheep::Sheep> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::critters::sheep::Sheep>,ref$<utils::components::transform::Transform>,ref_mut$<country_core::resources::critters::sheep::SheepAudio> >,tuple$<> >,bevy_ecs::event::writer::EventWriter<system_critters::sheep::sheep_pet_animation::PlaySheepPetAnimation>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::system::system_param::Local<system_critters::sheep::ui_pet_sheep::IntersectionDepth>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>,bevy_ecs::event::reader::EventReader<country_core::resources::window::CursorMovedScreenSpace>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::sheep_audio::PlaySheepReactSound>),void (*)(bevy_ecs::system::query::Query<ref_mut$<country_core::resources::critters::sheep::Sheep>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::event::reader::EventReader<system_critters::sheep::sheep_pet_animation::PlaySheepPetAnimation>),void (*)(bevy_ecs::event::reader::EventReader<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::critters::sheep::Sheep>,ref$<utils::components::transform::Transform>,ref$<country_core::resources::critters::sheep::SheepAudio> >,tuple$<> >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::sheep_audio::PlaySheepReactSound>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<system_critters::sheep::sheep_react_to_being_inside_buildings::SheepReactionEmoji>,ref_mut$<country_core::components::Ui2dExtent>,bevy_ecs::entity::Entity>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::system::commands::Commands)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<system_critters::sheep::sheep_react_to_being_inside_buildings::SheepReactionEmoji>,ref_mut$<country_core::components::Ui2dExtent>,bevy_ecs::entity::Entity>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::system::commands::Commands)>", "constructor": "tuple$", "arguments": ["bevy_ecs::system::function_system::IsFunctionSystem", "void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<system_critters::sheep::sheep_react_to_being_inside_buildings::SheepReactionEmoji>,ref_mut$<country_core::components::Ui2dExtent>,bevy_ecs::entity::Entity>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::system::commands::Commands)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<system_critters::sheep::sheep_react_to_being_inside_buildings::SheepReactionEmoji>,ref_mut$<country_core::components::Ui2dExtent>,bevy_ecs::entity::Entity>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::system::commands::Commands)> >", "constructor": "tuple$", "arguments": ["bevy_ecs::schedule::config::Infallible", "tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<system_critters::sheep::sheep_react_to_being_inside_buildings::SheepReactionEmoji>,ref_mut$<country_core::components::Ui2dExtent>,bevy_ecs::entity::Entity>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::system::commands::Commands)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<bevy_ecs::schedule::config::ScheduleConfigTupleMarker,tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::query::Query<tuple$<enum2$<core::option::Option<ref_mut$<utils::components::transform::Transform> > >,ref_mut$<country_core::resources::critters::sheep::Sheep> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)> >,tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::critters::sheep::Sheep>,ref$<utils::components::transform::Transform>,ref_mut$<country_core::resources::critters::sheep::SheepAudio> >,tuple$<> >,bevy_ecs::event::writer::EventWriter<system_critters::sheep::sheep_pet_animation::PlaySheepPetAnimation>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::system::system_param::Local<system_critters::sheep::ui_pet_sheep::IntersectionDepth>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>,bevy_ecs::event::reader::EventReader<country_core::resources::window::CursorMovedScreenSpace>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::sheep_audio::PlaySheepReactSound>)> >,tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::query::Query<ref_mut$<country_core::resources::critters::sheep::Sheep>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::event::reader::EventReader<system_critters::sheep::sheep_pet_animation::PlaySheepPetAnimation>)> >,tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::event::reader::EventReader<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::critters::sheep::Sheep>,ref$<utils::components::transform::Transform>,ref$<country_core::resources::critters::sheep::SheepAudio> >,tuple$<> >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::sheep_audio::PlaySheepReactSound>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)> >,tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<system_critters::sheep::sheep_react_to_being_inside_buildings::SheepReactionEmoji>,ref_mut$<country_core::components::Ui2dExtent>,bevy_ecs::entity::Entity>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::system::commands::Commands)> > >", "constructor": "tuple$", "arguments": ["bevy_ecs::schedule::config::ScheduleConfigTupleMarker", "tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::query::Query<tuple$<enum2$<core::option::Option<ref_mut$<utils::components::transform::Transform> > >,ref_mut$<country_core::resources::critters::sheep::Sheep> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)> >", "tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::critters::sheep::Sheep>,ref$<utils::components::transform::Transform>,ref_mut$<country_core::resources::critters::sheep::SheepAudio> >,tuple$<> >,bevy_ecs::event::writer::EventWriter<system_critters::sheep::sheep_pet_animation::PlaySheepPetAnimation>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::system::system_param::Local<system_critters::sheep::ui_pet_sheep::IntersectionDepth>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>,bevy_ecs::event::reader::EventReader<country_core::resources::window::CursorMovedScreenSpace>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::sheep_audio::PlaySheepReactSound>)> >", "tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::query::Query<ref_mut$<country_core::resources::critters::sheep::Sheep>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::event::reader::EventReader<system_critters::sheep::sheep_pet_animation::PlaySheepPetAnimation>)> >", "tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::event::reader::EventReader<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::critters::sheep::Sheep>,ref$<utils::components::transform::Transform>,ref$<country_core::resources::critters::sheep::SheepAudio> >,tuple$<> >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::sheep_audio::PlaySheepReactSound>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)> >", "tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<system_critters::sheep::sheep_react_to_being_inside_buildings::SheepReactionEmoji>,ref_mut$<country_core::components::Ui2dExtent>,bevy_ecs::entity::Entity>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::system::commands::Commands)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<bevy_ecs::schedule::config::ScheduleConfigTupleMarker,tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<utils::components::transform::Transform>,ref_mut$<country_core::resources::critters::sheep::Sheep>,ref_mut$<country_core::resources::critters::CritterGoal> >,tuple$<> >,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::gui::EguiContextResource> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::DebugGui>,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<bevy_input::button_input::ButtonInput<enum2$<bevy_input::keyboard::KeyCode> > >,bevy_ecs::event::reader::EventReader<country_core::resources::critters::sheep::CmdUnfreezeSheep>)> >,tuple$<bevy_ecs::schedule::config::ScheduleConfigTupleMarker,tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::query::Query<tuple$<enum2$<core::option::Option<ref_mut$<utils::components::transform::Transform> > >,ref_mut$<country_core::resources::critters::sheep::Sheep> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)> >,tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::critters::sheep::Sheep>,ref$<utils::components::transform::Transform>,ref_mut$<country_core::resources::critters::sheep::SheepAudio> >,tuple$<> >,bevy_ecs::event::writer::EventWriter<system_critters::sheep::sheep_pet_animation::PlaySheepPetAnimation>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::system::system_param::Local<system_critters::sheep::ui_pet_sheep::IntersectionDepth>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>,bevy_ecs::event::reader::EventReader<country_core::resources::window::CursorMovedScreenSpace>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::sheep_audio::PlaySheepReactSound>)> >,tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::query::Query<ref_mut$<country_core::resources::critters::sheep::Sheep>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::event::reader::EventReader<system_critters::sheep::sheep_pet_animation::PlaySheepPetAnimation>)> >,tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::event::reader::EventReader<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::critters::sheep::Sheep>,ref$<utils::components::transform::Transform>,ref$<country_core::resources::critters::sheep::SheepAudio> >,tuple$<> >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::sheep_audio::PlaySheepReactSound>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)> >,tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<system_critters::sheep::sheep_react_to_being_inside_buildings::SheepReactionEmoji>,ref_mut$<country_core::components::Ui2dExtent>,bevy_ecs::entity::Entity>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::system::commands::Commands)> > > >", "constructor": "tuple$", "arguments": ["bevy_ecs::schedule::config::ScheduleConfigTupleMarker", "tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<utils::components::transform::Transform>,ref_mut$<country_core::resources::critters::sheep::Sheep>,ref_mut$<country_core::resources::critters::CritterGoal> >,tuple$<> >,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::gui::EguiContextResource> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::DebugGui>,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<bevy_input::button_input::ButtonInput<enum2$<bevy_input::keyboard::KeyCode> > >,bevy_ecs::event::reader::EventReader<country_core::resources::critters::sheep::CmdUnfreezeSheep>)> >", "tuple$<bevy_ecs::schedule::config::ScheduleConfigTupleMarker,tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::query::Query<tuple$<enum2$<core::option::Option<ref_mut$<utils::components::transform::Transform> > >,ref_mut$<country_core::resources::critters::sheep::Sheep> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)> >,tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::critters::sheep::Sheep>,ref$<utils::components::transform::Transform>,ref_mut$<country_core::resources::critters::sheep::SheepAudio> >,tuple$<> >,bevy_ecs::event::writer::EventWriter<system_critters::sheep::sheep_pet_animation::PlaySheepPetAnimation>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::system::system_param::Local<system_critters::sheep::ui_pet_sheep::IntersectionDepth>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>,bevy_ecs::event::reader::EventReader<country_core::resources::window::CursorMovedScreenSpace>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::sheep_audio::PlaySheepReactSound>)> >,tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::query::Query<ref_mut$<country_core::resources::critters::sheep::Sheep>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::event::reader::EventReader<system_critters::sheep::sheep_pet_animation::PlaySheepPetAnimation>)> >,tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::event::reader::EventReader<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::critters::sheep::Sheep>,ref$<utils::components::transform::Transform>,ref$<country_core::resources::critters::sheep::SheepAudio> >,tuple$<> >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::sheep_audio::PlaySheepReactSound>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)> >,tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<system_critters::sheep::sheep_react_to_being_inside_buildings::SheepReactionEmoji>,ref_mut$<country_core::components::Ui2dExtent>,bevy_ecs::entity::Entity>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::system::commands::Commands)> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::config::IntoScheduleConfigs::chain_ignore_deferred<tuple$<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<utils::components::transform::Transform>,ref_mut$<country_core::resources::critters::sheep::Sheep>,ref_mut$<country_core::resources::critters::CritterGoal> >,tuple$<> >,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::gui::EguiContextResource> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::DebugGui>,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<bevy_input::button_input::ButtonInput<enum2$<bevy_input::keyboard::KeyCode> > >,bevy_ecs::event::reader::EventReader<country_core::resources::critters::sheep::CmdUnfreezeSheep>),tuple$<void (*)(bevy_ecs::system::query::Query<tuple$<enum2$<core::option::Option<ref_mut$<utils::components::transform::Transform> > >,ref_mut$<country_core::resources::critters::sheep::Sheep> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::critters::sheep::Sheep>,ref$<utils::components::transform::Transform>,ref_mut$<country_core::resources::critters::sheep::SheepAudio> >,tuple$<> >,bevy_ecs::event::writer::EventWriter<system_critters::sheep::sheep_pet_animation::PlaySheepPetAnimation>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::system::system_param::Local<system_critters::sheep::ui_pet_sheep::IntersectionDepth>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>,bevy_ecs::event::reader::EventReader<country_core::resources::window::CursorMovedScreenSpace>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::sheep_audio::PlaySheepReactSound>),void (*)(bevy_ecs::system::query::Query<ref_mut$<country_core::resources::critters::sheep::Sheep>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::event::reader::EventReader<system_critters::sheep::sheep_pet_animation::PlaySheepPetAnimation>),void (*)(bevy_ecs::event::reader::EventReader<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::critters::sheep::Sheep>,ref$<utils::components::transform::Transform>,ref$<country_core::resources::critters::sheep::SheepAudio> >,tuple$<> >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::sheep_audio::PlaySheepReactSound>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<system_critters::sheep::sheep_react_to_being_inside_buildings::SheepReactionEmoji>,ref_mut$<country_core::components::Ui2dExtent>,bevy_ecs::entity::Entity>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::system::commands::Commands)> >,alloc::boxed::Box<dyn$<bevy_ecs::system::system::System<assoc$<In,tuple$<> >,assoc$<Out,enum2$<core::result::Result<tuple$<>,bevy_ecs::error::bevy_error::BevyError> > > > >,alloc::alloc::Global>,tuple$<bevy_ecs::schedule::config::ScheduleConfigTupleMarker,tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<utils::components::transform::Transform>,ref_mut$<country_core::resources::critters::sheep::Sheep>,ref_mut$<country_core::resources::critters::CritterGoal> >,tuple$<> >,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::gui::EguiContextResource> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::DebugGui>,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<bevy_input::button_input::ButtonInput<enum2$<bevy_input::keyboard::KeyCode> > >,bevy_ecs::event::reader::EventReader<country_core::resources::critters::sheep::CmdUnfreezeSheep>)> >,tuple$<bevy_ecs::schedule::config::ScheduleConfigTupleMarker,tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::query::Query<tuple$<enum2$<core::option::Option<ref_mut$<utils::components::transform::Transform> > >,ref_mut$<country_core::resources::critters::sheep::Sheep> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)> >,tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::critters::sheep::Sheep>,ref$<utils::components::transform::Transform>,ref_mut$<country_core::resources::critters::sheep::SheepAudio> >,tuple$<> >,bevy_ecs::event::writer::EventWriter<system_critters::sheep::sheep_pet_animation::PlaySheepPetAnimation>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::system::system_param::Local<system_critters::sheep::ui_pet_sheep::IntersectionDepth>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>,bevy_ecs::event::reader::EventReader<country_core::resources::window::CursorMovedScreenSpace>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::sheep_audio::PlaySheepReactSound>)> >,tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::query::Query<ref_mut$<country_core::resources::critters::sheep::Sheep>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::event::reader::EventReader<system_critters::sheep::sheep_pet_animation::PlaySheepPetAnimation>)> >,tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::event::reader::EventReader<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::critters::sheep::Sheep>,ref$<utils::components::transform::Transform>,ref$<country_core::resources::critters::sheep::SheepAudio> >,tuple$<> >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::sheep_audio::PlaySheepReactSound>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)> >,tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<system_critters::sheep::sheep_react_to_being_inside_buildings::SheepReactionEmoji>,ref_mut$<country_core::components::Ui2dExtent>,bevy_ecs::entity::Entity>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::system::commands::Commands)> > > > >", "constructor": "bevy_ecs::schedule::config::IntoScheduleConfigs::chain_ignore_deferred", "arguments": ["tuple$<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<utils::components::transform::Transform>,ref_mut$<country_core::resources::critters::sheep::Sheep>,ref_mut$<country_core::resources::critters::CritterGoal> >,tuple$<> >,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::gui::EguiContextResource> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::DebugGui>,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<bevy_input::button_input::ButtonInput<enum2$<bevy_input::keyboard::KeyCode> > >,bevy_ecs::event::reader::EventReader<country_core::resources::critters::sheep::CmdUnfreezeSheep>),tuple$<void (*)(bevy_ecs::system::query::Query<tuple$<enum2$<core::option::Option<ref_mut$<utils::components::transform::Transform> > >,ref_mut$<country_core::resources::critters::sheep::Sheep> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::critters::sheep::Sheep>,ref$<utils::components::transform::Transform>,ref_mut$<country_core::resources::critters::sheep::SheepAudio> >,tuple$<> >,bevy_ecs::event::writer::EventWriter<system_critters::sheep::sheep_pet_animation::PlaySheepPetAnimation>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::system::system_param::Local<system_critters::sheep::ui_pet_sheep::IntersectionDepth>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>,bevy_ecs::event::reader::EventReader<country_core::resources::window::CursorMovedScreenSpace>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::sheep_audio::PlaySheepReactSound>),void (*)(bevy_ecs::system::query::Query<ref_mut$<country_core::resources::critters::sheep::Sheep>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::event::reader::EventReader<system_critters::sheep::sheep_pet_animation::PlaySheepPetAnimation>),void (*)(bevy_ecs::event::reader::EventReader<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::critters::sheep::Sheep>,ref$<utils::components::transform::Transform>,ref$<country_core::resources::critters::sheep::SheepAudio> >,tuple$<> >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::sheep_audio::PlaySheepReactSound>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<system_critters::sheep::sheep_react_to_being_inside_buildings::SheepReactionEmoji>,ref_mut$<country_core::components::Ui2dExtent>,bevy_ecs::entity::Entity>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::system::commands::Commands)> >", "alloc::boxed::Box<dyn$<bevy_ecs::system::system::System<assoc$<In,tuple$<> >,assoc$<Out,enum2$<core::result::Result<tuple$<>,bevy_ecs::error::bevy_error::BevyError> > > > >,alloc::alloc::Global>", "tuple$<bevy_ecs::schedule::config::ScheduleConfigTupleMarker,tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<utils::components::transform::Transform>,ref_mut$<country_core::resources::critters::sheep::Sheep>,ref_mut$<country_core::resources::critters::CritterGoal> >,tuple$<> >,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::gui::EguiContextResource> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::DebugGui>,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<bevy_input::button_input::ButtonInput<enum2$<bevy_input::keyboard::KeyCode> > >,bevy_ecs::event::reader::EventReader<country_core::resources::critters::sheep::CmdUnfreezeSheep>)> >,tuple$<bevy_ecs::schedule::config::ScheduleConfigTupleMarker,tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::query::Query<tuple$<enum2$<core::option::Option<ref_mut$<utils::components::transform::Transform> > >,ref_mut$<country_core::resources::critters::sheep::Sheep> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)> >,tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::critters::sheep::Sheep>,ref$<utils::components::transform::Transform>,ref_mut$<country_core::resources::critters::sheep::SheepAudio> >,tuple$<> >,bevy_ecs::event::writer::EventWriter<system_critters::sheep::sheep_pet_animation::PlaySheepPetAnimation>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::system::system_param::Local<system_critters::sheep::ui_pet_sheep::IntersectionDepth>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>,bevy_ecs::event::reader::EventReader<country_core::resources::window::CursorMovedScreenSpace>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::sheep_audio::PlaySheepReactSound>)> >,tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::query::Query<ref_mut$<country_core::resources::critters::sheep::Sheep>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::event::reader::EventReader<system_critters::sheep::sheep_pet_animation::PlaySheepPetAnimation>)> >,tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::event::reader::EventReader<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::critters::sheep::Sheep>,ref$<utils::components::transform::Transform>,ref$<country_core::resources::critters::sheep::SheepAudio> >,tuple$<> >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::sheep_audio::PlaySheepReactSound>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)> >,tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<system_critters::sheep::sheep_react_to_being_inside_buildings::SheepReactionEmoji>,ref_mut$<country_core::components::Ui2dExtent>,bevy_ecs::entity::Entity>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::system::commands::Commands)> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::query::state::QueryState::new_with_access<tuple$<ref_mut$<system_critters::sheep::sheep_react_to_being_inside_buildings::SheepReactionEmoji>,ref_mut$<country_core::components::Ui2dExtent>,bevy_ecs::entity::Entity>,tuple$<> >", "constructor": "bevy_ecs::query::state::QueryState::new_with_access", "arguments": ["tuple$<ref_mut$<system_critters::sheep::sheep_react_to_being_inside_buildings::SheepReactionEmoji>,ref_mut$<country_core::components::Ui2dExtent>,bevy_ecs::entity::Entity>", "tuple$<>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::query::state::QueryState::update_archetype_component_access<tuple$<ref_mut$<system_critters::sheep::sheep_react_to_being_inside_buildings::SheepReactionEmoji>,ref_mut$<country_core::components::Ui2dExtent>,bevy_ecs::entity::Entity>,tuple$<> >", "constructor": "bevy_ecs::query::state::QueryState::update_archetype_component_access", "arguments": ["tuple$<ref_mut$<system_critters::sheep::sheep_react_to_being_inside_buildings::SheepReactionEmoji>,ref_mut$<country_core::components::Ui2dExtent>,bevy_ecs::entity::Entity>", "tuple$<>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system_param::init_query_param<tuple$<ref_mut$<system_critters::sheep::sheep_react_to_being_inside_buildings::SheepReactionEmoji>,ref_mut$<country_core::components::Ui2dExtent>,bevy_ecs::entity::Entity>,tuple$<> >", "constructor": "bevy_ecs::system::system_param::init_query_param", "arguments": ["tuple$<ref_mut$<system_critters::sheep::sheep_react_to_being_inside_buildings::SheepReactionEmoji>,ref_mut$<country_core::components::Ui2dExtent>,bevy_ecs::entity::Entity>", "tuple$<>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<bevy_ecs::system::query::Query<tuple$<ref_mut$<system_critters::sheep::sheep_react_to_being_inside_buildings::SheepReactionEmoji>,ref_mut$<country_core::components::Ui2dExtent>,bevy_ecs::entity::Entity>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::system::commands::Commands>", "constructor": "tuple$", "arguments": ["bevy_ecs::system::query::Query<tuple$<ref_mut$<system_critters::sheep::sheep_react_to_being_inside_buildings::SheepReactionEmoji>,ref_mut$<country_core::components::Ui2dExtent>,bevy_ecs::entity::Entity>,tuple$<> >", "bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>", "bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>", "bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>", "bevy_ecs::system::commands::Commands"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::FunctionSystemState<tuple$<bevy_ecs::system::query::Query<tuple$<ref_mut$<system_critters::sheep::sheep_react_to_being_inside_buildings::SheepReactionEmoji>,ref_mut$<country_core::components::Ui2dExtent>,bevy_ecs::entity::Entity>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::system::commands::Commands> >", "constructor": "bevy_ecs::system::function_system::FunctionSystemState", "arguments": ["tuple$<bevy_ecs::system::query::Query<tuple$<ref_mut$<system_critters::sheep::sheep_react_to_being_inside_buildings::SheepReactionEmoji>,ref_mut$<country_core::components::Ui2dExtent>,bevy_ecs::entity::Entity>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::system::commands::Commands>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<bevy_ecs::system::function_system::FunctionSystemState<tuple$<bevy_ecs::system::query::Query<tuple$<ref_mut$<system_critters::sheep::sheep_react_to_being_inside_buildings::SheepReactionEmoji>,ref_mut$<country_core::components::Ui2dExtent>,bevy_ecs::entity::Entity>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::system::commands::Commands> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["bevy_ecs::system::function_system::FunctionSystemState<tuple$<bevy_ecs::system::query::Query<tuple$<ref_mut$<system_critters::sheep::sheep_react_to_being_inside_buildings::SheepReactionEmoji>,ref_mut$<country_core::components::Ui2dExtent>,bevy_ecs::entity::Entity>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::system::commands::Commands> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<system_critters::sheep::sheep_react_to_being_inside_buildings::SheepReactionEmoji>,ref_mut$<country_core::components::Ui2dExtent>,bevy_ecs::entity::Entity>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::system::commands::Commands),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<system_critters::sheep::sheep_react_to_being_inside_buildings::SheepReactionEmoji>,ref_mut$<country_core::components::Ui2dExtent>,bevy_ecs::entity::Entity>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::system::commands::Commands)>", "constructor": "bevy_ecs::system::function_system::FunctionSystem", "arguments": ["void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<system_critters::sheep::sheep_react_to_being_inside_buildings::SheepReactionEmoji>,ref_mut$<country_core::components::Ui2dExtent>,bevy_ecs::entity::Entity>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::system::commands::Commands)", "void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<system_critters::sheep::sheep_react_to_being_inside_buildings::SheepReactionEmoji>,ref_mut$<country_core::components::Ui2dExtent>,bevy_ecs::entity::Entity>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::system::commands::Commands)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<system_critters::sheep::sheep_react_to_being_inside_buildings::SheepReactionEmoji>,ref_mut$<country_core::components::Ui2dExtent>,bevy_ecs::entity::Entity>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::system::commands::Commands),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<system_critters::sheep::sheep_react_to_being_inside_buildings::SheepReactionEmoji>,ref_mut$<country_core::components::Ui2dExtent>,bevy_ecs::entity::Entity>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::system::commands::Commands)> >", "constructor": "bevy_ecs::system::schedule_system::InfallibleSystemWrapper", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<system_critters::sheep::sheep_react_to_being_inside_buildings::SheepReactionEmoji>,ref_mut$<country_core::components::Ui2dExtent>,bevy_ecs::entity::Entity>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::system::commands::Commands),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<system_critters::sheep::sheep_react_to_being_inside_buildings::SheepReactionEmoji>,ref_mut$<country_core::components::Ui2dExtent>,bevy_ecs::entity::Entity>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::system::commands::Commands)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<system_critters::sheep::sheep_react_to_being_inside_buildings::SheepReactionEmoji>,ref_mut$<country_core::components::Ui2dExtent>,bevy_ecs::entity::Entity>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::system::commands::Commands),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<system_critters::sheep::sheep_react_to_being_inside_buildings::SheepReactionEmoji>,ref_mut$<country_core::components::Ui2dExtent>,bevy_ecs::entity::Entity>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::system::commands::Commands)> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<system_critters::sheep::sheep_react_to_being_inside_buildings::SheepReactionEmoji>,ref_mut$<country_core::components::Ui2dExtent>,bevy_ecs::entity::Entity>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::system::commands::Commands),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<system_critters::sheep::sheep_react_to_being_inside_buildings::SheepReactionEmoji>,ref_mut$<country_core::components::Ui2dExtent>,bevy_ecs::entity::Entity>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::system::commands::Commands)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$7::as_dyn_eq<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<system_critters::sheep::sheep_react_to_being_inside_buildings::SheepReactionEmoji>,ref_mut$<country_core::components::Ui2dExtent>,bevy_ecs::entity::Entity>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::system::commands::Commands),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<system_critters::sheep::sheep_react_to_being_inside_buildings::SheepReactionEmoji>,ref_mut$<country_core::components::Ui2dExtent>,bevy_ecs::entity::Entity>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::system::commands::Commands)> >", "constructor": "bevy_ecs::schedule::set::impl$7::as_dyn_eq", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<system_critters::sheep::sheep_react_to_being_inside_buildings::SheepReactionEmoji>,ref_mut$<country_core::components::Ui2dExtent>,bevy_ecs::entity::Entity>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::system::commands::Commands),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<system_critters::sheep::sheep_react_to_being_inside_buildings::SheepReactionEmoji>,ref_mut$<country_core::components::Ui2dExtent>,bevy_ecs::entity::Entity>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::system::commands::Commands)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::update_archetype_component_access<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<system_critters::sheep::sheep_react_to_being_inside_buildings::SheepReactionEmoji>,ref_mut$<country_core::components::Ui2dExtent>,bevy_ecs::entity::Entity>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::system::commands::Commands),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<system_critters::sheep::sheep_react_to_being_inside_buildings::SheepReactionEmoji>,ref_mut$<country_core::components::Ui2dExtent>,bevy_ecs::entity::Entity>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::system::commands::Commands)>", "constructor": "bevy_ecs::system::function_system::impl$7::update_archetype_component_access", "arguments": ["void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<system_critters::sheep::sheep_react_to_being_inside_buildings::SheepReactionEmoji>,ref_mut$<country_core::components::Ui2dExtent>,bevy_ecs::entity::Entity>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::system::commands::Commands)", "void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<system_critters::sheep::sheep_react_to_being_inside_buildings::SheepReactionEmoji>,ref_mut$<country_core::components::Ui2dExtent>,bevy_ecs::entity::Entity>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::system::commands::Commands)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<system_critters::sheep::sheep_react_to_being_inside_buildings::SheepReactionEmoji>,ref_mut$<country_core::components::Ui2dExtent>,bevy_ecs::entity::Entity>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::system::commands::Commands),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<system_critters::sheep::sheep_react_to_being_inside_buildings::SheepReactionEmoji>,ref_mut$<country_core::components::Ui2dExtent>,bevy_ecs::entity::Entity>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::system::commands::Commands)> >", "constructor": "bevy_ecs::schedule::set::SystemTypeSet", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<system_critters::sheep::sheep_react_to_being_inside_buildings::SheepReactionEmoji>,ref_mut$<country_core::components::Ui2dExtent>,bevy_ecs::entity::Entity>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::system::commands::Commands),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<system_critters::sheep::sheep_react_to_being_inside_buildings::SheepReactionEmoji>,ref_mut$<country_core::components::Ui2dExtent>,bevy_ecs::entity::Entity>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::system::commands::Commands)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::label::impl$0::as_any<bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<system_critters::sheep::sheep_react_to_being_inside_buildings::SheepReactionEmoji>,ref_mut$<country_core::components::Ui2dExtent>,bevy_ecs::entity::Entity>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::system::commands::Commands),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<system_critters::sheep::sheep_react_to_being_inside_buildings::SheepReactionEmoji>,ref_mut$<country_core::components::Ui2dExtent>,bevy_ecs::entity::Entity>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::system::commands::Commands)> > >", "constructor": "bevy_ecs::label::impl$0::as_any", "arguments": ["bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<system_critters::sheep::sheep_react_to_being_inside_buildings::SheepReactionEmoji>,ref_mut$<country_core::components::Ui2dExtent>,bevy_ecs::entity::Entity>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::system::commands::Commands),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<system_critters::sheep::sheep_react_to_being_inside_buildings::SheepReactionEmoji>,ref_mut$<country_core::components::Ui2dExtent>,bevy_ecs::entity::Entity>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::system::commands::Commands)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::default_system_sets<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<system_critters::sheep::sheep_react_to_being_inside_buildings::SheepReactionEmoji>,ref_mut$<country_core::components::Ui2dExtent>,bevy_ecs::entity::Entity>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::system::commands::Commands),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<system_critters::sheep::sheep_react_to_being_inside_buildings::SheepReactionEmoji>,ref_mut$<country_core::components::Ui2dExtent>,bevy_ecs::entity::Entity>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::system::commands::Commands)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::default_system_sets", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<system_critters::sheep::sheep_react_to_being_inside_buildings::SheepReactionEmoji>,ref_mut$<country_core::components::Ui2dExtent>,bevy_ecs::entity::Entity>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::system::commands::Commands),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<system_critters::sheep::sheep_react_to_being_inside_buildings::SheepReactionEmoji>,ref_mut$<country_core::components::Ui2dExtent>,bevy_ecs::entity::Entity>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::system::commands::Commands)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::validate_param<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<system_critters::sheep::sheep_react_to_being_inside_buildings::SheepReactionEmoji>,ref_mut$<country_core::components::Ui2dExtent>,bevy_ecs::entity::Entity>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::system::commands::Commands),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<system_critters::sheep::sheep_react_to_being_inside_buildings::SheepReactionEmoji>,ref_mut$<country_core::components::Ui2dExtent>,bevy_ecs::entity::Entity>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::system::commands::Commands)> > >", "constructor": "bevy_ecs::system::system::System::validate_param", "arguments": ["bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<system_critters::sheep::sheep_react_to_being_inside_buildings::SheepReactionEmoji>,ref_mut$<country_core::components::Ui2dExtent>,bevy_ecs::entity::Entity>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::system::commands::Commands),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<system_critters::sheep::sheep_react_to_being_inside_buildings::SheepReactionEmoji>,ref_mut$<country_core::components::Ui2dExtent>,bevy_ecs::entity::Entity>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::system::commands::Commands)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<system_critters::sheep::sheep_react_to_being_inside_buildings::SheepReactionEmoji>,ref_mut$<country_core::components::Ui2dExtent>,bevy_ecs::entity::Entity>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::system::commands::Commands),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<system_critters::sheep::sheep_react_to_being_inside_buildings::SheepReactionEmoji>,ref_mut$<country_core::components::Ui2dExtent>,bevy_ecs::entity::Entity>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::system::commands::Commands)> > >", "constructor": "bevy_ecs::system::system::System::run_without_applying_deferred", "arguments": ["bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<system_critters::sheep::sheep_react_to_being_inside_buildings::SheepReactionEmoji>,ref_mut$<country_core::components::Ui2dExtent>,bevy_ecs::entity::Entity>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::system::commands::Commands),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<system_critters::sheep::sheep_react_to_being_inside_buildings::SheepReactionEmoji>,ref_mut$<country_core::components::Ui2dExtent>,bevy_ecs::entity::Entity>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::system::commands::Commands)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::run<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<system_critters::sheep::sheep_react_to_being_inside_buildings::SheepReactionEmoji>,ref_mut$<country_core::components::Ui2dExtent>,bevy_ecs::entity::Entity>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::system::commands::Commands),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<system_critters::sheep::sheep_react_to_being_inside_buildings::SheepReactionEmoji>,ref_mut$<country_core::components::Ui2dExtent>,bevy_ecs::entity::Entity>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::system::commands::Commands)> > >", "constructor": "bevy_ecs::system::system::System::run", "arguments": ["bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<system_critters::sheep::sheep_react_to_being_inside_buildings::SheepReactionEmoji>,ref_mut$<country_core::components::Ui2dExtent>,bevy_ecs::entity::Entity>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::system::commands::Commands),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<system_critters::sheep::sheep_react_to_being_inside_buildings::SheepReactionEmoji>,ref_mut$<country_core::components::Ui2dExtent>,bevy_ecs::entity::Entity>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::system::commands::Commands)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$7::system_type<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<system_critters::sheep::sheep_react_to_being_inside_buildings::SheepReactionEmoji>,ref_mut$<country_core::components::Ui2dExtent>,bevy_ecs::entity::Entity>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::system::commands::Commands),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<system_critters::sheep::sheep_react_to_being_inside_buildings::SheepReactionEmoji>,ref_mut$<country_core::components::Ui2dExtent>,bevy_ecs::entity::Entity>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::system::commands::Commands)> >", "constructor": "bevy_ecs::schedule::set::impl$7::system_type", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<system_critters::sheep::sheep_react_to_being_inside_buildings::SheepReactionEmoji>,ref_mut$<country_core::components::Ui2dExtent>,bevy_ecs::entity::Entity>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::system::commands::Commands),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<system_critters::sheep::sheep_react_to_being_inside_buildings::SheepReactionEmoji>,ref_mut$<country_core::components::Ui2dExtent>,bevy_ecs::entity::Entity>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::system::commands::Commands)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$7::dyn_hash<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<system_critters::sheep::sheep_react_to_being_inside_buildings::SheepReactionEmoji>,ref_mut$<country_core::components::Ui2dExtent>,bevy_ecs::entity::Entity>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::system::commands::Commands),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<system_critters::sheep::sheep_react_to_being_inside_buildings::SheepReactionEmoji>,ref_mut$<country_core::components::Ui2dExtent>,bevy_ecs::entity::Entity>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::system::commands::Commands)> >", "constructor": "bevy_ecs::schedule::set::impl$7::dyn_hash", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<system_critters::sheep::sheep_react_to_being_inside_buildings::SheepReactionEmoji>,ref_mut$<country_core::components::Ui2dExtent>,bevy_ecs::entity::Entity>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::system::commands::Commands),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<system_critters::sheep::sheep_react_to_being_inside_buildings::SheepReactionEmoji>,ref_mut$<country_core::components::Ui2dExtent>,bevy_ecs::entity::Entity>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::system::commands::Commands)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$7::dyn_clone<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<system_critters::sheep::sheep_react_to_being_inside_buildings::SheepReactionEmoji>,ref_mut$<country_core::components::Ui2dExtent>,bevy_ecs::entity::Entity>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::system::commands::Commands),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<system_critters::sheep::sheep_react_to_being_inside_buildings::SheepReactionEmoji>,ref_mut$<country_core::components::Ui2dExtent>,bevy_ecs::entity::Entity>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::system::commands::Commands)> >", "constructor": "bevy_ecs::schedule::set::impl$7::dyn_clone", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<system_critters::sheep::sheep_react_to_being_inside_buildings::SheepReactionEmoji>,ref_mut$<country_core::components::Ui2dExtent>,bevy_ecs::entity::Entity>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::system::commands::Commands),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<system_critters::sheep::sheep_react_to_being_inside_buildings::SheepReactionEmoji>,ref_mut$<country_core::components::Ui2dExtent>,bevy_ecs::entity::Entity>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::system::commands::Commands)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::any::impl$0::type_id<bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<system_critters::sheep::sheep_react_to_being_inside_buildings::SheepReactionEmoji>,ref_mut$<country_core::components::Ui2dExtent>,bevy_ecs::entity::Entity>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::system::commands::Commands),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<system_critters::sheep::sheep_react_to_being_inside_buildings::SheepReactionEmoji>,ref_mut$<country_core::components::Ui2dExtent>,bevy_ecs::entity::Entity>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::system::commands::Commands)> > >", "constructor": "core::any::impl$0::type_id", "arguments": ["bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<system_critters::sheep::sheep_react_to_being_inside_buildings::SheepReactionEmoji>,ref_mut$<country_core::components::Ui2dExtent>,bevy_ecs::entity::Entity>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::system::commands::Commands),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<system_critters::sheep::sheep_react_to_being_inside_buildings::SheepReactionEmoji>,ref_mut$<country_core::components::Ui2dExtent>,bevy_ecs::entity::Entity>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::system::commands::Commands)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::label::impl$0::dyn_eq<bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<system_critters::sheep::sheep_react_to_being_inside_buildings::SheepReactionEmoji>,ref_mut$<country_core::components::Ui2dExtent>,bevy_ecs::entity::Entity>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::system::commands::Commands),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<system_critters::sheep::sheep_react_to_being_inside_buildings::SheepReactionEmoji>,ref_mut$<country_core::components::Ui2dExtent>,bevy_ecs::entity::Entity>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::system::commands::Commands)> > >", "constructor": "bevy_ecs::label::impl$0::dyn_eq", "arguments": ["bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<system_critters::sheep::sheep_react_to_being_inside_buildings::SheepReactionEmoji>,ref_mut$<country_core::components::Ui2dExtent>,bevy_ecs::entity::Entity>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::system::commands::Commands),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<system_critters::sheep::sheep_react_to_being_inside_buildings::SheepReactionEmoji>,ref_mut$<country_core::components::Ui2dExtent>,bevy_ecs::entity::Entity>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::system::commands::Commands)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$1::fmt<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<system_critters::sheep::sheep_react_to_being_inside_buildings::SheepReactionEmoji>,ref_mut$<country_core::components::Ui2dExtent>,bevy_ecs::entity::Entity>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::system::commands::Commands),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<system_critters::sheep::sheep_react_to_being_inside_buildings::SheepReactionEmoji>,ref_mut$<country_core::components::Ui2dExtent>,bevy_ecs::entity::Entity>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::system::commands::Commands)> >", "constructor": "bevy_ecs::schedule::set::impl$1::fmt", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<system_critters::sheep::sheep_react_to_being_inside_buildings::SheepReactionEmoji>,ref_mut$<country_core::components::Ui2dExtent>,bevy_ecs::entity::Entity>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::system::commands::Commands),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<system_critters::sheep::sheep_react_to_being_inside_buildings::SheepReactionEmoji>,ref_mut$<country_core::components::Ui2dExtent>,bevy_ecs::entity::Entity>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::system::commands::Commands)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<country_core::components::Ui2dExtent,country_core::components::Ui2dImage,system_critters::sheep::sheep_react_to_being_inside_buildings::SheepReactionEmoji>", "constructor": "tuple$", "arguments": ["country_core::components::Ui2dExtent", "country_core::components::Ui2dImage", "system_critters::sheep::sheep_react_to_being_inside_buildings::SheepReactionEmoji"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::commands::entity_command::insert::closure_env$0<tuple$<country_core::components::Ui2dExtent,country_core::components::Ui2dImage,system_critters::sheep::sheep_react_to_being_inside_buildings::SheepReactionEmoji> >", "constructor": "bevy_ecs::system::commands::entity_command::insert::closure_env$0", "arguments": ["tuple$<country_core::components::Ui2dExtent,country_core::components::Ui2dImage,system_critters::sheep::sheep_react_to_being_inside_buildings::SheepReactionEmoji>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0<bevy_ecs::system::commands::entity_command::insert::closure_env$0<tuple$<country_core::components::Ui2dExtent,country_core::components::Ui2dImage,system_critters::sheep::sheep_react_to_being_inside_buildings::SheepReactionEmoji> > >", "constructor": "bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0", "arguments": ["bevy_ecs::system::commands::entity_command::insert::closure_env$0<tuple$<country_core::components::Ui2dExtent,country_core::components::Ui2dImage,system_critters::sheep::sheep_react_to_being_inside_buildings::SheepReactionEmoji> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::error::command_handling::impl$0::handle_error_with::closure_env$0<bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0<bevy_ecs::system::commands::entity_command::insert::closure_env$0<tuple$<country_core::components::Ui2dExtent,country_core::components::Ui2dImage,system_critters::sheep::sheep_react_to_being_inside_buildings::SheepReactionEmoji> > >,tuple$<>,enum2$<bevy_ecs::world::error::EntityMutableFetchError> >", "constructor": "bevy_ecs::error::command_handling::impl$0::handle_error_with::closure_env$0", "arguments": ["bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0<bevy_ecs::system::commands::entity_command::insert::closure_env$0<tuple$<country_core::components::Ui2dExtent,country_core::components::Ui2dImage,system_critters::sheep::sheep_react_to_being_inside_buildings::SheepReactionEmoji> > >", "tuple$<>", "enum2$<bevy_ecs::world::error::EntityMutableFetchError>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::world::command_queue::impl$4::push::closure_env$0<bevy_ecs::error::command_handling::impl$0::handle_error_with::closure_env$0<bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0<bevy_ecs::system::commands::entity_command::insert::closure_env$0<tuple$<country_core::components::Ui2dExtent,country_core::components::Ui2dImage,system_critters::sheep::sheep_react_to_being_inside_buildings::SheepReactionEmoji> > >,tuple$<>,enum2$<bevy_ecs::world::error::EntityMutableFetchError> > >", "constructor": "bevy_ecs::world::command_queue::impl$4::push::closure_env$0", "arguments": ["bevy_ecs::error::command_handling::impl$0::handle_error_with::closure_env$0<bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0<bevy_ecs::system::commands::entity_command::insert::closure_env$0<tuple$<country_core::components::Ui2dExtent,country_core::components::Ui2dImage,system_critters::sheep::sheep_react_to_being_inside_buildings::SheepReactionEmoji> > >,tuple$<>,enum2$<bevy_ecs::world::error::EntityMutableFetchError> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::function::FnOnce::call_once<bevy_ecs::world::command_queue::impl$4::push::closure_env$0<bevy_ecs::error::command_handling::impl$0::handle_error_with::closure_env$0<bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0<bevy_ecs::system::commands::entity_command::insert::closure_env$0<tuple$<country_core::components::Ui2dExtent,country_core::components::Ui2dImage,system_critters::sheep::sheep_react_to_being_inside_buildings::SheepReactionEmoji> > >,tuple$<>,enum2$<bevy_ecs::world::error::EntityMutableFetchError> > >,tuple$<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2$<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World> > >,ref_mut$<usize> > >", "constructor": "core::ops::function::FnOnce::call_once", "arguments": ["bevy_ecs::world::command_queue::impl$4::push::closure_env$0<bevy_ecs::error::command_handling::impl$0::handle_error_with::closure_env$0<bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0<bevy_ecs::system::commands::entity_command::insert::closure_env$0<tuple$<country_core::components::Ui2dExtent,country_core::components::Ui2dImage,system_critters::sheep::sheep_react_to_being_inside_buildings::SheepReactionEmoji> > >,tuple$<>,enum2$<bevy_ecs::world::error::EntityMutableFetchError> > >", "tuple$<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2$<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World> > >,ref_mut$<usize> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::commands::Commands::spawn<tuple$<country_core::components::Ui2dExtent,country_core::components::Ui2dImage,system_critters::sheep::sheep_react_to_being_inside_buildings::SheepReactionEmoji> >", "constructor": "bevy_ecs::system::commands::Commands::spawn", "arguments": ["tuple$<country_core::components::Ui2dExtent,country_core::components::Ui2dImage,system_critters::sheep::sheep_react_to_being_inside_buildings::SheepReactionEmoji>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::commands::command::impl$0::apply<bevy_ecs::error::command_handling::impl$0::handle_error_with::closure_env$0<bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0<bevy_ecs::system::commands::entity_command::insert::closure_env$0<tuple$<country_core::components::Ui2dExtent,country_core::components::Ui2dImage,system_critters::sheep::sheep_react_to_being_inside_buildings::SheepReactionEmoji> > >,tuple$<>,enum2$<bevy_ecs::world::error::EntityMutableFetchError> >,tuple$<> >", "constructor": "bevy_ecs::system::commands::command::impl$0::apply", "arguments": ["bevy_ecs::error::command_handling::impl$0::handle_error_with::closure_env$0<bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0<bevy_ecs::system::commands::entity_command::insert::closure_env$0<tuple$<country_core::components::Ui2dExtent,country_core::components::Ui2dImage,system_critters::sheep::sheep_react_to_being_inside_buildings::SheepReactionEmoji> > >,tuple$<>,enum2$<bevy_ecs::world::error::EntityMutableFetchError> >", "tuple$<>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::config::impl$13::into_configs<tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::query::Query<tuple$<enum2$<core::option::Option<ref_mut$<utils::components::transform::Transform> > >,ref_mut$<country_core::resources::critters::sheep::Sheep> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)> >,void (*)(bevy_ecs::system::query::Query<tuple$<enum2$<core::option::Option<ref_mut$<utils::components::transform::Transform> > >,ref_mut$<country_core::resources::critters::sheep::Sheep> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>),tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::critters::sheep::Sheep>,ref$<utils::components::transform::Transform>,ref_mut$<country_core::resources::critters::sheep::SheepAudio> >,tuple$<> >,bevy_ecs::event::writer::EventWriter<system_critters::sheep::sheep_pet_animation::PlaySheepPetAnimation>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::system::system_param::Local<system_critters::sheep::ui_pet_sheep::IntersectionDepth>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>,bevy_ecs::event::reader::EventReader<country_core::resources::window::CursorMovedScreenSpace>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::sheep_audio::PlaySheepReactSound>)> >,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::critters::sheep::Sheep>,ref$<utils::components::transform::Transform>,ref_mut$<country_core::resources::critters::sheep::SheepAudio> >,tuple$<> >,bevy_ecs::event::writer::EventWriter<system_critters::sheep::sheep_pet_animation::PlaySheepPetAnimation>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::system::system_param::Local<system_critters::sheep::ui_pet_sheep::IntersectionDepth>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>,bevy_ecs::event::reader::EventReader<country_core::resources::window::CursorMovedScreenSpace>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::sheep_audio::PlaySheepReactSound>),tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::query::Query<ref_mut$<country_core::resources::critters::sheep::Sheep>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::event::reader::EventReader<system_critters::sheep::sheep_pet_animation::PlaySheepPetAnimation>)> >,void (*)(bevy_ecs::system::query::Query<ref_mut$<country_core::resources::critters::sheep::Sheep>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::event::reader::EventReader<system_critters::sheep::sheep_pet_animation::PlaySheepPetAnimation>),tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::event::reader::EventReader<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::critters::sheep::Sheep>,ref$<utils::components::transform::Transform>,ref$<country_core::resources::critters::sheep::SheepAudio> >,tuple$<> >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::sheep_audio::PlaySheepReactSound>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)> >,void (*)(bevy_ecs::event::reader::EventReader<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::critters::sheep::Sheep>,ref$<utils::components::transform::Transform>,ref$<country_core::resources::critters::sheep::SheepAudio> >,tuple$<> >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::sheep_audio::PlaySheepReactSound>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>),tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<system_critters::sheep::sheep_react_to_being_inside_buildings::SheepReactionEmoji>,ref_mut$<country_core::components::Ui2dExtent>,bevy_ecs::entity::Entity>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::system::commands::Commands)> >,void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<system_critters::sheep::sheep_react_to_being_inside_buildings::SheepReactionEmoji>,ref_mut$<country_core::components::Ui2dExtent>,bevy_ecs::entity::Entity>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::system::commands::Commands),alloc::boxed::Box<dyn$<bevy_ecs::system::system::System<assoc$<In,tuple$<> >,assoc$<Out,enum2$<core::result::Result<tuple$<>,bevy_ecs::error::bevy_error::BevyError> > > > >,alloc::alloc::Global> >", "constructor": "bevy_ecs::schedule::config::impl$13::into_configs", "arguments": ["tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::query::Query<tuple$<enum2$<core::option::Option<ref_mut$<utils::components::transform::Transform> > >,ref_mut$<country_core::resources::critters::sheep::Sheep> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)> >", "void (*)(bevy_ecs::system::query::Query<tuple$<enum2$<core::option::Option<ref_mut$<utils::components::transform::Transform> > >,ref_mut$<country_core::resources::critters::sheep::Sheep> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)", "tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::critters::sheep::Sheep>,ref$<utils::components::transform::Transform>,ref_mut$<country_core::resources::critters::sheep::SheepAudio> >,tuple$<> >,bevy_ecs::event::writer::EventWriter<system_critters::sheep::sheep_pet_animation::PlaySheepPetAnimation>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::system::system_param::Local<system_critters::sheep::ui_pet_sheep::IntersectionDepth>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>,bevy_ecs::event::reader::EventReader<country_core::resources::window::CursorMovedScreenSpace>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::sheep_audio::PlaySheepReactSound>)> >", "void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::critters::sheep::Sheep>,ref$<utils::components::transform::Transform>,ref_mut$<country_core::resources::critters::sheep::SheepAudio> >,tuple$<> >,bevy_ecs::event::writer::EventWriter<system_critters::sheep::sheep_pet_animation::PlaySheepPetAnimation>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::system::system_param::Local<system_critters::sheep::ui_pet_sheep::IntersectionDepth>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>,bevy_ecs::event::reader::EventReader<country_core::resources::window::CursorMovedScreenSpace>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::sheep_audio::PlaySheepReactSound>)", "tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::query::Query<ref_mut$<country_core::resources::critters::sheep::Sheep>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::event::reader::EventReader<system_critters::sheep::sheep_pet_animation::PlaySheepPetAnimation>)> >", "void (*)(bevy_ecs::system::query::Query<ref_mut$<country_core::resources::critters::sheep::Sheep>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::event::reader::EventReader<system_critters::sheep::sheep_pet_animation::PlaySheepPetAnimation>)", "tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::event::reader::EventReader<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::critters::sheep::Sheep>,ref$<utils::components::transform::Transform>,ref$<country_core::resources::critters::sheep::SheepAudio> >,tuple$<> >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::sheep_audio::PlaySheepReactSound>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)> >", "void (*)(bevy_ecs::event::reader::EventReader<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::critters::sheep::Sheep>,ref$<utils::components::transform::Transform>,ref$<country_core::resources::critters::sheep::SheepAudio> >,tuple$<> >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::sheep_audio::PlaySheepReactSound>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)", "tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<system_critters::sheep::sheep_react_to_being_inside_buildings::SheepReactionEmoji>,ref_mut$<country_core::components::Ui2dExtent>,bevy_ecs::entity::Entity>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::system::commands::Commands)> >", "void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<system_critters::sheep::sheep_react_to_being_inside_buildings::SheepReactionEmoji>,ref_mut$<country_core::components::Ui2dExtent>,bevy_ecs::entity::Entity>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::system::commands::Commands)", "alloc::boxed::Box<dyn$<bevy_ecs::system::system::System<assoc$<In,tuple$<> >,assoc$<Out,enum2$<core::result::Result<tuple$<>,bevy_ecs::error::bevy_error::BevyError> > > > >,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::initialize<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<system_critters::sheep::sheep_react_to_being_inside_buildings::SheepReactionEmoji>,ref_mut$<country_core::components::Ui2dExtent>,bevy_ecs::entity::Entity>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::system::commands::Commands),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<system_critters::sheep::sheep_react_to_being_inside_buildings::SheepReactionEmoji>,ref_mut$<country_core::components::Ui2dExtent>,bevy_ecs::entity::Entity>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::system::commands::Commands)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::initialize", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<system_critters::sheep::sheep_react_to_being_inside_buildings::SheepReactionEmoji>,ref_mut$<country_core::components::Ui2dExtent>,bevy_ecs::entity::Entity>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::system::commands::Commands),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<system_critters::sheep::sheep_react_to_being_inside_buildings::SheepReactionEmoji>,ref_mut$<country_core::components::Ui2dExtent>,bevy_ecs::entity::Entity>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::system::commands::Commands)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<system_critters::sheep::sheep_react_to_being_inside_buildings::SheepReactionEmoji>,ref_mut$<country_core::components::Ui2dExtent>,bevy_ecs::entity::Entity>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::system::commands::Commands),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<system_critters::sheep::sheep_react_to_being_inside_buildings::SheepReactionEmoji>,ref_mut$<country_core::components::Ui2dExtent>,bevy_ecs::entity::Entity>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::system::commands::Commands)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::run_unsafe", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<system_critters::sheep::sheep_react_to_being_inside_buildings::SheepReactionEmoji>,ref_mut$<country_core::components::Ui2dExtent>,bevy_ecs::entity::Entity>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::system::commands::Commands),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<system_critters::sheep::sheep_react_to_being_inside_buildings::SheepReactionEmoji>,ref_mut$<country_core::components::Ui2dExtent>,bevy_ecs::entity::Entity>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::system::commands::Commands)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::get_last_run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<system_critters::sheep::sheep_react_to_being_inside_buildings::SheepReactionEmoji>,ref_mut$<country_core::components::Ui2dExtent>,bevy_ecs::entity::Entity>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::system::commands::Commands),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<system_critters::sheep::sheep_react_to_being_inside_buildings::SheepReactionEmoji>,ref_mut$<country_core::components::Ui2dExtent>,bevy_ecs::entity::Entity>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::system::commands::Commands)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::get_last_run", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<system_critters::sheep::sheep_react_to_being_inside_buildings::SheepReactionEmoji>,ref_mut$<country_core::components::Ui2dExtent>,bevy_ecs::entity::Entity>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::system::commands::Commands),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<system_critters::sheep::sheep_react_to_being_inside_buildings::SheepReactionEmoji>,ref_mut$<country_core::components::Ui2dExtent>,bevy_ecs::entity::Entity>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::system::commands::Commands)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::has_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<system_critters::sheep::sheep_react_to_being_inside_buildings::SheepReactionEmoji>,ref_mut$<country_core::components::Ui2dExtent>,bevy_ecs::entity::Entity>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::system::commands::Commands),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<system_critters::sheep::sheep_react_to_being_inside_buildings::SheepReactionEmoji>,ref_mut$<country_core::components::Ui2dExtent>,bevy_ecs::entity::Entity>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::system::commands::Commands)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::has_deferred", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<system_critters::sheep::sheep_react_to_being_inside_buildings::SheepReactionEmoji>,ref_mut$<country_core::components::Ui2dExtent>,bevy_ecs::entity::Entity>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::system::commands::Commands),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<system_critters::sheep::sheep_react_to_being_inside_buildings::SheepReactionEmoji>,ref_mut$<country_core::components::Ui2dExtent>,bevy_ecs::entity::Entity>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::system::commands::Commands)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::is_exclusive<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<system_critters::sheep::sheep_react_to_being_inside_buildings::SheepReactionEmoji>,ref_mut$<country_core::components::Ui2dExtent>,bevy_ecs::entity::Entity>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::system::commands::Commands),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<system_critters::sheep::sheep_react_to_being_inside_buildings::SheepReactionEmoji>,ref_mut$<country_core::components::Ui2dExtent>,bevy_ecs::entity::Entity>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::system::commands::Commands)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::is_exclusive", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<system_critters::sheep::sheep_react_to_being_inside_buildings::SheepReactionEmoji>,ref_mut$<country_core::components::Ui2dExtent>,bevy_ecs::entity::Entity>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::system::commands::Commands),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<system_critters::sheep::sheep_react_to_being_inside_buildings::SheepReactionEmoji>,ref_mut$<country_core::components::Ui2dExtent>,bevy_ecs::entity::Entity>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::system::commands::Commands)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::set_last_run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<system_critters::sheep::sheep_react_to_being_inside_buildings::SheepReactionEmoji>,ref_mut$<country_core::components::Ui2dExtent>,bevy_ecs::entity::Entity>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::system::commands::Commands),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<system_critters::sheep::sheep_react_to_being_inside_buildings::SheepReactionEmoji>,ref_mut$<country_core::components::Ui2dExtent>,bevy_ecs::entity::Entity>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::system::commands::Commands)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::set_last_run", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<system_critters::sheep::sheep_react_to_being_inside_buildings::SheepReactionEmoji>,ref_mut$<country_core::components::Ui2dExtent>,bevy_ecs::entity::Entity>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::system::commands::Commands),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<system_critters::sheep::sheep_react_to_being_inside_buildings::SheepReactionEmoji>,ref_mut$<country_core::components::Ui2dExtent>,bevy_ecs::entity::Entity>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::system::commands::Commands)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::apply_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<system_critters::sheep::sheep_react_to_being_inside_buildings::SheepReactionEmoji>,ref_mut$<country_core::components::Ui2dExtent>,bevy_ecs::entity::Entity>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::system::commands::Commands),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<system_critters::sheep::sheep_react_to_being_inside_buildings::SheepReactionEmoji>,ref_mut$<country_core::components::Ui2dExtent>,bevy_ecs::entity::Entity>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::system::commands::Commands)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::apply_deferred", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<system_critters::sheep::sheep_react_to_being_inside_buildings::SheepReactionEmoji>,ref_mut$<country_core::components::Ui2dExtent>,bevy_ecs::entity::Entity>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::system::commands::Commands),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<system_critters::sheep::sheep_react_to_being_inside_buildings::SheepReactionEmoji>,ref_mut$<country_core::components::Ui2dExtent>,bevy_ecs::entity::Entity>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::system::commands::Commands)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::queue_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<system_critters::sheep::sheep_react_to_being_inside_buildings::SheepReactionEmoji>,ref_mut$<country_core::components::Ui2dExtent>,bevy_ecs::entity::Entity>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::system::commands::Commands),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<system_critters::sheep::sheep_react_to_being_inside_buildings::SheepReactionEmoji>,ref_mut$<country_core::components::Ui2dExtent>,bevy_ecs::entity::Entity>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::system::commands::Commands)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::queue_deferred", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<system_critters::sheep::sheep_react_to_being_inside_buildings::SheepReactionEmoji>,ref_mut$<country_core::components::Ui2dExtent>,bevy_ecs::entity::Entity>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::system::commands::Commands),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<system_critters::sheep::sheep_react_to_being_inside_buildings::SheepReactionEmoji>,ref_mut$<country_core::components::Ui2dExtent>,bevy_ecs::entity::Entity>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::system::commands::Commands)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::component_access<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<system_critters::sheep::sheep_react_to_being_inside_buildings::SheepReactionEmoji>,ref_mut$<country_core::components::Ui2dExtent>,bevy_ecs::entity::Entity>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::system::commands::Commands),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<system_critters::sheep::sheep_react_to_being_inside_buildings::SheepReactionEmoji>,ref_mut$<country_core::components::Ui2dExtent>,bevy_ecs::entity::Entity>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::system::commands::Commands)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::component_access", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<system_critters::sheep::sheep_react_to_being_inside_buildings::SheepReactionEmoji>,ref_mut$<country_core::components::Ui2dExtent>,bevy_ecs::entity::Entity>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::system::commands::Commands),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<system_critters::sheep::sheep_react_to_being_inside_buildings::SheepReactionEmoji>,ref_mut$<country_core::components::Ui2dExtent>,bevy_ecs::entity::Entity>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::system::commands::Commands)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::check_change_tick<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<system_critters::sheep::sheep_react_to_being_inside_buildings::SheepReactionEmoji>,ref_mut$<country_core::components::Ui2dExtent>,bevy_ecs::entity::Entity>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::system::commands::Commands),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<system_critters::sheep::sheep_react_to_being_inside_buildings::SheepReactionEmoji>,ref_mut$<country_core::components::Ui2dExtent>,bevy_ecs::entity::Entity>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::system::commands::Commands)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::check_change_tick", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<system_critters::sheep::sheep_react_to_being_inside_buildings::SheepReactionEmoji>,ref_mut$<country_core::components::Ui2dExtent>,bevy_ecs::entity::Entity>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::system::commands::Commands),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<system_critters::sheep::sheep_react_to_being_inside_buildings::SheepReactionEmoji>,ref_mut$<country_core::components::Ui2dExtent>,bevy_ecs::entity::Entity>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::system::commands::Commands)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::validate_param_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<system_critters::sheep::sheep_react_to_being_inside_buildings::SheepReactionEmoji>,ref_mut$<country_core::components::Ui2dExtent>,bevy_ecs::entity::Entity>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::system::commands::Commands),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<system_critters::sheep::sheep_react_to_being_inside_buildings::SheepReactionEmoji>,ref_mut$<country_core::components::Ui2dExtent>,bevy_ecs::entity::Entity>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::system::commands::Commands)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::validate_param_unsafe", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<system_critters::sheep::sheep_react_to_being_inside_buildings::SheepReactionEmoji>,ref_mut$<country_core::components::Ui2dExtent>,bevy_ecs::entity::Entity>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::system::commands::Commands),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<system_critters::sheep::sheep_react_to_being_inside_buildings::SheepReactionEmoji>,ref_mut$<country_core::components::Ui2dExtent>,bevy_ecs::entity::Entity>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::system::commands::Commands)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::archetype_component_access<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<system_critters::sheep::sheep_react_to_being_inside_buildings::SheepReactionEmoji>,ref_mut$<country_core::components::Ui2dExtent>,bevy_ecs::entity::Entity>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::system::commands::Commands),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<system_critters::sheep::sheep_react_to_being_inside_buildings::SheepReactionEmoji>,ref_mut$<country_core::components::Ui2dExtent>,bevy_ecs::entity::Entity>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::system::commands::Commands)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::archetype_component_access", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<system_critters::sheep::sheep_react_to_being_inside_buildings::SheepReactionEmoji>,ref_mut$<country_core::components::Ui2dExtent>,bevy_ecs::entity::Entity>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::system::commands::Commands),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<system_critters::sheep::sheep_react_to_being_inside_buildings::SheepReactionEmoji>,ref_mut$<country_core::components::Ui2dExtent>,bevy_ecs::entity::Entity>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::system::commands::Commands)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::update_archetype_component_access<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<system_critters::sheep::sheep_react_to_being_inside_buildings::SheepReactionEmoji>,ref_mut$<country_core::components::Ui2dExtent>,bevy_ecs::entity::Entity>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::system::commands::Commands),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<system_critters::sheep::sheep_react_to_being_inside_buildings::SheepReactionEmoji>,ref_mut$<country_core::components::Ui2dExtent>,bevy_ecs::entity::Entity>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::system::commands::Commands)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::update_archetype_component_access", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<system_critters::sheep::sheep_react_to_being_inside_buildings::SheepReactionEmoji>,ref_mut$<country_core::components::Ui2dExtent>,bevy_ecs::entity::Entity>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::system::commands::Commands),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<system_critters::sheep::sheep_react_to_being_inside_buildings::SheepReactionEmoji>,ref_mut$<country_core::components::Ui2dExtent>,bevy_ecs::entity::Entity>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::system::commands::Commands)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::name<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<system_critters::sheep::sheep_react_to_being_inside_buildings::SheepReactionEmoji>,ref_mut$<country_core::components::Ui2dExtent>,bevy_ecs::entity::Entity>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::system::commands::Commands),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<system_critters::sheep::sheep_react_to_being_inside_buildings::SheepReactionEmoji>,ref_mut$<country_core::components::Ui2dExtent>,bevy_ecs::entity::Entity>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::system::commands::Commands)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::name", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<system_critters::sheep::sheep_react_to_being_inside_buildings::SheepReactionEmoji>,ref_mut$<country_core::components::Ui2dExtent>,bevy_ecs::entity::Entity>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::system::commands::Commands),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<system_critters::sheep::sheep_react_to_being_inside_buildings::SheepReactionEmoji>,ref_mut$<country_core::components::Ui2dExtent>,bevy_ecs::entity::Entity>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::system::commands::Commands)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::is_send<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<system_critters::sheep::sheep_react_to_being_inside_buildings::SheepReactionEmoji>,ref_mut$<country_core::components::Ui2dExtent>,bevy_ecs::entity::Entity>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::system::commands::Commands),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<system_critters::sheep::sheep_react_to_being_inside_buildings::SheepReactionEmoji>,ref_mut$<country_core::components::Ui2dExtent>,bevy_ecs::entity::Entity>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::system::commands::Commands)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::is_send", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<system_critters::sheep::sheep_react_to_being_inside_buildings::SheepReactionEmoji>,ref_mut$<country_core::components::Ui2dExtent>,bevy_ecs::entity::Entity>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::system::commands::Commands),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<system_critters::sheep::sheep_react_to_being_inside_buildings::SheepReactionEmoji>,ref_mut$<country_core::components::Ui2dExtent>,bevy_ecs::entity::Entity>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::system::commands::Commands)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::config::impl$4::into_configs<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<system_critters::sheep::sheep_react_to_being_inside_buildings::SheepReactionEmoji>,ref_mut$<country_core::components::Ui2dExtent>,bevy_ecs::entity::Entity>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::system::commands::Commands),tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<system_critters::sheep::sheep_react_to_being_inside_buildings::SheepReactionEmoji>,ref_mut$<country_core::components::Ui2dExtent>,bevy_ecs::entity::Entity>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::system::commands::Commands)> >", "constructor": "bevy_ecs::schedule::config::impl$4::into_configs", "arguments": ["void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<system_critters::sheep::sheep_react_to_being_inside_buildings::SheepReactionEmoji>,ref_mut$<country_core::components::Ui2dExtent>,bevy_ecs::entity::Entity>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::system::commands::Commands)", "tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<system_critters::sheep::sheep_react_to_being_inside_buildings::SheepReactionEmoji>,ref_mut$<country_core::components::Ui2dExtent>,bevy_ecs::entity::Entity>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::system::commands::Commands)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::query::world_query::impl$3::update_component_access<ref_mut$<system_critters::sheep::sheep_react_to_being_inside_buildings::SheepReactionEmoji>,ref_mut$<country_core::components::Ui2dExtent>,bevy_ecs::entity::Entity>", "constructor": "bevy_ecs::query::world_query::impl$3::update_component_access", "arguments": ["ref_mut$<system_critters::sheep::sheep_react_to_being_inside_buildings::SheepReactionEmoji>", "ref_mut$<country_core::components::Ui2dExtent>", "bevy_ecs::entity::Entity"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::type_id<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<system_critters::sheep::sheep_react_to_being_inside_buildings::SheepReactionEmoji>,ref_mut$<country_core::components::Ui2dExtent>,bevy_ecs::entity::Entity>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::system::commands::Commands),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<system_critters::sheep::sheep_react_to_being_inside_buildings::SheepReactionEmoji>,ref_mut$<country_core::components::Ui2dExtent>,bevy_ecs::entity::Entity>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::system::commands::Commands)> > >", "constructor": "bevy_ecs::system::system::System::type_id", "arguments": ["bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<system_critters::sheep::sheep_react_to_being_inside_buildings::SheepReactionEmoji>,ref_mut$<country_core::components::Ui2dExtent>,bevy_ecs::entity::Entity>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::system::commands::Commands),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<system_critters::sheep::sheep_react_to_being_inside_buildings::SheepReactionEmoji>,ref_mut$<country_core::components::Ui2dExtent>,bevy_ecs::entity::Entity>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::system::commands::Commands)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::component::ComponentsRegistrator::register_component<system_critters::sheep::sheep_react_to_being_inside_buildings::SheepReactionEmoji>", "constructor": "bevy_ecs::component::ComponentsRegistrator::register_component", "arguments": ["system_critters::sheep::sheep_react_to_being_inside_buildings::SheepReactionEmoji"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::bundle::impl$8::register_info::closure$0::closure_env$0<tuple$<country_core::components::Ui2dExtent,country_core::components::Ui2dImage,system_critters::sheep::sheep_react_to_being_inside_buildings::SheepReactionEmoji> >", "constructor": "bevy_ecs::bundle::impl$8::register_info::closure$0::closure_env$0", "arguments": ["tuple$<country_core::components::Ui2dExtent,country_core::components::Ui2dImage,system_critters::sheep::sheep_react_to_being_inside_buildings::SheepReactionEmoji>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::bundle::impl$18::component_ids<country_core::components::Ui2dExtent,country_core::components::Ui2dImage,system_critters::sheep::sheep_react_to_being_inside_buildings::SheepReactionEmoji,bevy_ecs::bundle::impl$8::register_info::closure$0::closure_env$0<tuple$<country_core::components::Ui2dExtent,country_core::components::Ui2dImage,system_critters::sheep::sheep_react_to_being_inside_buildings::SheepReactionEmoji> > >", "constructor": "bevy_ecs::bundle::impl$18::component_ids", "arguments": ["country_core::components::Ui2dExtent", "country_core::components::Ui2dImage", "system_critters::sheep::sheep_react_to_being_inside_buildings::SheepReactionEmoji", "bevy_ecs::bundle::impl$8::register_info::closure$0::closure_env$0<tuple$<country_core::components::Ui2dExtent,country_core::components::Ui2dImage,system_critters::sheep::sheep_react_to_being_inside_buildings::SheepReactionEmoji> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::bundle::BundleInfo::write_components<tuple$<country_core::components::Ui2dExtent,country_core::components::Ui2dImage,system_critters::sheep::sheep_react_to_being_inside_buildings::SheepReactionEmoji>,bevy_ecs::archetype::ArchetypeAfterBundleInsert,core::slice::iter::Iter<bevy_ecs::component::RequiredComponentConstructor> >", "constructor": "bevy_ecs::bundle::BundleInfo::write_components", "arguments": ["tuple$<country_core::components::Ui2dExtent,country_core::components::Ui2dImage,system_critters::sheep::sheep_react_to_being_inside_buildings::SheepReactionEmoji>", "bevy_ecs::archetype::ArchetypeAfterBundleInsert", "core::slice::iter::Iter<bevy_ecs::component::RequiredComponentConstructor>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::commands::entity_command::impl$0::apply<tuple$<>,bevy_ecs::system::commands::entity_command::insert::closure_env$0<tuple$<country_core::components::Ui2dExtent,country_core::components::Ui2dImage,system_critters::sheep::sheep_react_to_being_inside_buildings::SheepReactionEmoji> > >", "constructor": "bevy_ecs::system::commands::entity_command::impl$0::apply", "arguments": ["tuple$<>", "bevy_ecs::system::commands::entity_command::insert::closure_env$0<tuple$<country_core::components::Ui2dExtent,country_core::components::Ui2dImage,system_critters::sheep::sheep_react_to_being_inside_buildings::SheepReactionEmoji> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::bundle::Bundles::register_info<tuple$<country_core::components::Ui2dExtent,country_core::components::Ui2dImage,system_critters::sheep::sheep_react_to_being_inside_buildings::SheepReactionEmoji> >", "constructor": "bevy_ecs::bundle::Bundles::register_info", "arguments": ["tuple$<country_core::components::Ui2dExtent,country_core::components::Ui2dImage,system_critters::sheep::sheep_react_to_being_inside_buildings::SheepReactionEmoji>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: sheep_react_to_being_inside_buildings.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// [已有模块证据关联；不自动成为字段事实] evidence/parallel-camera/B-decompile/report.json; explicit B; [{"kind": "exact_PDB_named_entity_mention", "name": "system_critters::sheep::sheep_react_to_being_inside_buildings::SheepReactionEmoji", "status": "evidence_association; original field ownership unknown"}]
// [已有模块证据关联；不自动成为字段事实] evidence/parallel-critters/B-decompile/report.json; explicit B; [{"kind": "exact_PDB_named_entity_mention", "name": "system_critters::sheep::sheep_react_to_being_inside_buildings::SheepReactionEmoji", "status": "evidence_association; original field ownership unknown"}]
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: sheep_react_to_being_inside_buildings.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 2; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::event::reader::EventReader<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::critters::sheep::Sheep>,ref$<utils::components::transform::Transform>,ref$<country_core::resources::critters::sheep::SheepAudio> >,tuple$<> >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::sheep_audio::PlaySheepReactSound>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)
// 候选参数类型: ["bevy_ecs::event::reader::EventReader<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>", "bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::critters::sheep::Sheep>,ref$<utils::components::transform::Transform>,ref$<country_core::resources::critters::sheep::SheepAudio> >,tuple$<> >", "bevy_ecs::system::commands::Commands", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >", "bevy_ecs::event::writer::EventWriter<country_core::systems::audio::sheep_audio::PlaySheepReactSound>", "bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>"]
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::critters::sheep::Sheep>,ref$<utils::components::transform::Transform>,ref$<country_core::resources::critters::sheep::SheepAudio> >,tuple$<> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::systems::audio::sheep_audio::PlaySheepReactSound>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<system_critters::sheep::sheep_react_to_being_inside_buildings::SheepReactionEmoji>,ref_mut$<country_core::components::Ui2dExtent>,bevy_ecs::entity::Entity>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::system::query::Query<tuple$<ref_mut$<system_critters::sheep::sheep_react_to_being_inside_buildings::SheepReactionEmoji>,ref_mut$<country_core::components::Ui2dExtent>,bevy_ecs::entity::Entity>,tuple$<> >", "bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>", "bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>", "bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<ref_mut$<system_critters::sheep::sheep_react_to_being_inside_buildings::SheepReactionEmoji>,ref_mut$<country_core::components::Ui2dExtent>,bevy_ecs::entity::Entity>,tuple$<> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands

// ------------------------------------------------------------------
// 函数: system_critters::sheep::sheep_react_to_being_inside_buildings::impl$0::register_required_components
// 信息 ID: 183d380e01231fb0dd30be1f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::component::ComponentsRegistrator::register_component<system_critters::sheep::sheep_react_to_being_inside_buildings::SheepReactionEmoji> @ 0x2309a75 -> 0x2320480 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_critters::sheep::sheep_react_to_being_inside_buildings::impl$0::register_required_components @ 0x232091a -> 0x2320480 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_critters::sheep::sheep_react_to_being_inside_buildings::react_to_being_inside_buildings
// 信息 ID: 06eeb1c53ed2ae7bae31ad64
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] ac3b1a3ea9f9df4add1b4ff8: void (*)(bevy_ecs::event::reader::EventReader<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::critters::sheep::Sheep>,ref$<utils::components::transform::Transform>,ref$<country_core::resources::critters::sheep::SheepAudio> >,tuple$<> >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::sheep_audio::PlaySheepReactSound>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::event::reader::EventReader<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::critters::sheep::Sheep>,ref$<utils::components::transform::Transform>,ref$<country_core::resources::critters::sheep::SheepAudio> >,tuple$<> >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::sheep_audio::PlaySheepReactSound>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>),void (*)(bevy_ecs::event::reader::EventReader<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::critters::sheep::Sheep>,ref$<utils::components::transform::Transform>,ref$<country_core::resources::critters::sheep::SheepAudio> >,tuple$<> >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::sheep_audio::PlaySheepReactSound>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)> > > @ 0x1d9e539 -> 0x231f610 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::schedule_system::impl$1::run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::event::reader::EventReader<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::critters::sheep::Sheep>,ref$<utils::components::transform::Transform>,ref$<country_core::resources::critters::sheep::SheepAudio> >,tuple$<> >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::sheep_audio::PlaySheepReactSound>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>),void (*)(bevy_ecs::event::reader::EventReader<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::critters::sheep::Sheep>,ref$<utils::components::transform::Transform>,ref$<country_core::resources::critters::sheep::SheepAudio> >,tuple$<> >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::sheep_audio::PlaySheepReactSound>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)> > @ 0x2308a21 -> 0x231f610 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_critters::sheep::sheep_react_to_being_inside_buildings::react_to_being_inside_buildings::closure$0
// 信息 ID: c458bfc8ab2ede5fd8d4a7a3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_critters::sheep::sheep_react_to_being_inside_buildings::react_to_being_inside_buildings", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_critters::sheep::sheep_react_to_being_inside_buildings::react_to_being_inside_buildings; module=3887, inline_record=6708; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_critters::sheep::sheep_react_to_being_inside_buildings::react_to_being_inside_buildings::closure$1
// 信息 ID: 7482777c918de98c0c93ba53
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "system_critters::sheep::sheep_react_to_being_inside_buildings::react_to_being_inside_buildings", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_critters::sheep::sheep_react_to_being_inside_buildings::react_to_being_inside_buildings; module=3887, inline_record=8236; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_critters::sheep::sheep_react_to_being_inside_buildings::sheep_reaction_emoji
// 信息 ID: ffae74dc580ae01fff244a02
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] dba48b676c4dd149064cec20: void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<system_critters::sheep::sheep_react_to_being_inside_buildings::SheepReactionEmoji>,ref_mut$<country_core::components::Ui2dExtent>,bevy_ecs::entity::Entity>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::system::commands::Commands)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<system_critters::sheep::sheep_react_to_being_inside_buildings::SheepReactionEmoji>,ref_mut$<country_core::components::Ui2dExtent>,bevy_ecs::entity::Entity>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::system::commands::Commands),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<system_critters::sheep::sheep_react_to_being_inside_buildings::SheepReactionEmoji>,ref_mut$<country_core::components::Ui2dExtent>,bevy_ecs::entity::Entity>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::system::commands::Commands)> > > @ 0x1da4635 -> 0x231ff70 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::schedule_system::impl$1::run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<system_critters::sheep::sheep_react_to_being_inside_buildings::SheepReactionEmoji>,ref_mut$<country_core::components::Ui2dExtent>,bevy_ecs::entity::Entity>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::system::commands::Commands),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<system_critters::sheep::sheep_react_to_being_inside_buildings::SheepReactionEmoji>,ref_mut$<country_core::components::Ui2dExtent>,bevy_ecs::entity::Entity>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::system::commands::Commands)> > @ 0x2308660 -> 0x231ff70 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
