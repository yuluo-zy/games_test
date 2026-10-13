// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/systems/decorator/src/cull_oob_decorators.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: a337fe462509fa1be95847b4c00151320d2d51fa6749827b6930845d2ced4ba0
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: cull_oob_decorators.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 453, 1504, 1510, 1522, 1552, 1562, 1570, 1576
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
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.3"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.5"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.15.2"}]
// use nonmax; // namespace candidate
//   package identities: [{"name":"nonmax","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.5.5"}]
// use puffin; // namespace candidate
//   package identities: [{"name":"puffin","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.1"}]
// use std; // namespace candidate
//   package identities: []
// use system_decorator; // namespace candidate
//   package identities: [{"name":"system-decorator","source":"local_or_path","version":"0.1.0"}]
// use utils; // namespace candidate
//   package identities: [{"name":"utils","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_decorator::cull_oob_decorators::check_if_dormer_ridge_is_within_roof_bounds
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":453,"record_offset":43264,"rva":8459328,"va":5377168448}
// 编译实例 RVAs: 0x811440
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: cull_oob_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2087378","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377168448,"raw_hex":"e82e26e0ff","size":5,"soft_fail":false,"target_va":5375081504,"va":5377168877,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c9cdc0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: cull_oob_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4765346","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377168448,"raw_hex":"e8a2b64800","size":5,"soft_fail":false,"target_va":5381934528,"va":5377169177,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c9cde0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: cull_oob_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4765524","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377168448,"raw_hex":"e854b74800","size":5,"soft_fail":false,"target_va":5381934560,"va":5377169031,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t34373481","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377168448,"raw_hex":"e8697f0c02","size":5,"soft_fail":false,"target_va":5411542296,"va":5377168810,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::gothic_window_profile::gothic_half_dormer_profile_curve
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1652520","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377168448,"raw_hex":"e828371900","size":5,"soft_fail":false,"target_va":5378821312,"va":5377168787,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::roofs::roof_shape::Roof::height
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t859066","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377168448,"raw_hex":"e8ba1b0d00","size":5,"soft_fail":false,"target_va":5378028128,"va":5377169057,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::roofs::roof_shape::Roof::roof_shape_at_intersection_y
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t860665","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377168448,"raw_hex":"e8f9210d00","size":5,"soft_fail":false,"target_va":5378029600,"va":5377168930,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_walls::PublicWalls::roof_entity
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2624999","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377168448,"raw_hex":"e8e70d2800","size":5,"soft_fail":false,"target_va":5379793584,"va":5377168580,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_decorator::decorator_visual::gothic_dormer::gothic_dormer_profile_curve
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-303301","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377168448,"raw_hex":"e83b5ffbff","size":5,"soft_fail":false,"target_va":5376865536,"va":5377168832,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::circle::Circle2d::closest_point_on_perimeter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t4744399","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377168448,"raw_hex":"e8cf644800","size":5,"soft_fail":false,"target_va":5381913408,"va":5377169004,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::closest_point_on_perimeter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t4757447","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377168448,"raw_hex":"e8c7974800","size":5,"soft_fail":false,"target_va":5381926432,"va":5377168980,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::impl$13::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":453,"record_offset":44992,"parent_record_offset":43264,"ancestor_record_offset":43264,"inlinee":4358,"source_line":3357,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::and_then
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":453,"record_offset":43460,"parent_record_offset":43264,"ancestor_record_offset":43264,"inlinee":4518,"source_line":1447,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":453,"record_offset":45172,"parent_record_offset":43264,"ancestor_record_offset":43264,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":453,"record_offset":45944,"parent_record_offset":43264,"ancestor_record_offset":43264,"inlinee":4749,"source_line":166,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::normalize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":453,"record_offset":45708,"parent_record_offset":43264,"ancestor_record_offset":43264,"inlinee":10290,"source_line":466,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":453,"record_offset":45468,"parent_record_offset":43264,"ancestor_record_offset":43264,"inlinee":4590,"source_line":1245,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":453,"record_offset":45524,"parent_record_offset":43264,"ancestor_record_offset":43264,"inlinee":4752,"source_line":1333,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":453,"record_offset":45612,"parent_record_offset":43264,"ancestor_record_offset":43264,"inlinee":4743,"source_line":1473,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":453,"record_offset":45364,"parent_record_offset":45340,"ancestor_record_offset":43264,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":453,"record_offset":45340,"parent_record_offset":45252,"ancestor_record_offset":43264,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":453,"record_offset":43760,"parent_record_offset":43736,"ancestor_record_offset":43264,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":453,"record_offset":45280,"parent_record_offset":45252,"ancestor_record_offset":43264,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":453,"record_offset":45252,"parent_record_offset":45228,"ancestor_record_offset":43264,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":453,"record_offset":43808,"parent_record_offset":43784,"ancestor_record_offset":43264,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":453,"record_offset":43784,"parent_record_offset":43760,"ancestor_record_offset":43264,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":453,"record_offset":45228,"parent_record_offset":45204,"ancestor_record_offset":43264,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":453,"record_offset":43736,"parent_record_offset":43712,"ancestor_record_offset":43264,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":453,"record_offset":43712,"parent_record_offset":43612,"ancestor_record_offset":43264,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":453,"record_offset":44148,"parent_record_offset":44124,"ancestor_record_offset":43264,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::impl$25::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":453,"record_offset":43912,"parent_record_offset":43580,"ancestor_record_offset":43264,"inlinee":4846,"source_line":76,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::Entities::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/entity/mod.rs
//     evidence={"module":453,"record_offset":43580,"parent_record_offset":43536,"ancestor_record_offset":43264,"inlinee":4840,"source_line":842,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::StorageSwitch::extract
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":453,"record_offset":44860,"parent_record_offset":44836,"ancestor_record_offset":43264,"inlinee":4876,"source_line":2472,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$26::set_archetype
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":453,"record_offset":44384,"parent_record_offset":43536,"ancestor_record_offset":43264,"inlinee":4856,"source_line":1160,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$26::set_table
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":453,"record_offset":44408,"parent_record_offset":44384,"ancestor_record_offset":43264,"inlinee":4857,"source_line":1175,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$27::fetch
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":453,"record_offset":44836,"parent_record_offset":43536,"ancestor_record_offset":43264,"inlinee":4874,"source_line":1229,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$27::fetch::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":453,"record_offset":44884,"parent_record_offset":44860,"ancestor_record_offset":43264,"inlinee":4878,"source_line":1235,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_array::BlobArray::get_sub_slice
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_array.rs
//     evidence={"module":453,"record_offset":44784,"parent_record_offset":44760,"ancestor_record_offset":43264,"inlinee":4872,"source_line":143,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseArray::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":453,"record_offset":44508,"parent_record_offset":44484,"ancestor_record_offset":43264,"inlinee":4864,"source_line":58,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":453,"record_offset":44484,"parent_record_offset":44460,"ancestor_record_offset":43264,"inlinee":4862,"source_line":395,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$16::get::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":453,"record_offset":44644,"parent_record_offset":44620,"ancestor_record_offset":43264,"inlinee":4866,"source_line":396,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_column
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":453,"record_offset":44460,"parent_record_offset":44432,"ancestor_record_offset":43264,"inlinee":4860,"source_line":475,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_data_slice_for
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":453,"record_offset":44432,"parent_record_offset":44408,"ancestor_record_offset":43264,"inlinee":4859,"source_line":363,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":453,"record_offset":44124,"parent_record_offset":43536,"ancestor_record_offset":43264,"inlinee":4854,"source_line":721,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::column::ThinColumn::get_data_slice
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/column.rs
//     evidence={"module":453,"record_offset":44760,"parent_record_offset":44736,"ancestor_record_offset":43264,"inlinee":4870,"source_line":303,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::impl$4::get_data_slice_for::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":453,"record_offset":44736,"parent_record_offset":44712,"ancestor_record_offset":43264,"inlinee":4868,"source_line":368,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":453,"record_offset":43512,"parent_record_offset":43488,"ancestor_record_offset":43264,"inlinee":4837,"source_line":1385,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::get_inner
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":453,"record_offset":43536,"parent_record_offset":43512,"ancestor_record_offset":43264,"inlinee":4838,"source_line":1591,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ptr::ThinSlicePtr::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ptr","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ptr-0.16.0/src/lib.rs
//     evidence={"module":453,"record_offset":44908,"parent_record_offset":44884,"ancestor_record_offset":43264,"inlinee":4880,"source_line":526,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::bool::impl$0::then
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\bool.rs
//     evidence={"module":453,"record_offset":43972,"parent_record_offset":43944,"ancestor_record_offset":43264,"inlinee":4781,"source_line":61,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$24::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":453,"record_offset":43880,"parent_record_offset":43856,"ancestor_record_offset":43264,"inlinee":4844,"source_line":1742,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":453,"record_offset":45308,"parent_record_offset":45280,"ancestor_record_offset":43264,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::impl$2::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":453,"record_offset":43856,"parent_record_offset":43580,"ancestor_record_offset":43264,"inlinee":4842,"source_line":200,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":453,"record_offset":45552,"parent_record_offset":45524,"ancestor_record_offset":43264,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":453,"record_offset":45492,"parent_record_offset":45468,"ancestor_record_offset":43264,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":453,"record_offset":45648,"parent_record_offset":45612,"ancestor_record_offset":43264,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":453,"record_offset":44556,"parent_record_offset":44532,"ancestor_record_offset":43264,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":453,"record_offset":44532,"parent_record_offset":44508,"ancestor_record_offset":43264,"inlinee":4518,"source_line":1447,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":453,"record_offset":44580,"parent_record_offset":44556,"ancestor_record_offset":43264,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":453,"record_offset":44620,"parent_record_offset":44484,"ancestor_record_offset":43264,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":453,"record_offset":45204,"parent_record_offset":45172,"ancestor_record_offset":43264,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":453,"record_offset":44076,"parent_record_offset":44044,"ancestor_record_offset":43264,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":453,"record_offset":43612,"parent_record_offset":43580,"ancestor_record_offset":43264,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_noubcheck
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":453,"record_offset":43680,"parent_record_offset":43652,"ancestor_record_offset":43264,"inlinee":4164,"source_line":87,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":453,"record_offset":45016,"parent_record_offset":44992,"ancestor_record_offset":43264,"inlinee":4360,"source_line":15,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":453,"record_offset":43652,"parent_record_offset":43612,"ancestor_record_offset":43264,"inlinee":4145,"source_line":226,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":453,"record_offset":45040,"parent_record_offset":45016,"ancestor_record_offset":43264,"inlinee":4640,"source_line":272,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::contains
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":453,"record_offset":43944,"parent_record_offset":43536,"ancestor_record_offset":43264,"inlinee":4848,"source_line":392,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::contains_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":453,"record_offset":44020,"parent_record_offset":43996,"ancestor_record_offset":43264,"inlinee":4851,"source_line":408,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::get_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":453,"record_offset":44044,"parent_record_offset":44020,"ancestor_record_offset":43264,"inlinee":4852,"source_line":171,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::impl$2::contains::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":453,"record_offset":43996,"parent_record_offset":43972,"ancestor_record_offset":43264,"inlinee":4850,"source_line":395,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":453,"record_offset":45808,"parent_record_offset":45772,"ancestor_record_offset":43264,"inlinee":4749,"source_line":166,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":453,"record_offset":45772,"parent_record_offset":45736,"ancestor_record_offset":43264,"inlinee":4748,"source_line":396,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_recip
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":453,"record_offset":45736,"parent_record_offset":45708,"ancestor_record_offset":43264,"inlinee":4747,"source_line":415,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nonmax","version":"0.5.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":453,"record_offset":44668,"parent_record_offset":44644,"ancestor_record_offset":43264,"inlinee":4171,"source_line":153,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":453,"record_offset":45844,"parent_record_offset":45808,"ancestor_record_offset":43264,"inlinee":4750,"source_line":369,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::cull_oob_decorators::check_if_dormer_ridge_is_within_roof_bounds::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/cull_oob_decorators.rs
//     evidence={"module":453,"record_offset":43488,"parent_record_offset":43460,"ancestor_record_offset":43264,"inlinee":25634,"source_line":1112,"archive":"_pdb/B/modules/00453.jsonl.gz"}
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
// 首条位置: {"module":453,"record_offset":29816,"rva":8451184,"va":5377160304}
// 编译实例 RVAs: 0x80f470
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: cull_oob_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2082677","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377160304,"raw_hex":"e88b38e0ff","size":5,"soft_fail":false,"target_va":5375081504,"va":5377164176,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140833180
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: cull_oob_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t143293","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377160304,"raw_hex":"e8bd2f0200","size":5,"soft_fail":false,"target_va":5377307008,"va":5377163710,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140833250
//     owner=unresolved_namespace; count=3
//     target_aliases=14 candidates; complete list: cull_oob_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t146291","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377160304,"raw_hex":"e8733b0200","size":5,"soft_fail":false,"target_va":5377307216,"va":5377160920,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408c5670
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: cull_oob_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t740470","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377160304,"raw_hex":"e8764c0b00","size":5,"soft_fail":false,"target_va":5377906288,"va":5377165813,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b25c90
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: cull_oob_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3237289","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377160304,"raw_hex":"e8a9653100","size":5,"soft_fail":false,"target_va":5380398224,"va":5377160930,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c91e40
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: cull_oob_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4727568","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377160304,"raw_hex":"e810234800","size":5,"soft_fail":false,"target_va":5381889600,"va":5377162027,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c91eb0
//     owner=unresolved_namespace; count=7
//     target_aliases=2 candidates; complete list: cull_oob_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4728608","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377160304,"raw_hex":"e820274800","size":5,"soft_fail":false,"target_va":5381889712,"va":5377161099,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c91f60
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: cull_oob_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4723979","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377160304,"raw_hex":"e80b154800","size":5,"soft_fail":false,"target_va":5381889888,"va":5377165904,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c9cde0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: cull_oob_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4772837","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377160304,"raw_hex":"e8e5d34800","size":5,"soft_fail":false,"target_va":5381934560,"va":5377161718,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14125cf90
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: cull_oob_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10803339","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377160304,"raw_hex":"e88bd8a400","size":5,"soft_fail":false,"target_va":5387964304,"va":5377160960,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141269fa0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: cull_oob_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10852185","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377160304,"raw_hex":"e85997a500","size":5,"soft_fail":false,"target_va":5388017568,"va":5377165378,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1412898e0
//     owner=unresolved_namespace; count=3
//     target_aliases=3 candidates; complete list: cull_oob_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10985699","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377160304,"raw_hex":"e8e3a0a700","size":5,"soft_fail":false,"target_va":5388146912,"va":5377161208,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14129d600
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: cull_oob_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11062864","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377160304,"raw_hex":"e850cea800","size":5,"soft_fail":false,"target_va":5388228096,"va":5377165227,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=5
//     evidence={"assembly":"\tcallq\t34379424","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377160304,"raw_hex":"e8a0960c02","size":5,"soft_fail":false,"target_va":5411541776,"va":5377162347,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t34377173","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377160304,"raw_hex":"e8d58d0c02","size":5,"soft_fail":false,"target_va":5411542064,"va":5377164886,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=8
//     evidence={"assembly":"\tcallq\t34374278","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377160304,"raw_hex":"e886820c02","size":5,"soft_fail":false,"target_va":5411542296,"va":5377168013,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t34374032","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377160304,"raw_hex":"e890810c02","size":5,"soft_fail":false,"target_va":5411541904,"va":5377167867,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t34375109","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377160304,"raw_hex":"e8c5850c02","size":5,"soft_fail":false,"target_va":5411542880,"va":5377167766,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::gothic_window_profile::gothic_half_dormer_profile_curve
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1657208","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377160304,"raw_hex":"e878491900","size":5,"soft_fail":false,"target_va":5378821312,"va":5377164099,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::roofs::roof_shape::Roof::gable_sides
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t864737","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377160304,"raw_hex":"e8e1310d00","size":5,"soft_fail":false,"target_va":5378028512,"va":5377163770,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::roofs::roof_shape::Roof::is_flat
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t864721","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377160304,"raw_hex":"e8d1310d00","size":5,"soft_fail":false,"target_va":5378028416,"va":5377163690,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::roofs::roof_shape::Roof::is_gable
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t864737","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377160304,"raw_hex":"e8e1310d00","size":5,"soft_fail":false,"target_va":5378028464,"va":5377163722,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms::transform
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2143407","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377160304,"raw_hex":"e8afb42000","size":5,"soft_fail":false,"target_va":5379303968,"va":5377160556,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator::DecoratorType::is_window
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t139684","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377160304,"raw_hex":"e8a4210200","size":5,"soft_fail":false,"target_va":5377306976,"va":5377167287,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator::DecoratorType::should_offer_snap_to_corner
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t141270","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377160304,"raw_hex":"e8d6270200","size":5,"soft_fail":false,"target_va":5377307072,"va":5377165797,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator_storage::DecoratorStorageAddr::as_wall
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2149562","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377160304,"raw_hex":"e8bacc2000","size":5,"soft_fail":false,"target_va":5379310144,"va":5377160577,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator_subtype::DecoratorSubtype::can_be_on_corner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3232278","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377160304,"raw_hex":"e816523100","size":5,"soft_fail":false,"target_va":5380397904,"va":5377165621,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo::subtype
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3240982","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377160304,"raw_hex":"e816743100","size":5,"soft_fail":false,"target_va":5380401520,"va":5377160533,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_wall_state::PublicWallState::is_rectangle
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3251768","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377160304,"raw_hex":"e8389e3100","size":5,"soft_fail":false,"target_va":5380417424,"va":5377165651,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_wall_state::PublicWallState::max_y
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3250119","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377160304,"raw_hex":"e8c7973100","size":5,"soft_fail":false,"target_va":5380417056,"va":5377166932,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_wall_state::PublicWallState::wall_space
//     owner=package_namespace_candidate; count=12
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3256295","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377160304,"raw_hex":"e8e7af3100","size":5,"soft_fail":false,"target_va":5380417344,"va":5377161044,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_walls::PublicWalls::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2631595","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377160304,"raw_hex":"e8ab272800","size":5,"soft_fail":false,"target_va":5379792624,"va":5377161024,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_walls::PublicWalls::roof_entity
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2632991","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377160304,"raw_hex":"e81f2d2800","size":5,"soft_fail":false,"target_va":5379793584,"va":5377160588,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::wall::valid_enclosure::ShapeParameters::as_rectangle
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t10414254","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377160304,"raw_hex":"e8aee89e00","size":5,"soft_fail":false,"target_va":5387581248,"va":5377166989,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_decorator::cull_oob_decorators::check_if_dormer_ridge_is_within_roof_bounds
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t6477","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377160304,"raw_hex":"e84d190000","size":5,"soft_fail":false,"target_va":5377168448,"va":5377161966,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_decorator::decorator_visual::cottage_dormers::cottage_dormer::is_out_of_roof_bounds
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-284662","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377160304,"raw_hex":"e80aa8fbff","size":5,"soft_fail":false,"target_va":5376878272,"va":5377162929,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_decorator::ui_systems::decorator_interaction_intent::snap_balcony_door
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t10938318","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377160304,"raw_hex":"e8cee7a600","size":5,"soft_fail":false,"target_va":5388103904,"va":5377165581,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::get_closest_edge_id
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t4759001","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377160304,"raw_hex":"e8d99d4800","size":5,"soft_fail":false,"target_va":5381926064,"va":5377167058,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::get_point_u
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t4754738","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377160304,"raw_hex":"e8328d4800","size":5,"soft_fail":false,"target_va":5381920464,"va":5377165721,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::wall_space::WallSpaceCoord::to_world_space
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t4728427","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377160304,"raw_hex":"e86b264800","size":5,"soft_fail":false,"target_va":5381890192,"va":5377161760,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::system::query::Query::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":453,"record_offset":30012,"parent_record_offset":29816,"ancestor_record_offset":29816,"inlinee":4837,"source_line":1385,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_direct] core::ops::range::Range::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":453,"record_offset":42812,"parent_record_offset":29816,"ancestor_record_offset":29816,"inlinee":7522,"source_line":120,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":453,"record_offset":42776,"parent_record_offset":29816,"ancestor_record_offset":29816,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":453,"record_offset":42940,"parent_record_offset":29816,"ancestor_record_offset":29816,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo::bbx_ls
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":453,"record_offset":40428,"parent_record_offset":29816,"ancestor_record_offset":29816,"inlinee":25099,"source_line":291,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::quat::Quat::mul_vec3
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/quat.rs
//     evidence={"module":453,"record_offset":41052,"parent_record_offset":29816,"ancestor_record_offset":29816,"inlinee":11318,"source_line":711,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::normalize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":453,"record_offset":42556,"parent_record_offset":29816,"ancestor_record_offset":29816,"inlinee":10290,"source_line":466,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::normalize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":453,"record_offset":41708,"parent_record_offset":29816,"ancestor_record_offset":29816,"inlinee":13657,"source_line":523,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":453,"record_offset":40736,"parent_record_offset":29816,"ancestor_record_offset":29816,"inlinee":4590,"source_line":1245,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":453,"record_offset":40792,"parent_record_offset":29816,"ancestor_record_offset":29816,"inlinee":4752,"source_line":1333,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::cull_oob_decorators::is_wall_decorator_location_valid
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/cull_oob_decorators.rs
//     evidence={"module":453,"record_offset":31664,"parent_record_offset":29816,"ancestor_record_offset":29816,"inlinee":25621,"source_line":707,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle3d::height
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":453,"record_offset":40708,"parent_record_offset":29816,"ancestor_record_offset":29816,"inlinee":10294,"source_line":79,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":453,"record_offset":35632,"parent_record_offset":35596,"ancestor_record_offset":29816,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":453,"record_offset":35596,"parent_record_offset":35448,"ancestor_record_offset":29816,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":453,"record_offset":30368,"parent_record_offset":30344,"ancestor_record_offset":29816,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":453,"record_offset":35500,"parent_record_offset":35448,"ancestor_record_offset":29816,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":453,"record_offset":35448,"parent_record_offset":35412,"ancestor_record_offset":29816,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":453,"record_offset":30416,"parent_record_offset":30392,"ancestor_record_offset":29816,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":453,"record_offset":30392,"parent_record_offset":30368,"ancestor_record_offset":29816,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":453,"record_offset":35412,"parent_record_offset":35380,"ancestor_record_offset":29816,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":453,"record_offset":30344,"parent_record_offset":30320,"ancestor_record_offset":29816,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":453,"record_offset":30320,"parent_record_offset":30220,"ancestor_record_offset":29816,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$13::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":453,"record_offset":33416,"parent_record_offset":33384,"ancestor_record_offset":29816,"inlinee":4358,"source_line":3357,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":453,"record_offset":30756,"parent_record_offset":30732,"ancestor_record_offset":29816,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::impl$25::eq
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":453,"record_offset":30520,"parent_record_offset":30188,"ancestor_record_offset":29816,"inlinee":4846,"source_line":76,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::Entities::get
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/entity/mod.rs
//     evidence={"module":453,"record_offset":30188,"parent_record_offset":30132,"ancestor_record_offset":29816,"inlinee":4840,"source_line":842,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::StorageSwitch::extract
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":453,"record_offset":31512,"parent_record_offset":31488,"ancestor_record_offset":29816,"inlinee":4876,"source_line":2472,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$26::set_archetype
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":453,"record_offset":30992,"parent_record_offset":30132,"ancestor_record_offset":29816,"inlinee":4856,"source_line":1160,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$26::set_table
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":453,"record_offset":31016,"parent_record_offset":30992,"ancestor_record_offset":29816,"inlinee":4857,"source_line":1175,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$27::fetch
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":453,"record_offset":31488,"parent_record_offset":30132,"ancestor_record_offset":29816,"inlinee":4874,"source_line":1229,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$27::fetch::closure$0
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":453,"record_offset":31536,"parent_record_offset":31512,"ancestor_record_offset":29816,"inlinee":4878,"source_line":1235,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_array::BlobArray::get_sub_slice
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_array.rs
//     evidence={"module":453,"record_offset":31436,"parent_record_offset":31412,"ancestor_record_offset":29816,"inlinee":4872,"source_line":143,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseArray::get
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":453,"record_offset":31116,"parent_record_offset":31092,"ancestor_record_offset":29816,"inlinee":4864,"source_line":58,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":453,"record_offset":31092,"parent_record_offset":31068,"ancestor_record_offset":29816,"inlinee":4862,"source_line":395,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$16::get::closure$0
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":453,"record_offset":31296,"parent_record_offset":31272,"ancestor_record_offset":29816,"inlinee":4866,"source_line":396,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_column
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":453,"record_offset":31068,"parent_record_offset":31040,"ancestor_record_offset":29816,"inlinee":4860,"source_line":475,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_data_slice_for
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":453,"record_offset":31040,"parent_record_offset":31016,"ancestor_record_offset":29816,"inlinee":4859,"source_line":363,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":453,"record_offset":30732,"parent_record_offset":30132,"ancestor_record_offset":29816,"inlinee":4854,"source_line":721,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::column::ThinColumn::get_data_slice
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/column.rs
//     evidence={"module":453,"record_offset":31412,"parent_record_offset":31388,"ancestor_record_offset":29816,"inlinee":4870,"source_line":303,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::impl$4::get_data_slice_for::closure$0
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":453,"record_offset":31388,"parent_record_offset":31364,"ancestor_record_offset":29816,"inlinee":4868,"source_line":368,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::as_readonly
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":453,"record_offset":30048,"parent_record_offset":30012,"ancestor_record_offset":29816,"inlinee":13705,"source_line":542,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::copy_unsafe
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":453,"record_offset":30096,"parent_record_offset":30072,"ancestor_record_offset":29816,"inlinee":13707,"source_line":636,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":453,"record_offset":36508,"parent_record_offset":36480,"ancestor_record_offset":29816,"inlinee":4837,"source_line":1385,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::get_inner
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":453,"record_offset":30132,"parent_record_offset":30012,"ancestor_record_offset":29816,"inlinee":4838,"source_line":1591,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::reborrow_unsafe
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":453,"record_offset":30072,"parent_record_offset":30048,"ancestor_record_offset":29816,"inlinee":13706,"source_line":618,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ptr::ThinSlicePtr::get
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ptr","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ptr-0.16.0/src/lib.rs
//     evidence={"module":453,"record_offset":31560,"parent_record_offset":31536,"ancestor_record_offset":29816,"inlinee":4880,"source_line":526,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::bool::impl$0::then
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\bool.rs
//     evidence={"module":453,"record_offset":30580,"parent_record_offset":30552,"ancestor_record_offset":29816,"inlinee":4781,"source_line":61,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":453,"record_offset":42868,"parent_record_offset":42840,"ancestor_record_offset":29816,"inlinee":4984,"source_line":1943,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$24::ne
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":453,"record_offset":30488,"parent_record_offset":30464,"ancestor_record_offset":29816,"inlinee":4844,"source_line":1742,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":453,"record_offset":42896,"parent_record_offset":42868,"ancestor_record_offset":29816,"inlinee":4986,"source_line":1788,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":453,"record_offset":41496,"parent_record_offset":41076,"ancestor_record_offset":29816,"inlinee":11327,"source_line":33,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ss
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":453,"record_offset":41404,"parent_record_offset":41348,"ancestor_record_offset":29816,"inlinee":11326,"source_line":21,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=17
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":453,"record_offset":41172,"parent_record_offset":41148,"ancestor_record_offset":29816,"inlinee":11323,"source_line":81,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":453,"record_offset":41204,"parent_record_offset":41120,"ancestor_record_offset":29816,"inlinee":11324,"source_line":1016,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_sub_ps
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":453,"record_offset":41268,"parent_record_offset":41076,"ancestor_record_offset":29816,"inlinee":11325,"source_line":57,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::abs
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":453,"record_offset":36204,"parent_record_offset":36052,"ancestor_record_offset":29816,"inlinee":10511,"source_line":1446,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":453,"record_offset":32764,"parent_record_offset":32484,"ancestor_record_offset":29816,"inlinee":4551,"source_line":890,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":453,"record_offset":32320,"parent_record_offset":32296,"ancestor_record_offset":29816,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":453,"record_offset":35552,"parent_record_offset":35500,"ancestor_record_offset":29816,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::impl$2::ne
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":453,"record_offset":30464,"parent_record_offset":30188,"ancestor_record_offset":29816,"inlinee":4842,"source_line":200,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":453,"record_offset":35052,"parent_record_offset":34684,"ancestor_record_offset":29816,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":453,"record_offset":33744,"parent_record_offset":33716,"ancestor_record_offset":29816,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":453,"record_offset":34056,"parent_record_offset":34028,"ancestor_record_offset":29816,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":453,"record_offset":31208,"parent_record_offset":31144,"ancestor_record_offset":29816,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":453,"record_offset":42840,"parent_record_offset":42812,"ancestor_record_offset":29816,"inlinee":4982,"source_line":817,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":453,"record_offset":36448,"parent_record_offset":31664,"ancestor_record_offset":29816,"inlinee":4518,"source_line":1447,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":453,"record_offset":31232,"parent_record_offset":31208,"ancestor_record_offset":29816,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":453,"record_offset":31272,"parent_record_offset":31092,"ancestor_record_offset":29816,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":453,"record_offset":32448,"parent_record_offset":31664,"ancestor_record_offset":29816,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":453,"record_offset":32264,"parent_record_offset":32236,"ancestor_record_offset":29816,"inlinee":4197,"source_line":1017,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":453,"record_offset":35348,"parent_record_offset":31664,"ancestor_record_offset":29816,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":453,"record_offset":30684,"parent_record_offset":30652,"ancestor_record_offset":29816,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":453,"record_offset":31628,"parent_record_offset":30132,"ancestor_record_offset":29816,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$45::slice_contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":453,"record_offset":38104,"parent_record_offset":38080,"ancestor_record_offset":29816,"inlinee":25630,"source_line":266,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$45::slice_contains::closure$1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":453,"record_offset":38152,"parent_record_offset":38128,"ancestor_record_offset":29816,"inlinee":25632,"source_line":278,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":453,"record_offset":38080,"parent_record_offset":31664,"ancestor_record_offset":29816,"inlinee":12923,"source_line":2551,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":453,"record_offset":30220,"parent_record_offset":30188,"ancestor_record_offset":29816,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":453,"record_offset":35692,"parent_record_offset":31664,"ancestor_record_offset":29816,"inlinee":4460,"source_line":1050,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::last
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":453,"record_offset":35252,"parent_record_offset":31664,"ancestor_record_offset":29816,"inlinee":4716,"source_line":279,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_noubcheck
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":453,"record_offset":30288,"parent_record_offset":30260,"ancestor_record_offset":29816,"inlinee":4164,"source_line":87,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":453,"record_offset":33448,"parent_record_offset":33416,"ancestor_record_offset":29816,"inlinee":4360,"source_line":15,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":453,"record_offset":30260,"parent_record_offset":30220,"ancestor_record_offset":29816,"inlinee":4145,"source_line":226,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":453,"record_offset":33480,"parent_record_offset":33448,"ancestor_record_offset":29816,"inlinee":4640,"source_line":272,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":453,"record_offset":35716,"parent_record_offset":35692,"ancestor_record_offset":29816,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":453,"record_offset":38128,"parent_record_offset":38104,"ancestor_record_offset":29816,"inlinee":4402,"source_line":279,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::DecoratorCollision::dims_wall_space
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":453,"record_offset":32356,"parent_record_offset":32236,"ancestor_record_offset":29816,"inlinee":25150,"source_line":118,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary::dims_wall_space
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":453,"record_offset":32236,"parent_record_offset":32192,"ancestor_record_offset":29816,"inlinee":25148,"source_line":241,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary::get_collision_bbx_ls
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":453,"record_offset":40464,"parent_record_offset":40428,"ancestor_record_offset":29816,"inlinee":25059,"source_line":223,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo::dims_wall_space
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":453,"record_offset":32192,"parent_record_offset":31664,"ancestor_record_offset":29816,"inlinee":25147,"source_line":285,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::impl$4::dims_wall_space::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":453,"record_offset":32296,"parent_record_offset":32264,"ancestor_record_offset":29816,"inlinee":25160,"source_line":244,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::impl$4::get_collision_bbx_ls::closure$0
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":453,"record_offset":40524,"parent_record_offset":40492,"ancestor_record_offset":29816,"inlinee":25042,"source_line":226,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::public_wall_state::PublicWallState::is_premade_shape
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_wall_state.rs
//     evidence={"module":453,"record_offset":36420,"parent_record_offset":31664,"ancestor_record_offset":29816,"inlinee":24411,"source_line":113,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::contains
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":453,"record_offset":30552,"parent_record_offset":30132,"ancestor_record_offset":29816,"inlinee":4848,"source_line":392,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::contains_unchecked
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":453,"record_offset":30628,"parent_record_offset":30604,"ancestor_record_offset":29816,"inlinee":4851,"source_line":408,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::get_unchecked
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":453,"record_offset":30652,"parent_record_offset":30628,"ancestor_record_offset":29816,"inlinee":4852,"source_line":171,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::impl$2::contains::closure$0
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":453,"record_offset":30604,"parent_record_offset":30580,"ancestor_record_offset":29816,"inlinee":4850,"source_line":395,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::mul_vec3a
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/quat.rs
//     evidence={"module":453,"record_offset":41076,"parent_record_offset":41052,"ancestor_record_offset":29816,"inlinee":11319,"source_line":788,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::cross
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec3a.rs
//     evidence={"module":453,"record_offset":41524,"parent_record_offset":41076,"ancestor_record_offset":29816,"inlinee":11329,"source_line":229,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":453,"record_offset":34488,"parent_record_offset":34456,"ancestor_record_offset":29816,"inlinee":4749,"source_line":166,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":453,"record_offset":34456,"parent_record_offset":34424,"ancestor_record_offset":29816,"inlinee":4748,"source_line":396,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_recip
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":453,"record_offset":34424,"parent_record_offset":34400,"ancestor_record_offset":29816,"inlinee":4747,"source_line":415,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::normalize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":453,"record_offset":34400,"parent_record_offset":31664,"ancestor_record_offset":29816,"inlinee":10290,"source_line":466,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::perp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":453,"record_offset":34372,"parent_record_offset":31664,"ancestor_record_offset":29816,"inlinee":4741,"source_line":891,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":453,"record_offset":34232,"parent_record_offset":34200,"ancestor_record_offset":29816,"inlinee":4895,"source_line":203,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":453,"record_offset":34200,"parent_record_offset":34168,"ancestor_record_offset":29816,"inlinee":4894,"source_line":451,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length_recip
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":453,"record_offset":34168,"parent_record_offset":34144,"ancestor_record_offset":29816,"inlinee":4893,"source_line":470,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":453,"record_offset":33688,"parent_record_offset":33660,"ancestor_record_offset":29816,"inlinee":24403,"source_line":759,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::normalize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":453,"record_offset":34144,"parent_record_offset":34120,"ancestor_record_offset":29816,"inlinee":13657,"source_line":523,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":453,"record_offset":33716,"parent_record_offset":33688,"ancestor_record_offset":29816,"inlinee":4590,"source_line":1245,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":453,"record_offset":34028,"parent_record_offset":33900,"ancestor_record_offset":29816,"inlinee":4743,"source_line":1473,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_in_x
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/sse2.rs
//     evidence={"module":453,"record_offset":41148,"parent_record_offset":41120,"ancestor_record_offset":29816,"inlinee":11322,"source_line":42,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_into_m128
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/sse2.rs
//     evidence={"module":453,"record_offset":41120,"parent_record_offset":41076,"ancestor_record_offset":29816,"inlinee":11321,"source_line":66,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"nonmax","version":"0.5.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":453,"record_offset":31320,"parent_record_offset":31296,"ancestor_record_offset":29816,"inlinee":4171,"source_line":153,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":453,"record_offset":34264,"parent_record_offset":34232,"ancestor_record_offset":29816,"inlinee":4750,"source_line":369,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::cull_oob_decorators::is_wall_decorator_location_valid::closure$0
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/cull_oob_decorators.rs
//     evidence={"module":453,"record_offset":32484,"parent_record_offset":31664,"ancestor_record_offset":29816,"inlinee":25623,"source_line":728,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::cull_oob_decorators::is_wall_decorator_location_valid::closure$3
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/cull_oob_decorators.rs
//     evidence={"module":453,"record_offset":36480,"parent_record_offset":36448,"ancestor_record_offset":29816,"inlinee":25628,"source_line":963,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::cull_oob_decorators::should_snap_to_corner
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/cull_oob_decorators.rs
//     evidence={"module":453,"record_offset":36052,"parent_record_offset":31664,"ancestor_record_offset":29816,"inlinee":25626,"source_line":1168,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::get_coord_at_2d_ws_distance
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":453,"record_offset":32552,"parent_record_offset":32484,"ancestor_record_offset":29816,"inlinee":25132,"source_line":1161,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::get_pos_at_coord
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":453,"record_offset":33384,"parent_record_offset":31664,"ancestor_record_offset":29816,"inlinee":9575,"source_line":799,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::get_tangent_at_coord
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":453,"record_offset":33872,"parent_record_offset":31664,"ancestor_record_offset":29816,"inlinee":24690,"source_line":816,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::get_unnormalized_tangent_at_coord
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":453,"record_offset":33900,"parent_record_offset":33872,"ancestor_record_offset":29816,"inlinee":24691,"source_line":812,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i1
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":453,"record_offset":33524,"parent_record_offset":33384,"ancestor_record_offset":29816,"inlinee":4641,"source_line":1337,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::CurveSlice::end
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":453,"record_offset":32732,"parent_record_offset":32552,"ancestor_record_offset":29816,"inlinee":24439,"source_line":338,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::CurveSlice::extend_right
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":453,"record_offset":32704,"parent_record_offset":32552,"ancestor_record_offset":29816,"inlinee":24465,"source_line":236,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::CurveSlice::from_curve_range
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":453,"record_offset":32588,"parent_record_offset":32552,"ancestor_record_offset":29816,"inlinee":25128,"source_line":89,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::CurveSlice::from_point_range
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":453,"record_offset":32616,"parent_record_offset":32588,"ancestor_record_offset":29816,"inlinee":24435,"source_line":110,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::CurveSlice::remap
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":453,"record_offset":32672,"parent_record_offset":32648,"ancestor_record_offset":29816,"inlinee":24464,"source_line":196,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::CurveSlice::remap_2d
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":453,"record_offset":32648,"parent_record_offset":32552,"ancestor_record_offset":29816,"inlinee":24463,"source_line":220,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::impl$12::sample_at_coord
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":453,"record_offset":34684,"parent_record_offset":31664,"ancestor_record_offset":29816,"inlinee":4637,"source_line":1392,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle3d::height
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":453,"record_offset":32408,"parent_record_offset":32356,"ancestor_record_offset":29816,"inlinee":10294,"source_line":79,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle3d::width
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":453,"record_offset":32380,"parent_record_offset":32356,"ancestor_record_offset":29816,"inlinee":10292,"source_line":84,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$1::lerp
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":453,"record_offset":33660,"parent_record_offset":33384,"ancestor_record_offset":29816,"inlinee":24442,"source_line":109,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$1::normalize
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":453,"record_offset":34120,"parent_record_offset":33872,"ancestor_record_offset":29816,"inlinee":24692,"source_line":99,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] utils::wall_space::WallSpaceCoord::new
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/wall_space.rs
//     evidence={"module":453,"record_offset":36024,"parent_record_offset":31664,"ancestor_record_offset":29816,"inlinee":4897,"source_line":116,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] utils::wall_space::WallSpaceCoord::subtract
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/wall_space.rs
//     evidence={"module":453,"record_offset":36176,"parent_record_offset":36052,"ancestor_record_offset":29816,"inlinee":25627,"source_line":284,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] utils::wall_space::WallSpaceCoord::with_offset
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/wall_space.rs
//     evidence={"module":453,"record_offset":34620,"parent_record_offset":31664,"ancestor_record_offset":29816,"inlinee":25625,"source_line":213,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] utils::wall_space::WallSpaceCoord::with_offset_clamped
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/wall_space.rs
//     evidence={"module":453,"record_offset":32520,"parent_record_offset":32484,"ancestor_record_offset":29816,"inlinee":25624,"source_line":257,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::cull_oob_decorators::auto_allow_decorator_culling
// 记录形式: compiled_procedure; instances=1
// 源码行: 48
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":453,"record_offset":2156,"rva":8442832,"va":5377151952}
// 编译实例 RVAs: 0x80d3d0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x141265530
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: cull_oob_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10846299","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377151952,"raw_hex":"e85b80a500","size":5,"soft_fail":false,"target_va":5387998512,"va":5377152208,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::history::History::is_recording
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2594172","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377151952,"raw_hex":"e87c952700","size":5,"soft_fail":false,"target_va":5379746160,"va":5377151983,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator_storage::DecoratorStorage::wall_decorator_ids
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2166326","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377151952,"raw_hex":"e8360e2100","size":5,"soft_fail":false,"target_va":5379318528,"va":5377152197,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] country_core::resources::walls::decorator_archivist::DecoratorArchivist::decorator_cull_allow_stairs
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tjmp\t3270653","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5377151952,"raw_hex":"e9fde73100","size":5,"soft_fail":false,"target_va":5380422960,"va":5377152302,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":453,"record_offset":3368,"parent_record_offset":2156,"ancestor_record_offset":2156,"inlinee":4457,"source_line":434,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::iterators::impl$0::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/iterators.rs
//     evidence={"module":453,"record_offset":3020,"parent_record_offset":2156,"ancestor_record_offset":2156,"inlinee":12537,"source_line":14,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::reader::EventReader::read
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/reader.rs
//     evidence={"module":453,"record_offset":2320,"parent_record_offset":2156,"ancestor_record_offset":2156,"inlinee":8034,"source_line":27,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator_archivist::DecoratorArchivist::decorator_cull_allow
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_archivist.rs
//     evidence={"module":453,"record_offset":3424,"parent_record_offset":2156,"ancestor_record_offset":2156,"inlinee":25603,"source_line":302,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":453,"record_offset":2664,"parent_record_offset":2640,"ancestor_record_offset":2156,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":453,"record_offset":2640,"parent_record_offset":2408,"ancestor_record_offset":2156,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":453,"record_offset":3392,"parent_record_offset":3368,"ancestor_record_offset":2156,"inlinee":4459,"source_line":397,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::event_cursor::EventCursor::read_with_id
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/event_cursor.rs
//     evidence={"module":453,"record_offset":2384,"parent_record_offset":2344,"ancestor_record_offset":2156,"inlinee":8037,"source_line":88,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::iterators::EventIteratorWithId::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/iterators.rs
//     evidence={"module":453,"record_offset":2408,"parent_record_offset":2384,"ancestor_record_offset":2156,"inlinee":8039,"source_line":54,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::iterators::impl$3::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/iterators.rs
//     evidence={"module":453,"record_offset":3048,"parent_record_offset":3020,"ancestor_record_offset":2156,"inlinee":12538,"source_line":86,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::system_param::impl$19::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/system_param.rs
//     evidence={"module":453,"record_offset":2344,"parent_record_offset":2320,"ancestor_record_offset":2156,"inlinee":10713,"source_line":1234,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::and_then_or_clear
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":453,"record_offset":3104,"parent_record_offset":3080,"ancestor_record_offset":2156,"inlinee":5541,"source_line":332,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":453,"record_offset":3080,"parent_record_offset":3048,"ancestor_record_offset":2156,"inlinee":4570,"source_line":83,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":453,"record_offset":3184,"parent_record_offset":3160,"ancestor_record_offset":2156,"inlinee":4573,"source_line":84,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":453,"record_offset":2596,"parent_record_offset":2568,"ancestor_record_offset":2156,"inlinee":4611,"source_line":653,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":453,"record_offset":2456,"parent_record_offset":2408,"ancestor_record_offset":2156,"inlinee":8040,"source_line":1859,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":453,"record_offset":3128,"parent_record_offset":3104,"ancestor_record_offset":2156,"inlinee":4130,"source_line":728,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":453,"record_offset":3160,"parent_record_offset":3080,"ancestor_record_offset":2156,"inlinee":4571,"source_line":1546,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":453,"record_offset":2912,"parent_record_offset":2888,"ancestor_record_offset":2156,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":453,"record_offset":3308,"parent_record_offset":3284,"ancestor_record_offset":2156,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":453,"record_offset":2512,"parent_record_offset":2408,"ancestor_record_offset":2156,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":453,"record_offset":2864,"parent_record_offset":2408,"ancestor_record_offset":2156,"inlinee":4460,"source_line":1050,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":453,"record_offset":2568,"parent_record_offset":2540,"ancestor_record_offset":2156,"inlinee":8042,"source_line":364,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$7::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":453,"record_offset":2540,"parent_record_offset":2512,"ancestor_record_offset":2156,"inlinee":8041,"source_line":537,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":453,"record_offset":2888,"parent_record_offset":2864,"ancestor_record_offset":2156,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":453,"record_offset":3284,"parent_record_offset":3260,"ancestor_record_offset":2156,"inlinee":4510,"source_line":99,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":453,"record_offset":3236,"parent_record_offset":3184,"ancestor_record_offset":2156,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$186::next_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":453,"record_offset":3260,"parent_record_offset":3236,"ancestor_record_offset":2156,"inlinee":4509,"source_line":440,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::impl$10::extend
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\set.rs
//     evidence={"module":453,"record_offset":3472,"parent_record_offset":3448,"ancestor_record_offset":2156,"inlinee":6932,"source_line":1307,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::impl$9::extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\set.rs
//     evidence={"module":453,"record_offset":3448,"parent_record_offset":3424,"ancestor_record_offset":2156,"inlinee":13716,"source_line":1117,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::cull_oob_decorators::cull_oob_decorators
// 记录形式: compiled_procedure; instances=1
// 源码行: 71
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":453,"record_offset":3568,"rva":8443200,"va":5377152320}
// 编译实例 RVAs: 0x80d540
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140149f50
//     owner=unresolved_namespace; count=2
//     target_aliases=35 candidates; complete list: cull_oob_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7096794","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377152320,"raw_hex":"e826b693ff","size":5,"soft_fail":false,"target_va":5370060624,"va":5377157413,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402ca2f0
//     owner=unresolved_namespace; count=3
//     target_aliases=5 candidates; complete list: cull_oob_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5519139","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377152320,"raw_hex":"e8ddc8abff","size":5,"soft_fail":false,"target_va":5371634416,"va":5377153550,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405b0d70
//     owner=unresolved_namespace; count=3
//     target_aliases=7 candidates; complete list: cull_oob_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2477310","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377152320,"raw_hex":"e80233daff","size":5,"soft_fail":false,"target_va":5374676336,"va":5377153641,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405b0ed0
//     owner=unresolved_namespace; count=2
//     target_aliases=6 candidates; complete list: cull_oob_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2478879","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377152320,"raw_hex":"e8e12cdaff","size":5,"soft_fail":false,"target_va":5374676688,"va":5377155562,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=5
//     target_aliases=1 candidates; complete list: cull_oob_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2074383","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377152320,"raw_hex":"e8f158e0ff","size":5,"soft_fail":false,"target_va":5375081504,"va":5377155882,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408bc520
//     owner=unresolved_namespace; count=2
//     target_aliases=4 candidates; complete list: cull_oob_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t713668","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377152320,"raw_hex":"e8c4e30a00","size":5,"soft_fail":false,"target_va":5377869088,"va":5377155415,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a1b100
//     owner=unresolved_namespace; count=1
//     target_aliases=5 candidates; complete list: cull_oob_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2152738","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377152320,"raw_hex":"e822d92000","size":5,"soft_fail":false,"target_va":5379305728,"va":5377152985,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a1e190
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: cull_oob_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2162965","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377152320,"raw_hex":"e815012100","size":5,"soft_fail":false,"target_va":5379318160,"va":5377155190,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b26a70
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: cull_oob_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3247141","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377152320,"raw_hex":"e8258c3100","size":5,"soft_fail":false,"target_va":5380401776,"va":5377154630,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b26a80
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: cull_oob_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3245957","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377152320,"raw_hex":"e885873100","size":5,"soft_fail":false,"target_va":5380401792,"va":5377155830,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1412593c0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: cull_oob_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10791082","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377152320,"raw_hex":"e8aaa8a400","size":5,"soft_fail":false,"target_va":5387948992,"va":5377157905,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141264200
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: cull_oob_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10840380","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377152320,"raw_hex":"e83c69a500","size":5,"soft_fail":false,"target_va":5387993600,"va":5377153215,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141264820
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: cull_oob_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10838520","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377152320,"raw_hex":"e8f861a500","size":5,"soft_fail":false,"target_va":5387995168,"va":5377156643,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141265710
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: cull_oob_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10842814","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377152320,"raw_hex":"e8be72a500","size":5,"soft_fail":false,"target_va":5387998992,"va":5377156173,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14129be00
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: cull_oob_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11066001","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377152320,"raw_hex":"e891daa800","size":5,"soft_fail":false,"target_va":5388221952,"va":5377155946,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1412b2a70
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: cull_oob_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11159184","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377152320,"raw_hex":"e89046aa00","size":5,"soft_fail":false,"target_va":5388315248,"va":5377156059,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::sync::Arc$LT$T$C$A$GT$::drop_slow
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t117600","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377152320,"raw_hex":"e860cb0100","size":5,"soft_fail":false,"target_va":5377274272,"va":5377156667,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t34384475","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377152320,"raw_hex":"e85baa0c02","size":5,"soft_fail":false,"target_va":5411541584,"va":5377157104,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t34384969","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377152320,"raw_hex":"e849ac0c02","size":5,"soft_fail":false,"target_va":5411542064,"va":5377157090,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-1732","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377152320,"raw_hex":"e83cf9ffff","size":5,"soft_fail":false,"target_va":5377150704,"va":5377152431,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t34390854","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377152320,"raw_hex":"e846c30c02","size":5,"soft_fail":false,"target_va":5411543360,"va":5377152501,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::history::History::is_recording
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2590176","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377152320,"raw_hex":"e8e0852700","size":5,"soft_fail":false,"target_va":5379746160,"va":5377155979,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms::position
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2147798","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377152320,"raw_hex":"e8d6c52000","size":5,"soft_fail":false,"target_va":5379303920,"va":5377156117,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator_archivist::DecoratorArchivist::is_decorator_cull_allowed
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3267683","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377152320,"raw_hex":"e863dc3100","size":5,"soft_fail":false,"target_va":5380423056,"va":5377155368,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t4232597","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377152320,"raw_hex":"e895954000","size":5,"soft_fail":false,"target_va":5381385536,"va":5377152934,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t4237786","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377152320,"raw_hex":"e8daa94000","size":5,"soft_fail":false,"target_va":5381390640,"va":5377152849,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::end_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t4229519","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377152320,"raw_hex":"e88f894000","size":5,"soft_fail":false,"target_va":5381386480,"va":5377156956,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t4232297","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377152320,"raw_hex":"e869944000","size":5,"soft_fail":false,"target_va":5381384704,"va":5377152402,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::hash::random::RandomState::new::KEYS::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t5504831","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377152320,"raw_hex":"e83fff5300","size":5,"soft_fail":false,"target_va":5382657856,"va":5377153020,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::random::hashmap_random_keys
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t5600737","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377152320,"raw_hex":"e8e1755500","size":5,"soft_fail":false,"target_va":5382753792,"va":5377153050,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::thread_local::native::lazy::Storage$LT$T$C$D$GT$::initialize
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t34487775","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377152320,"raw_hex":"e8df3d0e02","size":5,"soft_fail":false,"target_va":5411644800,"va":5377157020,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::LocalKey$LT$T$GT$::with
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t10785526","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377152320,"raw_hex":"e8f692a400","size":5,"soft_fail":false,"target_va":5387942848,"va":5377157317,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t34380033","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377152320,"raw_hex":"e801990c02","size":5,"soft_fail":false,"target_va":5411537104,"va":5377157066,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_decorator::apply_decorator_op::apply_decorator_op
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t10913617","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377152320,"raw_hex":"e85187a600","size":5,"soft_fail":false,"target_va":5388070448,"va":5377156826,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_decorator::cull_oob_decorators::is_decorator_location_valid
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t4785","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377152320,"raw_hex":"e8b1120000","size":5,"soft_fail":false,"target_va":5377160304,"va":5377155514,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":453,"record_offset":18464,"parent_record_offset":3568,"ancestor_record_offset":3568,"inlinee":4457,"source_line":434,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::iterators::impl$0::next
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/iterators.rs
//     evidence={"module":453,"record_offset":9212,"parent_record_offset":3568,"ancestor_record_offset":3568,"inlinee":12537,"source_line":14,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::reader::EventReader::read
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/reader.rs
//     evidence={"module":453,"record_offset":7384,"parent_record_offset":3568,"ancestor_record_offset":3568,"inlinee":8034,"source_line":27,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":453,"record_offset":17052,"parent_record_offset":3568,"ancestor_record_offset":3568,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":453,"record_offset":6268,"parent_record_offset":3568,"ancestor_record_offset":3568,"inlinee":4669,"source_line":1967,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::filter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":453,"record_offset":6212,"parent_record_offset":3568,"ancestor_record_offset":3568,"inlinee":23805,"source_line":866,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::and_then
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":453,"record_offset":18136,"parent_record_offset":3568,"ancestor_record_offset":3568,"inlinee":4518,"source_line":1447,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_none
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":453,"record_offset":18408,"parent_record_offset":3568,"ancestor_record_offset":3568,"inlinee":5657,"source_line":651,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":453,"record_offset":18576,"parent_record_offset":3568,"ancestor_record_offset":3568,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":453,"record_offset":13876,"parent_record_offset":3568,"ancestor_record_offset":3568,"inlinee":24514,"source_line":273,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":453,"record_offset":5244,"parent_record_offset":3568,"ancestor_record_offset":3568,"inlinee":4416,"source_line":589,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":453,"record_offset":3812,"parent_record_offset":3568,"ancestor_record_offset":3568,"inlinee":4368,"source_line":626,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":453,"record_offset":4268,"parent_record_offset":3568,"ancestor_record_offset":3568,"inlinee":4385,"source_line":659,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::contains_key
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":453,"record_offset":8092,"parent_record_offset":3568,"ancestor_record_offset":3568,"inlinee":19952,"source_line":1125,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::insert
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":453,"record_offset":18548,"parent_record_offset":3568,"ancestor_record_offset":3568,"inlinee":4947,"source_line":1189,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":453,"record_offset":16060,"parent_record_offset":3568,"ancestor_record_offset":3568,"inlinee":4949,"source_line":535,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":453,"record_offset":13932,"parent_record_offset":3568,"ancestor_record_offset":3568,"inlinee":7395,"source_line":249,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$46::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":453,"record_offset":18776,"parent_record_offset":3568,"ancestor_record_offset":3568,"inlinee":10297,"source_line":2382,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$47::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":453,"record_offset":16392,"parent_record_offset":3568,"ancestor_record_offset":3568,"inlinee":10640,"source_line":2392,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$54::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":453,"record_offset":19452,"parent_record_offset":3568,"ancestor_record_offset":3568,"inlinee":10299,"source_line":2474,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::set::HashSet::insert
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\set.rs
//     evidence={"module":453,"record_offset":9156,"parent_record_offset":3568,"ancestor_record_offset":3568,"inlinee":6056,"source_line":917,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::set::impl$19::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\set.rs
//     evidence={"module":453,"record_offset":14668,"parent_record_offset":3568,"ancestor_record_offset":3568,"inlinee":4801,"source_line":1545,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::set::impl$25::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\set.rs
//     evidence={"module":453,"record_offset":15260,"parent_record_offset":3568,"ancestor_record_offset":3568,"inlinee":4820,"source_line":1604,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::blueprint::DecoratorBlueprints::get
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/blueprint.rs
//     evidence={"module":453,"record_offset":17108,"parent_record_offset":3568,"ancestor_record_offset":3568,"inlinee":24932,"source_line":334,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::blueprint::DecoratorBlueprints::remove
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/blueprint.rs
//     evidence={"module":453,"record_offset":20216,"parent_record_offset":3568,"ancestor_record_offset":3568,"inlinee":25615,"source_line":307,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::cull_oob_decorators::DecoratorLocationCheckResult::is_valid
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/cull_oob_decorators.rs
//     evidence={"module":453,"record_offset":17080,"parent_record_offset":3568,"ancestor_record_offset":3568,"inlinee":25610,"source_line":493,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":453,"record_offset":18352,"parent_record_offset":18328,"ancestor_record_offset":3568,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":453,"record_offset":18328,"parent_record_offset":18304,"ancestor_record_offset":3568,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":453,"record_offset":18304,"parent_record_offset":18280,"ancestor_record_offset":3568,"inlinee":4364,"source_line":1676,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":453,"record_offset":18224,"parent_record_offset":18184,"ancestor_record_offset":3568,"inlinee":5345,"source_line":2205,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$37::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":453,"record_offset":20404,"parent_record_offset":20380,"ancestor_record_offset":3568,"inlinee":4774,"source_line":2550,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":453,"record_offset":7720,"parent_record_offset":7696,"ancestor_record_offset":3568,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":453,"record_offset":7696,"parent_record_offset":7472,"ancestor_record_offset":3568,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":453,"record_offset":18488,"parent_record_offset":18464,"ancestor_record_offset":3568,"inlinee":4459,"source_line":397,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::event_cursor::EventCursor::read_with_id
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/event_cursor.rs
//     evidence={"module":453,"record_offset":7448,"parent_record_offset":7408,"ancestor_record_offset":3568,"inlinee":8037,"source_line":88,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::iterators::EventIteratorWithId::new
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/iterators.rs
//     evidence={"module":453,"record_offset":7472,"parent_record_offset":7448,"ancestor_record_offset":3568,"inlinee":8039,"source_line":54,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::iterators::impl$3::next
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/iterators.rs
//     evidence={"module":453,"record_offset":9236,"parent_record_offset":9212,"ancestor_record_offset":3568,"inlinee":12538,"source_line":86,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::system_param::impl$19::deref_mut
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/system_param.rs
//     evidence={"module":453,"record_offset":7408,"parent_record_offset":7384,"ancestor_record_offset":3568,"inlinee":10713,"source_line":1234,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":453,"record_offset":5564,"parent_record_offset":5536,"ancestor_record_offset":3568,"inlinee":4432,"source_line":1817,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":453,"record_offset":6844,"parent_record_offset":6812,"ancestor_record_offset":3568,"inlinee":4798,"source_line":541,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":453,"record_offset":5616,"parent_record_offset":5592,"ancestor_record_offset":3568,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":453,"record_offset":5592,"parent_record_offset":5564,"ancestor_record_offset":3568,"inlinee":4434,"source_line":428,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":453,"record_offset":5512,"parent_record_offset":5484,"ancestor_record_offset":3568,"inlinee":4429,"source_line":1089,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":453,"record_offset":5536,"parent_record_offset":5512,"ancestor_record_offset":3568,"inlinee":4430,"source_line":1121,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":453,"record_offset":5708,"parent_record_offset":5536,"ancestor_record_offset":3568,"inlinee":4439,"source_line":2194,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$44::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":453,"record_offset":5820,"parent_record_offset":5796,"ancestor_record_offset":3568,"inlinee":4443,"source_line":1808,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":453,"record_offset":8824,"parent_record_offset":8800,"ancestor_record_offset":3568,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":453,"record_offset":13900,"parent_record_offset":13876,"ancestor_record_offset":3568,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":453,"record_offset":15708,"parent_record_offset":15684,"ancestor_record_offset":3568,"inlinee":4832,"source_line":1214,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":453,"record_offset":8332,"parent_record_offset":8308,"ancestor_record_offset":3568,"inlinee":4484,"source_line":1227,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":453,"record_offset":8416,"parent_record_offset":8392,"ancestor_record_offset":3568,"inlinee":4486,"source_line":1433,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":453,"record_offset":8356,"parent_record_offset":8332,"ancestor_record_offset":3568,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":453,"record_offset":8892,"parent_record_offset":8248,"ancestor_record_offset":3568,"inlinee":4502,"source_line":1497,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::and_then_or_clear
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":453,"record_offset":9288,"parent_record_offset":9264,"ancestor_record_offset":3568,"inlinee":5541,"source_line":332,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":453,"record_offset":9264,"parent_record_offset":9236,"ancestor_record_offset":3568,"inlinee":4570,"source_line":83,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next::closure$0
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":453,"record_offset":9368,"parent_record_offset":9344,"ancestor_record_offset":3568,"inlinee":4573,"source_line":84,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::cloned::impl$1::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\cloned.rs
//     evidence={"module":453,"record_offset":7184,"parent_record_offset":7160,"ancestor_record_offset":3568,"inlinee":9901,"source_line":57,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::filter::Filter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\filter.rs
//     evidence={"module":453,"record_offset":6236,"parent_record_offset":6212,"ancestor_record_offset":3568,"inlinee":23807,"source_line":27,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::filter::impl$3::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\filter.rs
//     evidence={"module":453,"record_offset":7160,"parent_record_offset":7136,"ancestor_record_offset":3568,"inlinee":10095,"source_line":155,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":453,"record_offset":7136,"parent_record_offset":7112,"ancestor_record_offset":3568,"inlinee":4681,"source_line":124,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":453,"record_offset":7112,"parent_record_offset":7088,"ancestor_record_offset":3568,"inlinee":4679,"source_line":790,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":453,"record_offset":5640,"parent_record_offset":5616,"ancestor_record_offset":3568,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":453,"record_offset":14940,"parent_record_offset":14912,"ancestor_record_offset":3568,"inlinee":4812,"source_line":486,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":453,"record_offset":19312,"parent_record_offset":19284,"ancestor_record_offset":3568,"inlinee":4263,"source_line":870,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":453,"record_offset":7652,"parent_record_offset":7624,"ancestor_record_offset":3568,"inlinee":4611,"source_line":653,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":453,"record_offset":19340,"parent_record_offset":19312,"ancestor_record_offset":3568,"inlinee":4619,"source_line":2579,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":453,"record_offset":7512,"parent_record_offset":7472,"ancestor_record_offset":3568,"inlinee":8040,"source_line":1859,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\mod.rs
//     evidence={"module":453,"record_offset":3968,"parent_record_offset":3936,"ancestor_record_offset":3568,"inlinee":4377,"source_line":1044,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":453,"record_offset":6872,"parent_record_offset":6812,"ancestor_record_offset":3568,"inlinee":6331,"source_line":1980,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":453,"record_offset":8564,"parent_record_offset":8536,"ancestor_record_offset":3568,"inlinee":4503,"source_line":449,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":453,"record_offset":8536,"parent_record_offset":8508,"ancestor_record_offset":3568,"inlinee":4493,"source_line":601,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":453,"record_offset":5380,"parent_record_offset":5340,"ancestor_record_offset":3568,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":453,"record_offset":9312,"parent_record_offset":9288,"ancestor_record_offset":3568,"inlinee":4130,"source_line":728,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":453,"record_offset":18432,"parent_record_offset":18408,"ancestor_record_offset":3568,"inlinee":4512,"source_line":607,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":453,"record_offset":18104,"parent_record_offset":17108,"ancestor_record_offset":3568,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or_else
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":453,"record_offset":9344,"parent_record_offset":9264,"ancestor_record_offset":3568,"inlinee":4571,"source_line":1546,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":453,"record_offset":6564,"parent_record_offset":6536,"ancestor_record_offset":3568,"inlinee":4197,"source_line":1017,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":453,"record_offset":4216,"parent_record_offset":4188,"ancestor_record_offset":3568,"inlinee":4384,"source_line":933,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":453,"record_offset":6168,"parent_record_offset":5340,"ancestor_record_offset":3568,"inlinee":4795,"source_line":296,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=24
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":453,"record_offset":5772,"parent_record_offset":5484,"ancestor_record_offset":3568,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":453,"record_offset":7984,"parent_record_offset":7960,"ancestor_record_offset":3568,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":453,"record_offset":15984,"parent_record_offset":15372,"ancestor_record_offset":3568,"inlinee":4834,"source_line":1354,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::replace
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":453,"record_offset":6652,"parent_record_offset":6536,"ancestor_record_offset":3568,"inlinee":7403,"source_line":1581,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":453,"record_offset":8684,"parent_record_offset":8660,"ancestor_record_offset":3568,"inlinee":4723,"source_line":1129,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":453,"record_offset":9492,"parent_record_offset":9468,"ancestor_record_offset":3568,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":453,"record_offset":4680,"parent_record_offset":4656,"ancestor_record_offset":3568,"inlinee":4410,"source_line":533,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":453,"record_offset":4656,"parent_record_offset":4632,"ancestor_record_offset":3568,"inlinee":4409,"source_line":686,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":453,"record_offset":4864,"parent_record_offset":4836,"ancestor_record_offset":3568,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":453,"record_offset":6728,"parent_record_offset":6700,"ancestor_record_offset":3568,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::replace
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":453,"record_offset":6676,"parent_record_offset":6652,"ancestor_record_offset":3568,"inlinee":7404,"source_line":1212,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":453,"record_offset":5664,"parent_record_offset":5640,"ancestor_record_offset":3568,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":453,"record_offset":7568,"parent_record_offset":7472,"ancestor_record_offset":3568,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":453,"record_offset":7936,"parent_record_offset":7472,"ancestor_record_offset":3568,"inlinee":4460,"source_line":1050,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::get
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":453,"record_offset":7624,"parent_record_offset":7596,"ancestor_record_offset":3568,"inlinee":8042,"source_line":364,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$7::get
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":453,"record_offset":7596,"parent_record_offset":7568,"ancestor_record_offset":3568,"inlinee":8041,"source_line":537,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":453,"record_offset":7960,"parent_record_offset":7936,"ancestor_record_offset":3568,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":453,"record_offset":4608,"parent_record_offset":4584,"ancestor_record_offset":3568,"inlinee":4407,"source_line":80,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":453,"record_offset":9468,"parent_record_offset":9444,"ancestor_record_offset":3568,"inlinee":4510,"source_line":99,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":453,"record_offset":4632,"parent_record_offset":4608,"ancestor_record_offset":3568,"inlinee":4408,"source_line":118,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":453,"record_offset":4936,"parent_record_offset":4908,"ancestor_record_offset":3568,"inlinee":4402,"source_line":279,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":453,"record_offset":9420,"parent_record_offset":9368,"ancestor_record_offset":3568,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$183::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":453,"record_offset":4584,"parent_record_offset":4468,"ancestor_record_offset":3568,"inlinee":4405,"source_line":392,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$186::next_unchecked
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":453,"record_offset":9444,"parent_record_offset":9420,"ancestor_record_offset":3568,"inlinee":4509,"source_line":440,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":453,"record_offset":3936,"parent_record_offset":3908,"ancestor_record_offset":3568,"inlinee":4375,"source_line":366,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":453,"record_offset":4296,"parent_record_offset":4268,"ancestor_record_offset":3568,"inlinee":4386,"source_line":1488,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":453,"record_offset":4436,"parent_record_offset":4404,"ancestor_record_offset":3568,"inlinee":4396,"source_line":128,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":453,"record_offset":4404,"parent_record_offset":4368,"ancestor_record_offset":3568,"inlinee":4394,"source_line":218,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":453,"record_offset":4344,"parent_record_offset":4320,"ancestor_record_offset":3568,"inlinee":4390,"source_line":315,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":453,"record_offset":4368,"parent_record_offset":4344,"ancestor_record_offset":3568,"inlinee":4392,"source_line":709,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":453,"record_offset":4320,"parent_record_offset":4296,"ancestor_record_offset":3568,"inlinee":4388,"source_line":798,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":453,"record_offset":4908,"parent_record_offset":4368,"ancestor_record_offset":3568,"inlinee":4400,"source_line":659,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":453,"record_offset":3908,"parent_record_offset":3876,"ancestor_record_offset":3568,"inlinee":4373,"source_line":433,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":453,"record_offset":3876,"parent_record_offset":3848,"ancestor_record_offset":3568,"inlinee":4372,"source_line":463,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":453,"record_offset":4068,"parent_record_offset":4036,"ancestor_record_offset":3568,"inlinee":4380,"source_line":501,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":453,"record_offset":4164,"parent_record_offset":4068,"ancestor_record_offset":3568,"inlinee":4381,"source_line":521,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":453,"record_offset":4036,"parent_record_offset":4012,"ancestor_record_offset":3568,"inlinee":4379,"source_line":533,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":453,"record_offset":3848,"parent_record_offset":3812,"ancestor_record_offset":3568,"inlinee":4370,"source_line":59,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":453,"record_offset":4188,"parent_record_offset":4164,"ancestor_record_offset":3568,"inlinee":4383,"source_line":189,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":453,"record_offset":4468,"parent_record_offset":4436,"ancestor_record_offset":3568,"inlinee":4398,"source_line":79,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":453,"record_offset":4752,"parent_record_offset":4468,"ancestor_record_offset":3568,"inlinee":4412,"source_line":17,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":453,"record_offset":4724,"parent_record_offset":4468,"ancestor_record_offset":3568,"inlinee":4411,"source_line":11,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":453,"record_offset":4528,"parent_record_offset":4468,"ancestor_record_offset":3568,"inlinee":4403,"source_line":24,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":453,"record_offset":20440,"parent_record_offset":20404,"ancestor_record_offset":3568,"inlinee":5814,"source_line":2986,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":453,"record_offset":20464,"parent_record_offset":20440,"ancestor_record_offset":3568,"inlinee":4778,"source_line":3777,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":453,"record_offset":20496,"parent_record_offset":20404,"ancestor_record_offset":3568,"inlinee":4779,"source_line":4075,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator::impl$29::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":453,"record_offset":17884,"parent_record_offset":17860,"ancestor_record_offset":3568,"inlinee":4664,"source_line":155,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::impl$9::eq
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":453,"record_offset":8848,"parent_record_offset":8824,"ancestor_record_offset":3568,"inlinee":4919,"source_line":69,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":453,"record_offset":8476,"parent_record_offset":8448,"ancestor_record_offset":3568,"inlinee":4490,"source_line":49,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":453,"record_offset":8508,"parent_record_offset":8476,"ancestor_record_offset":3568,"inlinee":4491,"source_line":74,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::remove_lowest_bit
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":453,"record_offset":15648,"parent_record_offset":15460,"ancestor_record_offset":3568,"inlinee":4830,"source_line":37,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":453,"record_offset":8448,"parent_record_offset":8248,"ancestor_record_offset":3568,"inlinee":4488,"source_line":112,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":453,"record_offset":8308,"parent_record_offset":8248,"ancestor_record_offset":3568,"inlinee":4482,"source_line":51,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load_aligned
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":453,"record_offset":15684,"parent_record_offset":15408,"ancestor_record_offset":3568,"inlinee":4831,"source_line":59,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":453,"record_offset":8920,"parent_record_offset":8248,"ancestor_record_offset":3568,"inlinee":4504,"source_line":94,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty_or_deleted
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":453,"record_offset":15136,"parent_record_offset":15112,"ancestor_record_offset":3568,"inlinee":4818,"source_line":101,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_full
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":453,"record_offset":15112,"parent_record_offset":15084,"ancestor_record_offset":3568,"inlinee":4817,"source_line":117,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=8
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":453,"record_offset":8392,"parent_record_offset":8248,"ancestor_record_offset":3568,"inlinee":4485,"source_line":76,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\tag.rs
//     evidence={"module":453,"record_offset":9076,"parent_record_offset":8248,"ancestor_record_offset":3568,"inlinee":4517,"source_line":36,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\lib.rs
//     evidence={"module":453,"record_offset":8800,"parent_record_offset":8776,"ancestor_record_offset":3568,"inlinee":4499,"source_line":164,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::contains_key
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":453,"record_offset":8124,"parent_record_offset":8092,"ancestor_record_offset":3568,"inlinee":4474,"source_line":1414,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":453,"record_offset":17156,"parent_record_offset":17132,"ancestor_record_offset":3568,"inlinee":4719,"source_line":1304,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":453,"record_offset":8156,"parent_record_offset":8124,"ancestor_record_offset":3568,"inlinee":4475,"source_line":1347,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":453,"record_offset":16084,"parent_record_offset":16060,"ancestor_record_offset":3568,"inlinee":4950,"source_line":754,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::remove
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":453,"record_offset":20264,"parent_record_offset":20240,"ancestor_record_offset":3568,"inlinee":5625,"source_line":1954,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::remove_entry
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":453,"record_offset":20288,"parent_record_offset":20264,"ancestor_record_offset":3568,"inlinee":5626,"source_line":1993,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":453,"record_offset":8776,"parent_record_offset":8608,"ancestor_record_offset":3568,"inlinee":4497,"source_line":224,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$42::into_iter
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":453,"record_offset":14748,"parent_record_offset":14708,"ancestor_record_offset":3568,"inlinee":4805,"source_line":3151,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$44::fold
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":453,"record_offset":7280,"parent_record_offset":7256,"ancestor_record_offset":3568,"inlinee":18642,"source_line":3186,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$44::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":453,"record_offset":16416,"parent_record_offset":16392,"ancestor_record_offset":3568,"inlinee":10642,"source_line":3171,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$53::next
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":453,"record_offset":15316,"parent_record_offset":15288,"ancestor_record_offset":3568,"inlinee":4824,"source_line":3275,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$82::extend
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":453,"record_offset":7088,"parent_record_offset":7064,"ancestor_record_offset":3568,"inlinee":6915,"source_line":4480,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":453,"record_offset":9116,"parent_record_offset":8156,"ancestor_record_offset":3568,"inlinee":4514,"source_line":253,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":453,"record_offset":8716,"parent_record_offset":8632,"ancestor_record_offset":3568,"inlinee":4722,"source_line":384,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":453,"record_offset":17728,"parent_record_offset":17704,"ancestor_record_offset":3568,"inlinee":4721,"source_line":522,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":453,"record_offset":8660,"parent_record_offset":8632,"ancestor_record_offset":3568,"inlinee":5616,"source_line":261,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::next_n
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":453,"record_offset":15840,"parent_record_offset":15408,"ancestor_record_offset":3568,"inlinee":4833,"source_line":439,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":453,"record_offset":9016,"parent_record_offset":8248,"ancestor_record_offset":3568,"inlinee":4506,"source_line":81,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::new
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":453,"record_offset":15084,"parent_record_offset":15060,"ancestor_record_offset":3568,"inlinee":4816,"source_line":3397,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::next_impl
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":453,"record_offset":15408,"parent_record_offset":15372,"ancestor_record_offset":3568,"inlinee":4829,"source_line":3464,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":453,"record_offset":8632,"parent_record_offset":8608,"ancestor_record_offset":3568,"inlinee":5615,"source_line":734,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":453,"record_offset":8220,"parent_record_offset":8192,"ancestor_record_offset":3568,"inlinee":4478,"source_line":1156,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":453,"record_offset":8192,"parent_record_offset":8156,"ancestor_record_offset":3568,"inlinee":4477,"source_line":1179,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::into_allocation
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":453,"record_offset":14868,"parent_record_offset":14832,"ancestor_record_offset":3568,"inlinee":4809,"source_line":1358,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::into_iter_from
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":453,"record_offset":14832,"parent_record_offset":14788,"ancestor_record_offset":3568,"inlinee":4808,"source_line":1344,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::iter
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":453,"record_offset":15036,"parent_record_offset":14788,"ancestor_record_offset":3568,"inlinee":4813,"source_line":1288,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::buckets
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":453,"record_offset":19160,"parent_record_offset":18936,"ancestor_record_offset":3568,"inlinee":4952,"source_line":2488,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::data_end
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":453,"record_offset":16164,"parent_record_offset":16132,"ancestor_record_offset":3568,"inlinee":4951,"source_line":2291,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":453,"record_offset":8248,"parent_record_offset":8220,"ancestor_record_offset":3568,"inlinee":4480,"source_line":1862,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":453,"record_offset":15060,"parent_record_offset":15036,"ancestor_record_offset":3568,"inlinee":4814,"source_line":1991,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":453,"record_offset":9048,"parent_record_offset":8248,"ancestor_record_offset":3568,"inlinee":4515,"source_line":2301,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::TableLayout::calculate_layout_for
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":453,"record_offset":14912,"parent_record_offset":14868,"ancestor_record_offset":3568,"inlinee":4811,"source_line":162,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$19::into_iter
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":453,"record_offset":14788,"parent_record_offset":14748,"ancestor_record_offset":3568,"inlinee":4807,"source_line":3343,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$29::fold
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":453,"record_offset":7304,"parent_record_offset":7280,"ancestor_record_offset":3568,"inlinee":18112,"source_line":3685,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$29::next
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":453,"record_offset":15372,"parent_record_offset":15344,"ancestor_record_offset":3568,"inlinee":4828,"source_line":3661,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$39::drop
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":453,"record_offset":18672,"parent_record_offset":18648,"ancestor_record_offset":3568,"inlinee":4904,"source_line":3846,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$41::next
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":453,"record_offset":15344,"parent_record_offset":15316,"ancestor_record_offset":3568,"inlinee":4826,"source_line":3887,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":453,"record_offset":8608,"parent_record_offset":8248,"ancestor_record_offset":3568,"inlinee":4495,"source_line":1166,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::HashSet::insert
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\set.rs
//     evidence={"module":453,"record_offset":9180,"parent_record_offset":9156,"ancestor_record_offset":3568,"inlinee":6057,"source_line":1092,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::impl$10::extend
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\set.rs
//     evidence={"module":453,"record_offset":7064,"parent_record_offset":7040,"ancestor_record_offset":3568,"inlinee":6932,"source_line":1307,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::impl$22::into_iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\set.rs
//     evidence={"module":453,"record_offset":14708,"parent_record_offset":14668,"ancestor_record_offset":3568,"inlinee":4803,"source_line":1774,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::impl$30::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\set.rs
//     evidence={"module":453,"record_offset":15288,"parent_record_offset":15260,"ancestor_record_offset":3568,"inlinee":4822,"source_line":1843,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":453,"record_offset":5276,"parent_record_offset":5244,"ancestor_record_offset":3568,"inlinee":4418,"source_line":425,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":453,"record_offset":20888,"parent_record_offset":20856,"ancestor_record_offset":3568,"inlinee":4966,"source_line":601,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":453,"record_offset":21388,"parent_record_offset":21128,"ancestor_record_offset":3568,"inlinee":4968,"source_line":602,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":453,"record_offset":5408,"parent_record_offset":5380,"ancestor_record_offset":3568,"inlinee":4423,"source_line":90,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":453,"record_offset":5484,"parent_record_offset":5340,"ancestor_record_offset":3568,"inlinee":4427,"source_line":429,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":453,"record_offset":5744,"parent_record_offset":5484,"ancestor_record_offset":3568,"inlinee":4441,"source_line":591,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":453,"record_offset":17132,"parent_record_offset":17108,"ancestor_record_offset":3568,"inlinee":4718,"source_line":894,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::remove
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":453,"record_offset":20240,"parent_record_offset":20216,"ancestor_record_offset":3568,"inlinee":8292,"source_line":1244,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$47::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":453,"record_offset":7256,"parent_record_offset":7232,"ancestor_record_offset":3568,"inlinee":18748,"source_line":2404,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$58::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":453,"record_offset":7232,"parent_record_offset":7208,"ancestor_record_offset":3568,"inlinee":25608,"source_line":2528,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$8::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":453,"record_offset":13956,"parent_record_offset":13932,"ancestor_record_offset":3568,"inlinee":7397,"source_line":1406,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::HashSet::with_hasher
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\set.rs
//     evidence={"module":453,"record_offset":7012,"parent_record_offset":6300,"ancestor_record_offset":3568,"inlinee":13714,"source_line":392,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::impl$7::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\set.rs
//     evidence={"module":453,"record_offset":6300,"parent_record_offset":6268,"ancestor_record_offset":3568,"inlinee":13713,"source_line":1067,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::impl$9::extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\set.rs
//     evidence={"module":453,"record_offset":7040,"parent_record_offset":6300,"ancestor_record_offset":3568,"inlinee":13716,"source_line":1117,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":453,"record_offset":6364,"parent_record_offset":6340,"ancestor_record_offset":3568,"inlinee":4794,"source_line":56,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::__init
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":453,"record_offset":6612,"parent_record_offset":6588,"ancestor_record_offset":3568,"inlinee":7402,"source_line":83,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::constant$0::closure$1
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":453,"record_offset":6476,"parent_record_offset":6448,"ancestor_record_offset":3568,"inlinee":7399,"source_line":97,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":453,"record_offset":6812,"parent_record_offset":6412,"ancestor_record_offset":3568,"inlinee":4797,"source_line":72,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":453,"record_offset":6340,"parent_record_offset":6300,"ancestor_record_offset":3568,"inlinee":4792,"source_line":151,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":453,"record_offset":5436,"parent_record_offset":5408,"ancestor_record_offset":3568,"inlinee":4425,"source_line":51,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::initialize
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":453,"record_offset":6536,"parent_record_offset":6504,"ancestor_record_offset":3568,"inlinee":7400,"source_line":61,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":453,"record_offset":5340,"parent_record_offset":5308,"ancestor_record_offset":3568,"inlinee":4421,"source_line":305,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":453,"record_offset":5308,"parent_record_offset":5276,"ancestor_record_offset":3568,"inlinee":4420,"source_line":270,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::blueprint::DecoratorBlueprintCollider::as_transformed_collider
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/blueprint.rs
//     evidence={"module":453,"record_offset":18184,"parent_record_offset":18160,"ancestor_record_offset":3568,"inlinee":25614,"source_line":131,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::cull_oob_decorators::cull_oob_decorators::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/cull_oob_decorators.rs
//     evidence={"module":453,"record_offset":18160,"parent_record_offset":18136,"ancestor_record_offset":3568,"inlinee":25612,"source_line":247,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::cull_oob_decorators::cull_oob_decorators::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 97
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1170A
// 首条位置: {"module":1504,"inlinee":71434}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] std::collections::hash::map::HashMap::contains_key
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":1504,"record_offset":51296,"parent_record_offset":51272,"ancestor_record_offset":51272,"inlinee":19952,"source_line":1125,"archive":"_pdb/B/modules/01504.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1504,"record_offset":52008,"parent_record_offset":51984,"ancestor_record_offset":51272,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/01504.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1504,"record_offset":51524,"parent_record_offset":51500,"ancestor_record_offset":51272,"inlinee":4484,"source_line":1227,"archive":"_pdb/B/modules/01504.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1504,"record_offset":51608,"parent_record_offset":51584,"ancestor_record_offset":51272,"inlinee":4486,"source_line":1433,"archive":"_pdb/B/modules/01504.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":1504,"record_offset":51548,"parent_record_offset":51524,"ancestor_record_offset":51272,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/01504.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":1504,"record_offset":52076,"parent_record_offset":51444,"ancestor_record_offset":51272,"inlinee":4502,"source_line":1497,"archive":"_pdb/B/modules/01504.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":1504,"record_offset":51756,"parent_record_offset":51728,"ancestor_record_offset":51272,"inlinee":4503,"source_line":449,"archive":"_pdb/B/modules/01504.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":1504,"record_offset":51728,"parent_record_offset":51700,"ancestor_record_offset":51272,"inlinee":4493,"source_line":601,"archive":"_pdb/B/modules/01504.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1504,"record_offset":51872,"parent_record_offset":51848,"ancestor_record_offset":51272,"inlinee":4723,"source_line":1129,"archive":"_pdb/B/modules/01504.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::impl$9::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":1504,"record_offset":52032,"parent_record_offset":52008,"ancestor_record_offset":51272,"inlinee":4919,"source_line":69,"archive":"_pdb/B/modules/01504.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/control/bitmask.rs
//     evidence={"module":1504,"record_offset":51668,"parent_record_offset":51640,"ancestor_record_offset":51272,"inlinee":4490,"source_line":49,"archive":"_pdb/B/modules/01504.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/control/bitmask.rs
//     evidence={"module":1504,"record_offset":51700,"parent_record_offset":51668,"ancestor_record_offset":51272,"inlinee":4491,"source_line":74,"archive":"_pdb/B/modules/01504.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/control/bitmask.rs
//     evidence={"module":1504,"record_offset":51640,"parent_record_offset":51444,"ancestor_record_offset":51272,"inlinee":4488,"source_line":112,"archive":"_pdb/B/modules/01504.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/control/group/sse2.rs
//     evidence={"module":1504,"record_offset":51500,"parent_record_offset":51444,"ancestor_record_offset":51272,"inlinee":4482,"source_line":51,"archive":"_pdb/B/modules/01504.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1504,"record_offset":52104,"parent_record_offset":51444,"ancestor_record_offset":51272,"inlinee":4504,"source_line":94,"archive":"_pdb/B/modules/01504.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1504,"record_offset":51584,"parent_record_offset":51444,"ancestor_record_offset":51272,"inlinee":4485,"source_line":76,"archive":"_pdb/B/modules/01504.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/control/tag.rs
//     evidence={"module":1504,"record_offset":52256,"parent_record_offset":51444,"ancestor_record_offset":51272,"inlinee":4517,"source_line":36,"archive":"_pdb/B/modules/01504.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\lib.rs
//     evidence={"module":1504,"record_offset":51984,"parent_record_offset":51964,"ancestor_record_offset":51272,"inlinee":4499,"source_line":164,"archive":"_pdb/B/modules/01504.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::contains_key
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1504,"record_offset":51324,"parent_record_offset":51296,"ancestor_record_offset":51272,"inlinee":4474,"source_line":1414,"archive":"_pdb/B/modules/01504.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1504,"record_offset":51352,"parent_record_offset":51324,"ancestor_record_offset":51272,"inlinee":4475,"source_line":1347,"archive":"_pdb/B/modules/01504.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1504,"record_offset":51964,"parent_record_offset":51800,"ancestor_record_offset":51272,"inlinee":4497,"source_line":224,"archive":"_pdb/B/modules/01504.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1504,"record_offset":52296,"parent_record_offset":51352,"ancestor_record_offset":51272,"inlinee":4514,"source_line":253,"archive":"_pdb/B/modules/01504.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1504,"record_offset":51904,"parent_record_offset":51820,"ancestor_record_offset":51272,"inlinee":4722,"source_line":384,"archive":"_pdb/B/modules/01504.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1504,"record_offset":51848,"parent_record_offset":51820,"ancestor_record_offset":51272,"inlinee":5616,"source_line":261,"archive":"_pdb/B/modules/01504.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":1504,"record_offset":52196,"parent_record_offset":51444,"ancestor_record_offset":51272,"inlinee":4506,"source_line":81,"archive":"_pdb/B/modules/01504.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1504,"record_offset":51820,"parent_record_offset":51800,"ancestor_record_offset":51272,"inlinee":5615,"source_line":734,"archive":"_pdb/B/modules/01504.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1504,"record_offset":51416,"parent_record_offset":51388,"ancestor_record_offset":51272,"inlinee":4478,"source_line":1156,"archive":"_pdb/B/modules/01504.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1504,"record_offset":51388,"parent_record_offset":51352,"ancestor_record_offset":51272,"inlinee":4477,"source_line":1179,"archive":"_pdb/B/modules/01504.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1504,"record_offset":51444,"parent_record_offset":51416,"ancestor_record_offset":51272,"inlinee":4480,"source_line":1862,"archive":"_pdb/B/modules/01504.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":1504,"record_offset":52228,"parent_record_offset":51444,"ancestor_record_offset":51272,"inlinee":4515,"source_line":2301,"archive":"_pdb/B/modules/01504.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1504,"record_offset":51800,"parent_record_offset":51444,"ancestor_record_offset":51272,"inlinee":4495,"source_line":1166,"archive":"_pdb/B/modules/01504.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::cull_oob_decorators::cull_oob_decorators::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 247
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x640C
// 首条位置: {"module":453,"inlinee":25612}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] system_decorator::blueprint::DecoratorBlueprintCollider::as_transformed_collider
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/blueprint.rs
//     evidence={"module":453,"record_offset":18184,"parent_record_offset":18160,"ancestor_record_offset":18160,"inlinee":25614,"source_line":131,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":453,"record_offset":18352,"parent_record_offset":18328,"ancestor_record_offset":18160,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":453,"record_offset":18328,"parent_record_offset":18304,"ancestor_record_offset":18160,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":453,"record_offset":18304,"parent_record_offset":18280,"ancestor_record_offset":18160,"inlinee":4364,"source_line":1676,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":453,"record_offset":18224,"parent_record_offset":18184,"ancestor_record_offset":18160,"inlinee":5345,"source_line":2205,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":453,"record_offset":18252,"parent_record_offset":18184,"ancestor_record_offset":18160,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::cull_oob_decorators::cull_stair_attached_decorators_impl<system_decorator::cull_oob_decorators::cull_stair_attached_decorators::closure_env$0>
// 记录形式: compiled_procedure; instances=1
// 源码行: 354
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1570,"record_offset":6396,"rva":19622960,"va":5388332080}
// 编译实例 RVAs: 0x12b6c30
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1405b0ed0
//     owner=unresolved_namespace; count=1
//     target_aliases=6 candidates; complete list: cull_oob_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13656867","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388332080,"raw_hex":"e8dd9c2fff","size":5,"soft_fail":false,"target_va":5374676688,"va":5388333550,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: cull_oob_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13252300","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388332080,"raw_hex":"e834c935ff","size":5,"soft_fail":false,"target_va":5375081504,"va":5388333799,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1412593c0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: cull_oob_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-385093","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388332080,"raw_hex":"e8bb1ffaff","size":5,"soft_fail":false,"target_va":5387948992,"va":5388334080,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141264820
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: cull_oob_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-338401","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388332080,"raw_hex":"e81fd6faff","size":5,"soft_fail":false,"target_va":5387995168,"va":5388333564,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141265710
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: cull_oob_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-333512","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388332080,"raw_hex":"e838e9faff","size":5,"soft_fail":false,"target_va":5387998992,"va":5388332499,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::sync::Arc$LT$T$C$A$GT$::drop_slow
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-11059321","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388332080,"raw_hex":"e8873f57ff","size":5,"soft_fail":false,"target_va":5377274272,"va":5388333588,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t23208202","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388332080,"raw_hex":"e80a216201","size":5,"soft_fail":false,"target_va":5411542064,"va":5388333857,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::history::History::is_recording
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-8587461","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388332080,"raw_hex":"e83bf77cff","size":5,"soft_fail":false,"target_va":5379746160,"va":5388333616,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::stairs::stairs_state::StairsState::contains_edge_or_stub
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-10182987","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388332080,"raw_hex":"e8b59e64ff","size":5,"soft_fail":false,"target_va":5378150016,"va":5388332998,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms::try_position
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-9028736","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388332080,"raw_hex":"e8803b76ff","size":5,"soft_fail":false,"target_va":5379304320,"va":5388333051,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator::DecoratorInfo::dst_as_stair
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-11025218","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388332080,"raw_hex":"e8bec457ff","size":5,"soft_fail":false,"target_va":5377307760,"va":5388332973,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator_archivist::DecoratorArchivist::is_decorator_cull_allowed
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-7909887","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388332080,"raw_hex":"e8014e87ff","size":5,"soft_fail":false,"target_va":5380423056,"va":5388332938,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator_storage::DecoratorStorage::iter_stair_decorators
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-9013819","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388332080,"raw_hex":"e8c57576ff","size":5,"soft_fail":false,"target_va":5379318464,"va":5388332278,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::hash::random::RandomState::new::KEYS::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-5674299","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388332080,"raw_hex":"e8c56aa9ff","size":5,"soft_fail":false,"target_va":5382657856,"va":5388332150,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::random::hashmap_random_keys
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-5578400","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388332080,"raw_hex":"e860e1aaff","size":5,"soft_fail":false,"target_va":5382753792,"va":5388332187,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_decorator::apply_decorator_op::apply_decorator_op
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-263297","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388332080,"raw_hex":"e87ffbfbff","size":5,"soft_fail":false,"target_va":5388070448,"va":5388333740,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":1570,"record_offset":11416,"parent_record_offset":6396,"ancestor_record_offset":6396,"inlinee":4457,"source_line":434,"archive":"_pdb/B/modules/01570.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::flatten::impl$3::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\flatten.rs
//     evidence={"module":1570,"record_offset":7476,"parent_record_offset":6396,"ancestor_record_offset":6396,"inlinee":11831,"source_line":63,"archive":"_pdb/B/modules/01570.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\collect.rs
//     evidence={"module":1570,"record_offset":7388,"parent_record_offset":6396,"ancestor_record_offset":6396,"inlinee":4756,"source_line":322,"archive":"_pdb/B/modules/01570.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1570,"record_offset":7416,"parent_record_offset":6396,"ancestor_record_offset":6396,"inlinee":4666,"source_line":998,"archive":"_pdb/B/modules/01570.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1570,"record_offset":11908,"parent_record_offset":6396,"ancestor_record_offset":6396,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01570.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::insert
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":1570,"record_offset":7448,"parent_record_offset":6396,"ancestor_record_offset":6396,"inlinee":4947,"source_line":1189,"archive":"_pdb/B/modules/01570.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":1570,"record_offset":6680,"parent_record_offset":6396,"ancestor_record_offset":6396,"inlinee":7395,"source_line":249,"archive":"_pdb/B/modules/01570.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$46::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":1570,"record_offset":10032,"parent_record_offset":6396,"ancestor_record_offset":6396,"inlinee":10297,"source_line":2382,"archive":"_pdb/B/modules/01570.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$54::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":1570,"record_offset":10708,"parent_record_offset":6396,"ancestor_record_offset":6396,"inlinee":10299,"source_line":2474,"archive":"_pdb/B/modules/01570.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::blueprint::DecoratorBlueprints::remove
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/blueprint.rs
//     evidence={"module":1570,"record_offset":11472,"parent_record_offset":6396,"ancestor_record_offset":6396,"inlinee":25615,"source_line":307,"archive":"_pdb/B/modules/01570.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::cull_oob_decorators::cull_stair_attached_decorators::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/cull_oob_decorators.rs
//     evidence={"module":1570,"record_offset":10004,"parent_record_offset":6396,"ancestor_record_offset":6396,"inlinee":72051,"source_line":448,"archive":"_pdb/B/modules/01570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1570,"record_offset":12032,"parent_record_offset":12008,"ancestor_record_offset":6396,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1570,"record_offset":12008,"parent_record_offset":11980,"ancestor_record_offset":6396,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$37::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":1570,"record_offset":11660,"parent_record_offset":11636,"ancestor_record_offset":6396,"inlinee":4774,"source_line":2550,"archive":"_pdb/B/modules/01570.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":1570,"record_offset":11440,"parent_record_offset":11416,"ancestor_record_offset":6396,"inlinee":4459,"source_line":397,"archive":"_pdb/B/modules/01570.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1570,"record_offset":7212,"parent_record_offset":7184,"ancestor_record_offset":6396,"inlinee":4798,"source_line":541,"archive":"_pdb/B/modules/01570.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1570,"record_offset":7272,"parent_record_offset":7240,"ancestor_record_offset":6396,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/01570.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1570,"record_offset":8112,"parent_record_offset":8088,"ancestor_record_offset":6396,"inlinee":4832,"source_line":1214,"archive":"_pdb/B/modules/01570.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1570,"record_offset":8208,"parent_record_offset":8172,"ancestor_record_offset":6396,"inlinee":4486,"source_line":1433,"archive":"_pdb/B/modules/01570.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::flatten::and_then_or_clear
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\flatten.rs
//     evidence={"module":1570,"record_offset":7572,"parent_record_offset":7512,"ancestor_record_offset":6396,"inlinee":11087,"source_line":807,"archive":"_pdb/B/modules/01570.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::flatten::impl$33::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\flatten.rs
//     evidence={"module":1570,"record_offset":7512,"parent_record_offset":7476,"ancestor_record_offset":6396,"inlinee":11085,"source_line":573,"archive":"_pdb/B/modules/01570.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::fuse::impl$11::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\fuse.rs
//     evidence={"module":1570,"record_offset":8384,"parent_record_offset":8356,"ancestor_record_offset":6396,"inlinee":6982,"source_line":358,"archive":"_pdb/B/modules/01570.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::fuse::impl$3::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\fuse.rs
//     evidence={"module":1570,"record_offset":8356,"parent_record_offset":7512,"ancestor_record_offset":6396,"inlinee":6980,"source_line":49,"archive":"_pdb/B/modules/01570.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":1570,"record_offset":7668,"parent_record_offset":7636,"ancestor_record_offset":6396,"inlinee":6061,"source_line":106,"archive":"_pdb/B/modules/01570.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":1570,"record_offset":7076,"parent_record_offset":7052,"ancestor_record_offset":6396,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/01570.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1570,"record_offset":10628,"parent_record_offset":10540,"ancestor_record_offset":6396,"inlinee":4812,"source_line":486,"archive":"_pdb/B/modules/01570.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1570,"record_offset":10568,"parent_record_offset":10540,"ancestor_record_offset":6396,"inlinee":4263,"source_line":870,"archive":"_pdb/B/modules/01570.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1570,"record_offset":10596,"parent_record_offset":10568,"ancestor_record_offset":6396,"inlinee":4619,"source_line":2579,"archive":"_pdb/B/modules/01570.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1570,"record_offset":7240,"parent_record_offset":7184,"ancestor_record_offset":6396,"inlinee":6331,"source_line":1980,"archive":"_pdb/B/modules/01570.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":1570,"record_offset":8008,"parent_record_offset":7980,"ancestor_record_offset":6396,"inlinee":4503,"source_line":449,"archive":"_pdb/B/modules/01570.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":1570,"record_offset":7980,"parent_record_offset":7952,"ancestor_record_offset":6396,"inlinee":4493,"source_line":601,"archive":"_pdb/B/modules/01570.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":1570,"record_offset":6840,"parent_record_offset":6808,"ancestor_record_offset":6396,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/01570.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":1570,"record_offset":8472,"parent_record_offset":8440,"ancestor_record_offset":6396,"inlinee":5024,"source_line":304,"archive":"_pdb/B/modules/01570.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1570,"record_offset":7604,"parent_record_offset":7572,"ancestor_record_offset":6396,"inlinee":4130,"source_line":728,"archive":"_pdb/B/modules/01570.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1570,"record_offset":8440,"parent_record_offset":8412,"ancestor_record_offset":6396,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/01570.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1570,"record_offset":6944,"parent_record_offset":6920,"ancestor_record_offset":6396,"inlinee":4197,"source_line":1017,"archive":"_pdb/B/modules/01570.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1570,"record_offset":8300,"parent_record_offset":7816,"ancestor_record_offset":6396,"inlinee":4384,"source_line":933,"archive":"_pdb/B/modules/01570.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1570,"record_offset":11612,"parent_record_offset":11472,"ancestor_record_offset":6396,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01570.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1570,"record_offset":7028,"parent_record_offset":6920,"ancestor_record_offset":6396,"inlinee":7403,"source_line":1581,"archive":"_pdb/B/modules/01570.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1570,"record_offset":8268,"parent_record_offset":8244,"ancestor_record_offset":6396,"inlinee":4723,"source_line":1129,"archive":"_pdb/B/modules/01570.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1570,"record_offset":7100,"parent_record_offset":7076,"ancestor_record_offset":6396,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/01570.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1570,"record_offset":7052,"parent_record_offset":7028,"ancestor_record_offset":6396,"inlinee":7404,"source_line":1212,"archive":"_pdb/B/modules/01570.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1570,"record_offset":7128,"parent_record_offset":7076,"ancestor_record_offset":6396,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/01570.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1570,"record_offset":11696,"parent_record_offset":11660,"ancestor_record_offset":6396,"inlinee":5814,"source_line":2986,"archive":"_pdb/B/modules/01570.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1570,"record_offset":11720,"parent_record_offset":11696,"ancestor_record_offset":6396,"inlinee":4778,"source_line":3777,"archive":"_pdb/B/modules/01570.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1570,"record_offset":11752,"parent_record_offset":11660,"ancestor_record_offset":6396,"inlinee":4779,"source_line":4075,"archive":"_pdb/B/modules/01570.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_storage::impl$17::iter_stair_decorators::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_storage.rs
//     evidence={"module":1570,"record_offset":8496,"parent_record_offset":8472,"ancestor_record_offset":6396,"inlinee":71339,"source_line":718,"archive":"_pdb/B/modules/01570.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":1570,"record_offset":7912,"parent_record_offset":7868,"ancestor_record_offset":6396,"inlinee":4490,"source_line":49,"archive":"_pdb/B/modules/01570.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":1570,"record_offset":7952,"parent_record_offset":7912,"ancestor_record_offset":6396,"inlinee":4491,"source_line":74,"archive":"_pdb/B/modules/01570.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::remove_lowest_bit
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":1570,"record_offset":8052,"parent_record_offset":7868,"ancestor_record_offset":6396,"inlinee":4830,"source_line":37,"archive":"_pdb/B/modules/01570.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":1570,"record_offset":7868,"parent_record_offset":7816,"ancestor_record_offset":6396,"inlinee":4488,"source_line":112,"archive":"_pdb/B/modules/01570.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load_aligned
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1570,"record_offset":8088,"parent_record_offset":7816,"ancestor_record_offset":6396,"inlinee":4831,"source_line":59,"archive":"_pdb/B/modules/01570.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty_or_deleted
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1570,"record_offset":8172,"parent_record_offset":8144,"ancestor_record_offset":6396,"inlinee":4818,"source_line":101,"archive":"_pdb/B/modules/01570.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_full
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1570,"record_offset":8144,"parent_record_offset":7816,"ancestor_record_offset":6396,"inlinee":4817,"source_line":117,"archive":"_pdb/B/modules/01570.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1570,"record_offset":8544,"parent_record_offset":8520,"ancestor_record_offset":6396,"inlinee":4950,"source_line":754,"archive":"_pdb/B/modules/01570.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::remove
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1570,"record_offset":11520,"parent_record_offset":11496,"ancestor_record_offset":6396,"inlinee":5625,"source_line":1954,"archive":"_pdb/B/modules/01570.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::remove_entry
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1570,"record_offset":11544,"parent_record_offset":11520,"ancestor_record_offset":6396,"inlinee":5626,"source_line":1993,"archive":"_pdb/B/modules/01570.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$42::into_iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1570,"record_offset":10072,"parent_record_offset":10032,"ancestor_record_offset":6396,"inlinee":4805,"source_line":3151,"archive":"_pdb/B/modules/01570.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$44::next
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1570,"record_offset":7732,"parent_record_offset":7700,"ancestor_record_offset":6396,"inlinee":10642,"source_line":3171,"archive":"_pdb/B/modules/01570.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$53::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1570,"record_offset":10736,"parent_record_offset":10708,"ancestor_record_offset":6396,"inlinee":4824,"source_line":3275,"archive":"_pdb/B/modules/01570.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1570,"record_offset":11572,"parent_record_offset":11544,"ancestor_record_offset":6396,"inlinee":4514,"source_line":253,"archive":"_pdb/B/modules/01570.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::next_n
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1570,"record_offset":8244,"parent_record_offset":7816,"ancestor_record_offset":6396,"inlinee":4833,"source_line":439,"archive":"_pdb/B/modules/01570.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::new
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1570,"record_offset":8652,"parent_record_offset":8592,"ancestor_record_offset":6396,"inlinee":4816,"source_line":3397,"archive":"_pdb/B/modules/01570.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::next_impl
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1570,"record_offset":7816,"parent_record_offset":7764,"ancestor_record_offset":6396,"inlinee":4829,"source_line":3464,"archive":"_pdb/B/modules/01570.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::into_allocation
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1570,"record_offset":10492,"parent_record_offset":10452,"ancestor_record_offset":6396,"inlinee":4809,"source_line":1358,"archive":"_pdb/B/modules/01570.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::into_iter_from
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1570,"record_offset":10452,"parent_record_offset":10112,"ancestor_record_offset":6396,"inlinee":4808,"source_line":1344,"archive":"_pdb/B/modules/01570.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::iter
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1570,"record_offset":8568,"parent_record_offset":8544,"ancestor_record_offset":6396,"inlinee":4813,"source_line":1288,"archive":"_pdb/B/modules/01570.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::buckets
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1570,"record_offset":10416,"parent_record_offset":10192,"ancestor_record_offset":6396,"inlinee":4952,"source_line":2488,"archive":"_pdb/B/modules/01570.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::data_end
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1570,"record_offset":8624,"parent_record_offset":8592,"ancestor_record_offset":6396,"inlinee":4951,"source_line":2291,"archive":"_pdb/B/modules/01570.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1570,"record_offset":8592,"parent_record_offset":8568,"ancestor_record_offset":6396,"inlinee":4814,"source_line":1991,"archive":"_pdb/B/modules/01570.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::TableLayout::calculate_layout_for
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1570,"record_offset":10540,"parent_record_offset":10492,"ancestor_record_offset":6396,"inlinee":4811,"source_line":162,"archive":"_pdb/B/modules/01570.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$19::into_iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1570,"record_offset":10112,"parent_record_offset":10072,"ancestor_record_offset":6396,"inlinee":4807,"source_line":3343,"archive":"_pdb/B/modules/01570.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$29::next
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1570,"record_offset":7764,"parent_record_offset":7732,"ancestor_record_offset":6396,"inlinee":4828,"source_line":3661,"archive":"_pdb/B/modules/01570.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$39::drop
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1570,"record_offset":11980,"parent_record_offset":11956,"ancestor_record_offset":6396,"inlinee":4904,"source_line":3846,"archive":"_pdb/B/modules/01570.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$41::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1570,"record_offset":10764,"parent_record_offset":10736,"ancestor_record_offset":6396,"inlinee":4826,"source_line":3887,"archive":"_pdb/B/modules/01570.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":1570,"record_offset":8520,"parent_record_offset":8496,"ancestor_record_offset":6396,"inlinee":4949,"source_line":535,"archive":"_pdb/B/modules/01570.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::remove
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":1570,"record_offset":11496,"parent_record_offset":11472,"ancestor_record_offset":6396,"inlinee":8292,"source_line":1244,"archive":"_pdb/B/modules/01570.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$47::next
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":1570,"record_offset":7700,"parent_record_offset":7668,"ancestor_record_offset":6396,"inlinee":10640,"source_line":2392,"archive":"_pdb/B/modules/01570.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$8::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":1570,"record_offset":6704,"parent_record_offset":6680,"ancestor_record_offset":6396,"inlinee":7397,"source_line":1406,"archive":"_pdb/B/modules/01570.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":1570,"record_offset":6760,"parent_record_offset":6728,"ancestor_record_offset":6396,"inlinee":4794,"source_line":56,"archive":"_pdb/B/modules/01570.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::__init
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":1570,"record_offset":6988,"parent_record_offset":6968,"ancestor_record_offset":6396,"inlinee":7402,"source_line":83,"archive":"_pdb/B/modules/01570.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::constant$0::closure$1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":1570,"record_offset":6864,"parent_record_offset":6840,"ancestor_record_offset":6396,"inlinee":7399,"source_line":97,"archive":"_pdb/B/modules/01570.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":1570,"record_offset":7184,"parent_record_offset":6808,"ancestor_record_offset":6396,"inlinee":4797,"source_line":72,"archive":"_pdb/B/modules/01570.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":1570,"record_offset":6728,"parent_record_offset":6704,"ancestor_record_offset":6396,"inlinee":4792,"source_line":151,"archive":"_pdb/B/modules/01570.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":1570,"record_offset":6892,"parent_record_offset":6864,"ancestor_record_offset":6396,"inlinee":4425,"source_line":51,"archive":"_pdb/B/modules/01570.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::initialize
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":1570,"record_offset":6920,"parent_record_offset":6892,"ancestor_record_offset":6396,"inlinee":7400,"source_line":61,"archive":"_pdb/B/modules/01570.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":1570,"record_offset":6808,"parent_record_offset":6784,"ancestor_record_offset":6396,"inlinee":4421,"source_line":305,"archive":"_pdb/B/modules/01570.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":1570,"record_offset":6784,"parent_record_offset":6760,"ancestor_record_offset":6396,"inlinee":4420,"source_line":270,"archive":"_pdb/B/modules/01570.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::cull_oob_decorators::cull_stair_attached_decorators_impl<system_decorator::cull_oob_decorators::cull_stair_attached_decorators_on_invalid_stairs::closure_env$0>
// 记录形式: compiled_procedure; instances=1
// 源码行: 354
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1570,"record_offset":264,"rva":19620896,"va":5388330016}
// 编译实例 RVAs: 0x12b6420
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1405b0ed0
//     owner=unresolved_namespace; count=1
//     target_aliases=6 candidates; complete list: cull_oob_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13654851","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388330016,"raw_hex":"e8bda42fff","size":5,"soft_fail":false,"target_va":5374676688,"va":5388331534,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: cull_oob_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13250284","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388330016,"raw_hex":"e814d135ff","size":5,"soft_fail":false,"target_va":5375081504,"va":5388331783,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1412593c0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: cull_oob_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-383077","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388330016,"raw_hex":"e89b27faff","size":5,"soft_fail":false,"target_va":5387948992,"va":5388332064,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141264820
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: cull_oob_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-336385","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388330016,"raw_hex":"e8ffddfaff","size":5,"soft_fail":false,"target_va":5387995168,"va":5388331548,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141265710
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: cull_oob_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-331448","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388330016,"raw_hex":"e848f1faff","size":5,"soft_fail":false,"target_va":5387998992,"va":5388330435,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::sync::Arc$LT$T$C$A$GT$::drop_slow
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-11057305","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388330016,"raw_hex":"e8674757ff","size":5,"soft_fail":false,"target_va":5377274272,"va":5388331572,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t23210218","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388330016,"raw_hex":"e8ea286201","size":5,"soft_fail":false,"target_va":5411542064,"va":5388331841,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::history::History::is_recording
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-8585445","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388330016,"raw_hex":"e81bff7cff","size":5,"soft_fail":false,"target_va":5379746160,"va":5388331600,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms::try_position
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-9026733","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388330016,"raw_hex":"e8534376ff","size":5,"soft_fail":false,"target_va":5379304320,"va":5388331048,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator::DecoratorInfo::dst_as_stair
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-11023155","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388330016,"raw_hex":"e8cdcc57ff","size":5,"soft_fail":false,"target_va":5377307760,"va":5388330910,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator_archivist::DecoratorArchivist::is_decorator_cull_allowed
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-7907824","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388330016,"raw_hex":"e8105687ff","size":5,"soft_fail":false,"target_va":5380423056,"va":5388330875,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator_storage::DecoratorStorage::iter_stair_decorators
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-9011755","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388330016,"raw_hex":"e8d57d76ff","size":5,"soft_fail":false,"target_va":5379318464,"va":5388330214,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::hash::random::RandomState::new::KEYS::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-5672235","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388330016,"raw_hex":"e8d572a9ff","size":5,"soft_fail":false,"target_va":5382657856,"va":5388330086,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::random::hashmap_random_keys
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-5576336","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388330016,"raw_hex":"e870e9aaff","size":5,"soft_fail":false,"target_va":5382753792,"va":5388330123,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_decorator::apply_decorator_op::apply_decorator_op
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-261281","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388330016,"raw_hex":"e85f03fcff","size":5,"soft_fail":false,"target_va":5388070448,"va":5388331724,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":1570,"record_offset":5556,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":4457,"source_line":434,"archive":"_pdb/B/modules/01570.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::flatten::impl$3::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\flatten.rs
//     evidence={"module":1570,"record_offset":1364,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":11831,"source_line":63,"archive":"_pdb/B/modules/01570.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\collect.rs
//     evidence={"module":1570,"record_offset":1276,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":4756,"source_line":322,"archive":"_pdb/B/modules/01570.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1570,"record_offset":1304,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":4666,"source_line":998,"archive":"_pdb/B/modules/01570.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1570,"record_offset":6048,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01570.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::insert
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":1570,"record_offset":1336,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":4947,"source_line":1189,"archive":"_pdb/B/modules/01570.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":1570,"record_offset":568,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":7395,"source_line":249,"archive":"_pdb/B/modules/01570.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$46::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":1570,"record_offset":4172,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":10297,"source_line":2382,"archive":"_pdb/B/modules/01570.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$54::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":1570,"record_offset":4848,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":10299,"source_line":2474,"archive":"_pdb/B/modules/01570.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::blueprint::DecoratorBlueprints::remove
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/blueprint.rs
//     evidence={"module":1570,"record_offset":5612,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":25615,"source_line":307,"archive":"_pdb/B/modules/01570.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::cull_oob_decorators::cull_stair_attached_decorators_on_invalid_stairs::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/cull_oob_decorators.rs
//     evidence={"module":1570,"record_offset":3892,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":72047,"source_line":476,"archive":"_pdb/B/modules/01570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1570,"record_offset":6172,"parent_record_offset":6148,"ancestor_record_offset":264,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1570,"record_offset":6148,"parent_record_offset":6120,"ancestor_record_offset":264,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$37::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":1570,"record_offset":5800,"parent_record_offset":5776,"ancestor_record_offset":264,"inlinee":4774,"source_line":2550,"archive":"_pdb/B/modules/01570.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":1570,"record_offset":5580,"parent_record_offset":5556,"ancestor_record_offset":264,"inlinee":4459,"source_line":397,"archive":"_pdb/B/modules/01570.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1570,"record_offset":1100,"parent_record_offset":1072,"ancestor_record_offset":264,"inlinee":4798,"source_line":541,"archive":"_pdb/B/modules/01570.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1570,"record_offset":1160,"parent_record_offset":1128,"ancestor_record_offset":264,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/01570.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1570,"record_offset":2000,"parent_record_offset":1976,"ancestor_record_offset":264,"inlinee":4832,"source_line":1214,"archive":"_pdb/B/modules/01570.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1570,"record_offset":2096,"parent_record_offset":2060,"ancestor_record_offset":264,"inlinee":4486,"source_line":1433,"archive":"_pdb/B/modules/01570.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::flatten::and_then_or_clear
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\flatten.rs
//     evidence={"module":1570,"record_offset":1460,"parent_record_offset":1400,"ancestor_record_offset":264,"inlinee":11087,"source_line":807,"archive":"_pdb/B/modules/01570.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::flatten::impl$33::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\flatten.rs
//     evidence={"module":1570,"record_offset":1400,"parent_record_offset":1364,"ancestor_record_offset":264,"inlinee":11085,"source_line":573,"archive":"_pdb/B/modules/01570.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::fuse::impl$11::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\fuse.rs
//     evidence={"module":1570,"record_offset":2272,"parent_record_offset":2244,"ancestor_record_offset":264,"inlinee":6982,"source_line":358,"archive":"_pdb/B/modules/01570.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::fuse::impl$3::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\fuse.rs
//     evidence={"module":1570,"record_offset":2244,"parent_record_offset":1400,"ancestor_record_offset":264,"inlinee":6980,"source_line":49,"archive":"_pdb/B/modules/01570.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":1570,"record_offset":1556,"parent_record_offset":1524,"ancestor_record_offset":264,"inlinee":6061,"source_line":106,"archive":"_pdb/B/modules/01570.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":1570,"record_offset":964,"parent_record_offset":940,"ancestor_record_offset":264,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/01570.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1570,"record_offset":4768,"parent_record_offset":4680,"ancestor_record_offset":264,"inlinee":4812,"source_line":486,"archive":"_pdb/B/modules/01570.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1570,"record_offset":4708,"parent_record_offset":4680,"ancestor_record_offset":264,"inlinee":4263,"source_line":870,"archive":"_pdb/B/modules/01570.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1570,"record_offset":4736,"parent_record_offset":4708,"ancestor_record_offset":264,"inlinee":4619,"source_line":2579,"archive":"_pdb/B/modules/01570.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1570,"record_offset":1128,"parent_record_offset":1072,"ancestor_record_offset":264,"inlinee":6331,"source_line":1980,"archive":"_pdb/B/modules/01570.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":1570,"record_offset":1896,"parent_record_offset":1868,"ancestor_record_offset":264,"inlinee":4503,"source_line":449,"archive":"_pdb/B/modules/01570.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":1570,"record_offset":1868,"parent_record_offset":1840,"ancestor_record_offset":264,"inlinee":4493,"source_line":601,"archive":"_pdb/B/modules/01570.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":1570,"record_offset":728,"parent_record_offset":696,"ancestor_record_offset":264,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/01570.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":1570,"record_offset":2360,"parent_record_offset":2328,"ancestor_record_offset":264,"inlinee":5024,"source_line":304,"archive":"_pdb/B/modules/01570.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1570,"record_offset":1492,"parent_record_offset":1460,"ancestor_record_offset":264,"inlinee":4130,"source_line":728,"archive":"_pdb/B/modules/01570.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1570,"record_offset":2328,"parent_record_offset":2300,"ancestor_record_offset":264,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/01570.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1570,"record_offset":832,"parent_record_offset":808,"ancestor_record_offset":264,"inlinee":4197,"source_line":1017,"archive":"_pdb/B/modules/01570.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1570,"record_offset":2188,"parent_record_offset":1704,"ancestor_record_offset":264,"inlinee":4384,"source_line":933,"archive":"_pdb/B/modules/01570.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1570,"record_offset":5752,"parent_record_offset":5612,"ancestor_record_offset":264,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01570.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1570,"record_offset":916,"parent_record_offset":808,"ancestor_record_offset":264,"inlinee":7403,"source_line":1581,"archive":"_pdb/B/modules/01570.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1570,"record_offset":2156,"parent_record_offset":2132,"ancestor_record_offset":264,"inlinee":4723,"source_line":1129,"archive":"_pdb/B/modules/01570.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1570,"record_offset":4060,"parent_record_offset":4036,"ancestor_record_offset":264,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/01570.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1570,"record_offset":3984,"parent_record_offset":3948,"ancestor_record_offset":264,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/01570.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1570,"record_offset":988,"parent_record_offset":964,"ancestor_record_offset":264,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/01570.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1570,"record_offset":940,"parent_record_offset":916,"ancestor_record_offset":264,"inlinee":7404,"source_line":1212,"archive":"_pdb/B/modules/01570.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1570,"record_offset":1016,"parent_record_offset":964,"ancestor_record_offset":264,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/01570.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1570,"record_offset":4036,"parent_record_offset":4012,"ancestor_record_offset":264,"inlinee":4510,"source_line":99,"archive":"_pdb/B/modules/01570.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1570,"record_offset":3916,"parent_record_offset":3892,"ancestor_record_offset":264,"inlinee":4402,"source_line":279,"archive":"_pdb/B/modules/01570.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1570,"record_offset":3948,"parent_record_offset":3916,"ancestor_record_offset":264,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/01570.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$186::next_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1570,"record_offset":4012,"parent_record_offset":3948,"ancestor_record_offset":264,"inlinee":4509,"source_line":440,"archive":"_pdb/B/modules/01570.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1570,"record_offset":5836,"parent_record_offset":5800,"ancestor_record_offset":264,"inlinee":5814,"source_line":2986,"archive":"_pdb/B/modules/01570.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1570,"record_offset":5860,"parent_record_offset":5836,"ancestor_record_offset":264,"inlinee":4778,"source_line":3777,"archive":"_pdb/B/modules/01570.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1570,"record_offset":5892,"parent_record_offset":5800,"ancestor_record_offset":264,"inlinee":4779,"source_line":4075,"archive":"_pdb/B/modules/01570.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::stairs::stairs_state::impl$16::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/stairs/stairs_state.rs
//     evidence={"module":1570,"record_offset":4128,"parent_record_offset":4100,"ancestor_record_offset":264,"inlinee":4501,"source_line":38,"archive":"_pdb/B/modules/01570.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_storage::impl$17::iter_stair_decorators::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_storage.rs
//     evidence={"module":1570,"record_offset":2384,"parent_record_offset":2360,"ancestor_record_offset":264,"inlinee":71339,"source_line":718,"archive":"_pdb/B/modules/01570.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":1570,"record_offset":1800,"parent_record_offset":1756,"ancestor_record_offset":264,"inlinee":4490,"source_line":49,"archive":"_pdb/B/modules/01570.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":1570,"record_offset":1840,"parent_record_offset":1800,"ancestor_record_offset":264,"inlinee":4491,"source_line":74,"archive":"_pdb/B/modules/01570.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::remove_lowest_bit
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":1570,"record_offset":1940,"parent_record_offset":1756,"ancestor_record_offset":264,"inlinee":4830,"source_line":37,"archive":"_pdb/B/modules/01570.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":1570,"record_offset":1756,"parent_record_offset":1704,"ancestor_record_offset":264,"inlinee":4488,"source_line":112,"archive":"_pdb/B/modules/01570.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load_aligned
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1570,"record_offset":1976,"parent_record_offset":1704,"ancestor_record_offset":264,"inlinee":4831,"source_line":59,"archive":"_pdb/B/modules/01570.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty_or_deleted
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1570,"record_offset":2060,"parent_record_offset":2032,"ancestor_record_offset":264,"inlinee":4818,"source_line":101,"archive":"_pdb/B/modules/01570.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_full
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1570,"record_offset":2032,"parent_record_offset":1704,"ancestor_record_offset":264,"inlinee":4817,"source_line":117,"archive":"_pdb/B/modules/01570.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1570,"record_offset":2432,"parent_record_offset":2408,"ancestor_record_offset":264,"inlinee":4950,"source_line":754,"archive":"_pdb/B/modules/01570.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::remove
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1570,"record_offset":5660,"parent_record_offset":5636,"ancestor_record_offset":264,"inlinee":5625,"source_line":1954,"archive":"_pdb/B/modules/01570.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::remove_entry
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1570,"record_offset":5684,"parent_record_offset":5660,"ancestor_record_offset":264,"inlinee":5626,"source_line":1993,"archive":"_pdb/B/modules/01570.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$42::into_iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1570,"record_offset":4212,"parent_record_offset":4172,"ancestor_record_offset":264,"inlinee":4805,"source_line":3151,"archive":"_pdb/B/modules/01570.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$44::next
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1570,"record_offset":1620,"parent_record_offset":1588,"ancestor_record_offset":264,"inlinee":10642,"source_line":3171,"archive":"_pdb/B/modules/01570.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$53::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1570,"record_offset":4876,"parent_record_offset":4848,"ancestor_record_offset":264,"inlinee":4824,"source_line":3275,"archive":"_pdb/B/modules/01570.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1570,"record_offset":5712,"parent_record_offset":5684,"ancestor_record_offset":264,"inlinee":4514,"source_line":253,"archive":"_pdb/B/modules/01570.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::next_n
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1570,"record_offset":2132,"parent_record_offset":1704,"ancestor_record_offset":264,"inlinee":4833,"source_line":439,"archive":"_pdb/B/modules/01570.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::new
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1570,"record_offset":2540,"parent_record_offset":2480,"ancestor_record_offset":264,"inlinee":4816,"source_line":3397,"archive":"_pdb/B/modules/01570.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::next_impl
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1570,"record_offset":1704,"parent_record_offset":1652,"ancestor_record_offset":264,"inlinee":4829,"source_line":3464,"archive":"_pdb/B/modules/01570.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::into_allocation
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1570,"record_offset":4632,"parent_record_offset":4592,"ancestor_record_offset":264,"inlinee":4809,"source_line":1358,"archive":"_pdb/B/modules/01570.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::into_iter_from
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1570,"record_offset":4592,"parent_record_offset":4252,"ancestor_record_offset":264,"inlinee":4808,"source_line":1344,"archive":"_pdb/B/modules/01570.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::iter
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1570,"record_offset":2456,"parent_record_offset":2432,"ancestor_record_offset":264,"inlinee":4813,"source_line":1288,"archive":"_pdb/B/modules/01570.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::buckets
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1570,"record_offset":4556,"parent_record_offset":4332,"ancestor_record_offset":264,"inlinee":4952,"source_line":2488,"archive":"_pdb/B/modules/01570.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::data_end
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1570,"record_offset":2512,"parent_record_offset":2480,"ancestor_record_offset":264,"inlinee":4951,"source_line":2291,"archive":"_pdb/B/modules/01570.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1570,"record_offset":2480,"parent_record_offset":2456,"ancestor_record_offset":264,"inlinee":4814,"source_line":1991,"archive":"_pdb/B/modules/01570.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::TableLayout::calculate_layout_for
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1570,"record_offset":4680,"parent_record_offset":4632,"ancestor_record_offset":264,"inlinee":4811,"source_line":162,"archive":"_pdb/B/modules/01570.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$19::into_iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1570,"record_offset":4252,"parent_record_offset":4212,"ancestor_record_offset":264,"inlinee":4807,"source_line":3343,"archive":"_pdb/B/modules/01570.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$29::next
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1570,"record_offset":1652,"parent_record_offset":1620,"ancestor_record_offset":264,"inlinee":4828,"source_line":3661,"archive":"_pdb/B/modules/01570.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$39::drop
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1570,"record_offset":6120,"parent_record_offset":6096,"ancestor_record_offset":264,"inlinee":4904,"source_line":3846,"archive":"_pdb/B/modules/01570.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$41::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1570,"record_offset":4904,"parent_record_offset":4876,"ancestor_record_offset":264,"inlinee":4826,"source_line":3887,"archive":"_pdb/B/modules/01570.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":1570,"record_offset":2408,"parent_record_offset":2384,"ancestor_record_offset":264,"inlinee":4949,"source_line":535,"archive":"_pdb/B/modules/01570.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::remove
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":1570,"record_offset":5636,"parent_record_offset":5612,"ancestor_record_offset":264,"inlinee":8292,"source_line":1244,"archive":"_pdb/B/modules/01570.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$47::next
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":1570,"record_offset":1588,"parent_record_offset":1556,"ancestor_record_offset":264,"inlinee":10640,"source_line":2392,"archive":"_pdb/B/modules/01570.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$8::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":1570,"record_offset":592,"parent_record_offset":568,"ancestor_record_offset":264,"inlinee":7397,"source_line":1406,"archive":"_pdb/B/modules/01570.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":1570,"record_offset":648,"parent_record_offset":616,"ancestor_record_offset":264,"inlinee":4794,"source_line":56,"archive":"_pdb/B/modules/01570.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::__init
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":1570,"record_offset":876,"parent_record_offset":856,"ancestor_record_offset":264,"inlinee":7402,"source_line":83,"archive":"_pdb/B/modules/01570.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::constant$0::closure$1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":1570,"record_offset":752,"parent_record_offset":728,"ancestor_record_offset":264,"inlinee":7399,"source_line":97,"archive":"_pdb/B/modules/01570.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":1570,"record_offset":1072,"parent_record_offset":696,"ancestor_record_offset":264,"inlinee":4797,"source_line":72,"archive":"_pdb/B/modules/01570.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":1570,"record_offset":616,"parent_record_offset":592,"ancestor_record_offset":264,"inlinee":4792,"source_line":151,"archive":"_pdb/B/modules/01570.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":1570,"record_offset":780,"parent_record_offset":752,"ancestor_record_offset":264,"inlinee":4425,"source_line":51,"archive":"_pdb/B/modules/01570.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::initialize
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":1570,"record_offset":808,"parent_record_offset":780,"ancestor_record_offset":264,"inlinee":7400,"source_line":61,"archive":"_pdb/B/modules/01570.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":1570,"record_offset":696,"parent_record_offset":672,"ancestor_record_offset":264,"inlinee":4421,"source_line":305,"archive":"_pdb/B/modules/01570.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":1570,"record_offset":672,"parent_record_offset":648,"ancestor_record_offset":264,"inlinee":4420,"source_line":270,"archive":"_pdb/B/modules/01570.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::cull_oob_decorators::cull_stair_attached_decorators_on_invalid_stairs::closure$0::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/cull_oob_decorators.rs
//     evidence={"module":1570,"record_offset":4100,"parent_record_offset":3916,"ancestor_record_offset":264,"inlinee":72049,"source_line":476,"archive":"_pdb/B/modules/01570.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::cull_oob_decorators::cull_stair_attached_decorators
// 记录形式: compiled_procedure; instances=1
// 源码行: 432
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":453,"record_offset":22352,"rva":8449040,"va":5377158160}
// 编译实例 RVAs: 0x80ec10
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1412b6c30
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: cull_oob_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11173238","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377158160,"raw_hex":"e8767daa00","size":5,"soft_fail":false,"target_va":5388332080,"va":5377158837,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t34382589","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377158160,"raw_hex":"e8fda20c02","size":5,"soft_fail":false,"target_va":5411541584,"va":5377158990,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-7541","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377158160,"raw_hex":"e88be2ffff","size":5,"soft_fail":false,"target_va":5377150704,"va":5377158240,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t34385045","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377158160,"raw_hex":"e895ac0c02","size":5,"soft_fail":false,"target_va":5411543360,"va":5377158310,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t4226773","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377158160,"raw_hex":"e8d57e4000","size":5,"soft_fail":false,"target_va":5381385536,"va":5377158758,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t4231960","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377158160,"raw_hex":"e818934000","size":5,"soft_fail":false,"target_va":5381390640,"va":5377158675,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::end_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t4227582","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377158160,"raw_hex":"e8fe814000","size":5,"soft_fail":false,"target_va":5381386480,"va":5377158893,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t4226492","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377158160,"raw_hex":"e8bc7d4000","size":5,"soft_fail":false,"target_va":5381384704,"va":5377158207,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::thread_local::native::lazy::Storage$LT$T$C$D$GT$::initialize
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t34485863","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377158160,"raw_hex":"e867360e02","size":5,"soft_fail":false,"target_va":5411644800,"va":5377158932,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::LocalKey$LT$T$GT$::with
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t10783710","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377158160,"raw_hex":"e8de8ba400","size":5,"soft_fail":false,"target_va":5387942848,"va":5377159133,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t34378121","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377158160,"raw_hex":"e889910c02","size":5,"soft_fail":false,"target_va":5411537104,"va":5377158978,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":453,"record_offset":24920,"parent_record_offset":22352,"ancestor_record_offset":22352,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":453,"record_offset":23952,"parent_record_offset":22352,"ancestor_record_offset":22352,"inlinee":4416,"source_line":589,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":453,"record_offset":22520,"parent_record_offset":22352,"ancestor_record_offset":22352,"inlinee":4368,"source_line":626,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":453,"record_offset":22976,"parent_record_offset":22352,"ancestor_record_offset":22352,"inlinee":4385,"source_line":659,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":453,"record_offset":24272,"parent_record_offset":24244,"ancestor_record_offset":22352,"inlinee":4432,"source_line":1817,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":453,"record_offset":24324,"parent_record_offset":24300,"ancestor_record_offset":22352,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":453,"record_offset":24300,"parent_record_offset":24272,"ancestor_record_offset":22352,"inlinee":4434,"source_line":428,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":453,"record_offset":24220,"parent_record_offset":24192,"ancestor_record_offset":22352,"inlinee":4429,"source_line":1089,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":453,"record_offset":24244,"parent_record_offset":24220,"ancestor_record_offset":22352,"inlinee":4430,"source_line":1121,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":453,"record_offset":24416,"parent_record_offset":24244,"ancestor_record_offset":22352,"inlinee":4439,"source_line":2194,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$44::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":453,"record_offset":24528,"parent_record_offset":24504,"ancestor_record_offset":22352,"inlinee":4443,"source_line":1808,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":453,"record_offset":24348,"parent_record_offset":24324,"ancestor_record_offset":22352,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\mod.rs
//     evidence={"module":453,"record_offset":22676,"parent_record_offset":22644,"ancestor_record_offset":22352,"inlinee":4377,"source_line":1044,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":453,"record_offset":24088,"parent_record_offset":24048,"ancestor_record_offset":22352,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":453,"record_offset":22924,"parent_record_offset":22896,"ancestor_record_offset":22352,"inlinee":4384,"source_line":933,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":453,"record_offset":24876,"parent_record_offset":24048,"ancestor_record_offset":22352,"inlinee":4795,"source_line":296,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":453,"record_offset":24480,"parent_record_offset":24192,"ancestor_record_offset":22352,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":453,"record_offset":23388,"parent_record_offset":23364,"ancestor_record_offset":22352,"inlinee":4410,"source_line":533,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":453,"record_offset":23364,"parent_record_offset":23340,"ancestor_record_offset":22352,"inlinee":4409,"source_line":686,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":453,"record_offset":23572,"parent_record_offset":23544,"ancestor_record_offset":22352,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":453,"record_offset":24372,"parent_record_offset":24348,"ancestor_record_offset":22352,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":453,"record_offset":23316,"parent_record_offset":23292,"ancestor_record_offset":22352,"inlinee":4407,"source_line":80,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":453,"record_offset":23340,"parent_record_offset":23316,"ancestor_record_offset":22352,"inlinee":4408,"source_line":118,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":453,"record_offset":23644,"parent_record_offset":23616,"ancestor_record_offset":22352,"inlinee":4402,"source_line":279,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$183::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":453,"record_offset":23292,"parent_record_offset":23176,"ancestor_record_offset":22352,"inlinee":4405,"source_line":392,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":453,"record_offset":22644,"parent_record_offset":22616,"ancestor_record_offset":22352,"inlinee":4375,"source_line":366,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":453,"record_offset":23004,"parent_record_offset":22976,"ancestor_record_offset":22352,"inlinee":4386,"source_line":1488,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":453,"record_offset":23144,"parent_record_offset":23112,"ancestor_record_offset":22352,"inlinee":4396,"source_line":128,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":453,"record_offset":23112,"parent_record_offset":23076,"ancestor_record_offset":22352,"inlinee":4394,"source_line":218,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":453,"record_offset":23052,"parent_record_offset":23028,"ancestor_record_offset":22352,"inlinee":4390,"source_line":315,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":453,"record_offset":23076,"parent_record_offset":23052,"ancestor_record_offset":22352,"inlinee":4392,"source_line":709,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":453,"record_offset":23028,"parent_record_offset":23004,"ancestor_record_offset":22352,"inlinee":4388,"source_line":798,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":453,"record_offset":23616,"parent_record_offset":23076,"ancestor_record_offset":22352,"inlinee":4400,"source_line":659,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":453,"record_offset":22616,"parent_record_offset":22584,"ancestor_record_offset":22352,"inlinee":4373,"source_line":433,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":453,"record_offset":22584,"parent_record_offset":22556,"ancestor_record_offset":22352,"inlinee":4372,"source_line":463,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":453,"record_offset":22776,"parent_record_offset":22744,"ancestor_record_offset":22352,"inlinee":4380,"source_line":501,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":453,"record_offset":22872,"parent_record_offset":22776,"ancestor_record_offset":22352,"inlinee":4381,"source_line":521,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":453,"record_offset":22744,"parent_record_offset":22720,"ancestor_record_offset":22352,"inlinee":4379,"source_line":533,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":453,"record_offset":22556,"parent_record_offset":22520,"ancestor_record_offset":22352,"inlinee":4370,"source_line":59,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":453,"record_offset":22896,"parent_record_offset":22872,"ancestor_record_offset":22352,"inlinee":4383,"source_line":189,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":453,"record_offset":23176,"parent_record_offset":23144,"ancestor_record_offset":22352,"inlinee":4398,"source_line":79,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":453,"record_offset":23460,"parent_record_offset":23176,"ancestor_record_offset":22352,"inlinee":4412,"source_line":17,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":453,"record_offset":23432,"parent_record_offset":23176,"ancestor_record_offset":22352,"inlinee":4411,"source_line":11,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":453,"record_offset":23236,"parent_record_offset":23176,"ancestor_record_offset":22352,"inlinee":4403,"source_line":24,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":453,"record_offset":23984,"parent_record_offset":23952,"ancestor_record_offset":22352,"inlinee":4418,"source_line":425,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":453,"record_offset":24984,"parent_record_offset":24952,"ancestor_record_offset":22352,"inlinee":4966,"source_line":601,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":453,"record_offset":25484,"parent_record_offset":25224,"ancestor_record_offset":22352,"inlinee":4968,"source_line":602,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":453,"record_offset":24116,"parent_record_offset":24088,"ancestor_record_offset":22352,"inlinee":4423,"source_line":90,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":453,"record_offset":24192,"parent_record_offset":24048,"ancestor_record_offset":22352,"inlinee":4427,"source_line":429,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":453,"record_offset":24452,"parent_record_offset":24192,"ancestor_record_offset":22352,"inlinee":4441,"source_line":591,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":453,"record_offset":24144,"parent_record_offset":24116,"ancestor_record_offset":22352,"inlinee":4425,"source_line":51,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":453,"record_offset":24048,"parent_record_offset":24016,"ancestor_record_offset":22352,"inlinee":4421,"source_line":305,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":453,"record_offset":24016,"parent_record_offset":23984,"ancestor_record_offset":22352,"inlinee":4420,"source_line":270,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::cull_oob_decorators::cull_stair_attached_decorators::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 448
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x11973
// 首条位置: {"module":1570,"inlinee":72051}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::cull_oob_decorators::cull_stair_attached_decorators_on_invalid_stairs
// 记录形式: compiled_procedure; instances=1
// 源码行: 460
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":453,"record_offset":26076,"rva":8450112,"va":5377159232}
// 编译实例 RVAs: 0x80f040
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1412b6420
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: cull_oob_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11170102","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377159232,"raw_hex":"e83671aa00","size":5,"soft_fail":false,"target_va":5388330016,"va":5377159909,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t34381517","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377159232,"raw_hex":"e8cd9e0c02","size":5,"soft_fail":false,"target_va":5411541584,"va":5377160062,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-8613","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377159232,"raw_hex":"e85bdeffff","size":5,"soft_fail":false,"target_va":5377150704,"va":5377159312,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t34383973","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377159232,"raw_hex":"e865a80c02","size":5,"soft_fail":false,"target_va":5411543360,"va":5377159382,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t4225701","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377159232,"raw_hex":"e8a57a4000","size":5,"soft_fail":false,"target_va":5381385536,"va":5377159830,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t4230888","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377159232,"raw_hex":"e8e88e4000","size":5,"soft_fail":false,"target_va":5381390640,"va":5377159747,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::end_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t4226510","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377159232,"raw_hex":"e8ce7d4000","size":5,"soft_fail":false,"target_va":5381386480,"va":5377159965,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t4225420","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377159232,"raw_hex":"e88c794000","size":5,"soft_fail":false,"target_va":5381384704,"va":5377159279,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::thread_local::native::lazy::Storage$LT$T$C$D$GT$::initialize
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t34484791","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377159232,"raw_hex":"e837320e02","size":5,"soft_fail":false,"target_va":5411644800,"va":5377160004,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::LocalKey$LT$T$GT$::with
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t10782638","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377159232,"raw_hex":"e8ae87a400","size":5,"soft_fail":false,"target_va":5387942848,"va":5377160205,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t34377049","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377159232,"raw_hex":"e8598d0c02","size":5,"soft_fail":false,"target_va":5411537104,"va":5377160050,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":453,"record_offset":28660,"parent_record_offset":26076,"ancestor_record_offset":26076,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":453,"record_offset":27692,"parent_record_offset":26076,"ancestor_record_offset":26076,"inlinee":4416,"source_line":589,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":453,"record_offset":26260,"parent_record_offset":26076,"ancestor_record_offset":26076,"inlinee":4368,"source_line":626,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":453,"record_offset":26716,"parent_record_offset":26076,"ancestor_record_offset":26076,"inlinee":4385,"source_line":659,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":453,"record_offset":28012,"parent_record_offset":27984,"ancestor_record_offset":26076,"inlinee":4432,"source_line":1817,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":453,"record_offset":28064,"parent_record_offset":28040,"ancestor_record_offset":26076,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":453,"record_offset":28040,"parent_record_offset":28012,"ancestor_record_offset":26076,"inlinee":4434,"source_line":428,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":453,"record_offset":27960,"parent_record_offset":27932,"ancestor_record_offset":26076,"inlinee":4429,"source_line":1089,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":453,"record_offset":27984,"parent_record_offset":27960,"ancestor_record_offset":26076,"inlinee":4430,"source_line":1121,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":453,"record_offset":28156,"parent_record_offset":27984,"ancestor_record_offset":26076,"inlinee":4439,"source_line":2194,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$44::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":453,"record_offset":28268,"parent_record_offset":28244,"ancestor_record_offset":26076,"inlinee":4443,"source_line":1808,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":453,"record_offset":28088,"parent_record_offset":28064,"ancestor_record_offset":26076,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\mod.rs
//     evidence={"module":453,"record_offset":26416,"parent_record_offset":26384,"ancestor_record_offset":26076,"inlinee":4377,"source_line":1044,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":453,"record_offset":27828,"parent_record_offset":27788,"ancestor_record_offset":26076,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":453,"record_offset":26664,"parent_record_offset":26636,"ancestor_record_offset":26076,"inlinee":4384,"source_line":933,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":453,"record_offset":28616,"parent_record_offset":27788,"ancestor_record_offset":26076,"inlinee":4795,"source_line":296,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":453,"record_offset":28220,"parent_record_offset":27932,"ancestor_record_offset":26076,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":453,"record_offset":27128,"parent_record_offset":27104,"ancestor_record_offset":26076,"inlinee":4410,"source_line":533,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":453,"record_offset":27104,"parent_record_offset":27080,"ancestor_record_offset":26076,"inlinee":4409,"source_line":686,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":453,"record_offset":27312,"parent_record_offset":27284,"ancestor_record_offset":26076,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":453,"record_offset":28112,"parent_record_offset":28088,"ancestor_record_offset":26076,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":453,"record_offset":27056,"parent_record_offset":27032,"ancestor_record_offset":26076,"inlinee":4407,"source_line":80,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":453,"record_offset":27080,"parent_record_offset":27056,"ancestor_record_offset":26076,"inlinee":4408,"source_line":118,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":453,"record_offset":27384,"parent_record_offset":27356,"ancestor_record_offset":26076,"inlinee":4402,"source_line":279,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$183::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":453,"record_offset":27032,"parent_record_offset":26916,"ancestor_record_offset":26076,"inlinee":4405,"source_line":392,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":453,"record_offset":26384,"parent_record_offset":26356,"ancestor_record_offset":26076,"inlinee":4375,"source_line":366,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":453,"record_offset":26744,"parent_record_offset":26716,"ancestor_record_offset":26076,"inlinee":4386,"source_line":1488,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":453,"record_offset":26884,"parent_record_offset":26852,"ancestor_record_offset":26076,"inlinee":4396,"source_line":128,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":453,"record_offset":26852,"parent_record_offset":26816,"ancestor_record_offset":26076,"inlinee":4394,"source_line":218,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":453,"record_offset":26792,"parent_record_offset":26768,"ancestor_record_offset":26076,"inlinee":4390,"source_line":315,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":453,"record_offset":26816,"parent_record_offset":26792,"ancestor_record_offset":26076,"inlinee":4392,"source_line":709,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":453,"record_offset":26768,"parent_record_offset":26744,"ancestor_record_offset":26076,"inlinee":4388,"source_line":798,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":453,"record_offset":27356,"parent_record_offset":26816,"ancestor_record_offset":26076,"inlinee":4400,"source_line":659,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":453,"record_offset":26356,"parent_record_offset":26324,"ancestor_record_offset":26076,"inlinee":4373,"source_line":433,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":453,"record_offset":26324,"parent_record_offset":26296,"ancestor_record_offset":26076,"inlinee":4372,"source_line":463,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":453,"record_offset":26516,"parent_record_offset":26484,"ancestor_record_offset":26076,"inlinee":4380,"source_line":501,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":453,"record_offset":26612,"parent_record_offset":26516,"ancestor_record_offset":26076,"inlinee":4381,"source_line":521,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":453,"record_offset":26484,"parent_record_offset":26460,"ancestor_record_offset":26076,"inlinee":4379,"source_line":533,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":453,"record_offset":26296,"parent_record_offset":26260,"ancestor_record_offset":26076,"inlinee":4370,"source_line":59,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":453,"record_offset":26636,"parent_record_offset":26612,"ancestor_record_offset":26076,"inlinee":4383,"source_line":189,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":453,"record_offset":26916,"parent_record_offset":26884,"ancestor_record_offset":26076,"inlinee":4398,"source_line":79,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":453,"record_offset":27200,"parent_record_offset":26916,"ancestor_record_offset":26076,"inlinee":4412,"source_line":17,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":453,"record_offset":27172,"parent_record_offset":26916,"ancestor_record_offset":26076,"inlinee":4411,"source_line":11,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":453,"record_offset":26976,"parent_record_offset":26916,"ancestor_record_offset":26076,"inlinee":4403,"source_line":24,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":453,"record_offset":27724,"parent_record_offset":27692,"ancestor_record_offset":26076,"inlinee":4418,"source_line":425,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":453,"record_offset":28724,"parent_record_offset":28692,"ancestor_record_offset":26076,"inlinee":4966,"source_line":601,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":453,"record_offset":29224,"parent_record_offset":28964,"ancestor_record_offset":26076,"inlinee":4968,"source_line":602,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":453,"record_offset":27856,"parent_record_offset":27828,"ancestor_record_offset":26076,"inlinee":4423,"source_line":90,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":453,"record_offset":27932,"parent_record_offset":27788,"ancestor_record_offset":26076,"inlinee":4427,"source_line":429,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":453,"record_offset":28192,"parent_record_offset":27932,"ancestor_record_offset":26076,"inlinee":4441,"source_line":591,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":453,"record_offset":27884,"parent_record_offset":27856,"ancestor_record_offset":26076,"inlinee":4425,"source_line":51,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":453,"record_offset":27788,"parent_record_offset":27756,"ancestor_record_offset":26076,"inlinee":4421,"source_line":305,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":453,"record_offset":27756,"parent_record_offset":27724,"ancestor_record_offset":26076,"inlinee":4420,"source_line":270,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::cull_oob_decorators::cull_stair_attached_decorators_on_invalid_stairs::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 476
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1196F
// 首条位置: {"module":1570,"inlinee":72047}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::slice::iter::impl$182::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1570,"record_offset":3916,"parent_record_offset":3892,"ancestor_record_offset":3892,"inlinee":4402,"source_line":279,"archive":"_pdb/B/modules/01570.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1570,"record_offset":4060,"parent_record_offset":4036,"ancestor_record_offset":3892,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/01570.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1570,"record_offset":3984,"parent_record_offset":3948,"ancestor_record_offset":3892,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/01570.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1570,"record_offset":4036,"parent_record_offset":4012,"ancestor_record_offset":3892,"inlinee":4510,"source_line":99,"archive":"_pdb/B/modules/01570.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1570,"record_offset":3948,"parent_record_offset":3916,"ancestor_record_offset":3892,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/01570.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$186::next_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1570,"record_offset":4012,"parent_record_offset":3948,"ancestor_record_offset":3892,"inlinee":4509,"source_line":440,"archive":"_pdb/B/modules/01570.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::stairs::stairs_state::impl$16::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/stairs/stairs_state.rs
//     evidence={"module":1570,"record_offset":4128,"parent_record_offset":4100,"ancestor_record_offset":3892,"inlinee":4501,"source_line":38,"archive":"_pdb/B/modules/01570.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::cull_oob_decorators::cull_stair_attached_decorators_on_invalid_stairs::closure$0::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/cull_oob_decorators.rs
//     evidence={"module":1570,"record_offset":4100,"parent_record_offset":3916,"ancestor_record_offset":3892,"inlinee":72049,"source_line":476,"archive":"_pdb/B/modules/01570.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::cull_oob_decorators::cull_stair_attached_decorators_on_invalid_stairs::closure$0::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 476
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x11971
// 首条位置: {"module":1570,"inlinee":72049}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::resources::stairs::stairs_state::impl$16::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/stairs/stairs_state.rs
//     evidence={"module":1570,"record_offset":4128,"parent_record_offset":4100,"ancestor_record_offset":4100,"inlinee":4501,"source_line":38,"archive":"_pdb/B/modules/01570.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::cull_oob_decorators::DecoratorLocationCheckResult::is_valid
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 493
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x640A
// 首条位置: {"module":453,"inlinee":25610}
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
// 源码行: 497
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6427
// 首条位置: {"module":1576,"inlinee":25639}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] system_decorator::cull_oob_decorators::impl$1::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/cull_oob_decorators.rs
//     evidence={"module":1576,"record_offset":9920,"parent_record_offset":9872,"ancestor_record_offset":9872,"inlinee":25617,"source_line":506,"archive":"_pdb/B/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$101::clone
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1576,"record_offset":9988,"parent_record_offset":9920,"ancestor_record_offset":9872,"inlinee":25047,"source_line":17,"archive":"_pdb/B/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::impl$6::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":1576,"record_offset":10044,"parent_record_offset":9920,"ancestor_record_offset":9872,"inlinee":25619,"source_line":114,"archive":"_pdb/B/modules/01576.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::cull_oob_decorators::impl$1::clone
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 506
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6411
// 首条位置: {"module":453,"inlinee":25617}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec3::impl$101::clone
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1576,"record_offset":9988,"parent_record_offset":9920,"ancestor_record_offset":9920,"inlinee":25047,"source_line":17,"archive":"_pdb/B/modules/01576.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::rectangle::impl$6::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":1576,"record_offset":10044,"parent_record_offset":9920,"ancestor_record_offset":9920,"inlinee":25619,"source_line":114,"archive":"_pdb/B/modules/01576.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::cull_oob_decorators::is_wall_decorator_location_valid
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 707
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6415
// 首条位置: {"module":453,"inlinee":25621}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::and_then
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":453,"record_offset":36448,"parent_record_offset":31664,"ancestor_record_offset":31664,"inlinee":4518,"source_line":1447,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":453,"record_offset":32448,"parent_record_offset":31664,"ancestor_record_offset":31664,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":453,"record_offset":35348,"parent_record_offset":31664,"ancestor_record_offset":31664,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":453,"record_offset":38080,"parent_record_offset":31664,"ancestor_record_offset":31664,"inlinee":12923,"source_line":2551,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":453,"record_offset":35692,"parent_record_offset":31664,"ancestor_record_offset":31664,"inlinee":4460,"source_line":1050,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::last
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":453,"record_offset":35252,"parent_record_offset":31664,"ancestor_record_offset":31664,"inlinee":4716,"source_line":279,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo::dims_wall_space
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":453,"record_offset":32192,"parent_record_offset":31664,"ancestor_record_offset":31664,"inlinee":25147,"source_line":285,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::public_wall_state::PublicWallState::is_premade_shape
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_wall_state.rs
//     evidence={"module":453,"record_offset":36420,"parent_record_offset":31664,"ancestor_record_offset":31664,"inlinee":24411,"source_line":113,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":453,"record_offset":35776,"parent_record_offset":31664,"ancestor_record_offset":31664,"inlinee":4749,"source_line":166,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::normalize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":453,"record_offset":34400,"parent_record_offset":31664,"ancestor_record_offset":31664,"inlinee":10290,"source_line":466,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::perp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":453,"record_offset":34372,"parent_record_offset":31664,"ancestor_record_offset":31664,"inlinee":4741,"source_line":891,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::cull_oob_decorators::is_wall_decorator_location_valid::closure$0
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/cull_oob_decorators.rs
//     evidence={"module":453,"record_offset":32484,"parent_record_offset":31664,"ancestor_record_offset":31664,"inlinee":25623,"source_line":728,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::cull_oob_decorators::should_snap_to_corner
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/cull_oob_decorators.rs
//     evidence={"module":453,"record_offset":36052,"parent_record_offset":31664,"ancestor_record_offset":31664,"inlinee":25626,"source_line":1168,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::get_pos_at_coord
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":453,"record_offset":33384,"parent_record_offset":31664,"ancestor_record_offset":31664,"inlinee":9575,"source_line":799,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::get_tangent_at_coord
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":453,"record_offset":33872,"parent_record_offset":31664,"ancestor_record_offset":31664,"inlinee":24690,"source_line":816,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::impl$12::sample_at_coord
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":453,"record_offset":34684,"parent_record_offset":31664,"ancestor_record_offset":31664,"inlinee":4637,"source_line":1392,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_direct] utils::wall_space::WallSpaceCoord::new
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/wall_space.rs
//     evidence={"module":453,"record_offset":36024,"parent_record_offset":31664,"ancestor_record_offset":31664,"inlinee":4897,"source_line":116,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_direct] utils::wall_space::WallSpaceCoord::with_offset
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/wall_space.rs
//     evidence={"module":453,"record_offset":34620,"parent_record_offset":31664,"ancestor_record_offset":31664,"inlinee":25625,"source_line":213,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":453,"record_offset":35632,"parent_record_offset":35596,"ancestor_record_offset":31664,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":453,"record_offset":35596,"parent_record_offset":35448,"ancestor_record_offset":31664,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":453,"record_offset":36764,"parent_record_offset":36740,"ancestor_record_offset":31664,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":453,"record_offset":35500,"parent_record_offset":35448,"ancestor_record_offset":31664,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":453,"record_offset":35448,"parent_record_offset":35412,"ancestor_record_offset":31664,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":453,"record_offset":36812,"parent_record_offset":36788,"ancestor_record_offset":31664,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":453,"record_offset":36788,"parent_record_offset":36764,"ancestor_record_offset":31664,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":453,"record_offset":35412,"parent_record_offset":35380,"ancestor_record_offset":31664,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":453,"record_offset":36740,"parent_record_offset":36716,"ancestor_record_offset":31664,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":453,"record_offset":36716,"parent_record_offset":36616,"ancestor_record_offset":31664,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$13::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":453,"record_offset":33416,"parent_record_offset":33384,"ancestor_record_offset":31664,"inlinee":4358,"source_line":3357,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":453,"record_offset":37152,"parent_record_offset":37128,"ancestor_record_offset":31664,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::impl$25::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":453,"record_offset":36916,"parent_record_offset":36584,"ancestor_record_offset":31664,"inlinee":4846,"source_line":76,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::Entities::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/entity/mod.rs
//     evidence={"module":453,"record_offset":36584,"parent_record_offset":36536,"ancestor_record_offset":31664,"inlinee":4840,"source_line":842,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::StorageSwitch::extract
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":453,"record_offset":37908,"parent_record_offset":37884,"ancestor_record_offset":31664,"inlinee":4876,"source_line":2472,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$26::set_archetype
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":453,"record_offset":37388,"parent_record_offset":36536,"ancestor_record_offset":31664,"inlinee":4856,"source_line":1160,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$26::set_table
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":453,"record_offset":37412,"parent_record_offset":37388,"ancestor_record_offset":31664,"inlinee":4857,"source_line":1175,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$27::fetch
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":453,"record_offset":37884,"parent_record_offset":36536,"ancestor_record_offset":31664,"inlinee":4874,"source_line":1229,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$27::fetch::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":453,"record_offset":37932,"parent_record_offset":37908,"ancestor_record_offset":31664,"inlinee":4878,"source_line":1235,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_array::BlobArray::get_sub_slice
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_array.rs
//     evidence={"module":453,"record_offset":37832,"parent_record_offset":37808,"ancestor_record_offset":31664,"inlinee":4872,"source_line":143,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseArray::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":453,"record_offset":37512,"parent_record_offset":37488,"ancestor_record_offset":31664,"inlinee":4864,"source_line":58,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":453,"record_offset":37488,"parent_record_offset":37464,"ancestor_record_offset":31664,"inlinee":4862,"source_line":395,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$16::get::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":453,"record_offset":37692,"parent_record_offset":37668,"ancestor_record_offset":31664,"inlinee":4866,"source_line":396,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_column
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":453,"record_offset":37464,"parent_record_offset":37436,"ancestor_record_offset":31664,"inlinee":4860,"source_line":475,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_data_slice_for
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":453,"record_offset":37436,"parent_record_offset":37412,"ancestor_record_offset":31664,"inlinee":4859,"source_line":363,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":453,"record_offset":37128,"parent_record_offset":36536,"ancestor_record_offset":31664,"inlinee":4854,"source_line":721,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::column::ThinColumn::get_data_slice
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/column.rs
//     evidence={"module":453,"record_offset":37808,"parent_record_offset":37784,"ancestor_record_offset":31664,"inlinee":4870,"source_line":303,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::impl$4::get_data_slice_for::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":453,"record_offset":37784,"parent_record_offset":37760,"ancestor_record_offset":31664,"inlinee":4868,"source_line":368,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":453,"record_offset":36508,"parent_record_offset":36480,"ancestor_record_offset":31664,"inlinee":4837,"source_line":1385,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::get_inner
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":453,"record_offset":36536,"parent_record_offset":36508,"ancestor_record_offset":31664,"inlinee":4838,"source_line":1591,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ptr::ThinSlicePtr::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ptr","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ptr-0.16.0/src/lib.rs
//     evidence={"module":453,"record_offset":37956,"parent_record_offset":37932,"ancestor_record_offset":31664,"inlinee":4880,"source_line":526,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::bool::impl$0::then
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\bool.rs
//     evidence={"module":453,"record_offset":36976,"parent_record_offset":36948,"ancestor_record_offset":31664,"inlinee":4781,"source_line":61,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$24::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":453,"record_offset":36884,"parent_record_offset":36860,"ancestor_record_offset":31664,"inlinee":4844,"source_line":1742,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::abs
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":453,"record_offset":36204,"parent_record_offset":36052,"ancestor_record_offset":31664,"inlinee":10511,"source_line":1446,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":453,"record_offset":32764,"parent_record_offset":32484,"ancestor_record_offset":31664,"inlinee":4551,"source_line":890,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":453,"record_offset":32320,"parent_record_offset":32296,"ancestor_record_offset":31664,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":453,"record_offset":35552,"parent_record_offset":35500,"ancestor_record_offset":31664,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::impl$2::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":453,"record_offset":36860,"parent_record_offset":36584,"ancestor_record_offset":31664,"inlinee":4842,"source_line":200,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":453,"record_offset":35052,"parent_record_offset":34684,"ancestor_record_offset":31664,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":453,"record_offset":33744,"parent_record_offset":33716,"ancestor_record_offset":31664,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":453,"record_offset":34056,"parent_record_offset":34028,"ancestor_record_offset":31664,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":453,"record_offset":37604,"parent_record_offset":37540,"ancestor_record_offset":31664,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":453,"record_offset":37628,"parent_record_offset":37604,"ancestor_record_offset":31664,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":453,"record_offset":37668,"parent_record_offset":37488,"ancestor_record_offset":31664,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":453,"record_offset":35208,"parent_record_offset":35176,"ancestor_record_offset":31664,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":453,"record_offset":32264,"parent_record_offset":32236,"ancestor_record_offset":31664,"inlinee":4197,"source_line":1017,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":453,"record_offset":35380,"parent_record_offset":35348,"ancestor_record_offset":31664,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":453,"record_offset":35740,"parent_record_offset":35716,"ancestor_record_offset":31664,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$45::slice_contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":453,"record_offset":38104,"parent_record_offset":38080,"ancestor_record_offset":31664,"inlinee":25630,"source_line":266,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$45::slice_contains::closure$1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":453,"record_offset":38152,"parent_record_offset":38128,"ancestor_record_offset":31664,"inlinee":25632,"source_line":278,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":453,"record_offset":36616,"parent_record_offset":36584,"ancestor_record_offset":31664,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_noubcheck
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":453,"record_offset":36684,"parent_record_offset":36656,"ancestor_record_offset":31664,"inlinee":4164,"source_line":87,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":453,"record_offset":33448,"parent_record_offset":33416,"ancestor_record_offset":31664,"inlinee":4360,"source_line":15,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":453,"record_offset":36656,"parent_record_offset":36616,"ancestor_record_offset":31664,"inlinee":4145,"source_line":226,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":453,"record_offset":33480,"parent_record_offset":33448,"ancestor_record_offset":31664,"inlinee":4640,"source_line":272,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":453,"record_offset":35716,"parent_record_offset":35692,"ancestor_record_offset":31664,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":453,"record_offset":38128,"parent_record_offset":38104,"ancestor_record_offset":31664,"inlinee":4402,"source_line":279,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::DecoratorCollision::dims_wall_space
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":453,"record_offset":32356,"parent_record_offset":32236,"ancestor_record_offset":31664,"inlinee":25150,"source_line":118,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary::dims_wall_space
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":453,"record_offset":32236,"parent_record_offset":32192,"ancestor_record_offset":31664,"inlinee":25148,"source_line":241,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::impl$4::dims_wall_space::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":453,"record_offset":32296,"parent_record_offset":32264,"ancestor_record_offset":31664,"inlinee":25160,"source_line":244,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::contains
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":453,"record_offset":36948,"parent_record_offset":36536,"ancestor_record_offset":31664,"inlinee":4848,"source_line":392,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::contains_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":453,"record_offset":37024,"parent_record_offset":37000,"ancestor_record_offset":31664,"inlinee":4851,"source_line":408,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::get_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":453,"record_offset":37048,"parent_record_offset":37024,"ancestor_record_offset":31664,"inlinee":4852,"source_line":171,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::impl$2::contains::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":453,"record_offset":37000,"parent_record_offset":36976,"ancestor_record_offset":31664,"inlinee":4850,"source_line":395,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":453,"record_offset":34488,"parent_record_offset":34456,"ancestor_record_offset":31664,"inlinee":4749,"source_line":166,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":453,"record_offset":34456,"parent_record_offset":34424,"ancestor_record_offset":31664,"inlinee":4748,"source_line":396,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_recip
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":453,"record_offset":34424,"parent_record_offset":34400,"ancestor_record_offset":31664,"inlinee":4747,"source_line":415,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":453,"record_offset":34232,"parent_record_offset":34200,"ancestor_record_offset":31664,"inlinee":4895,"source_line":203,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":453,"record_offset":34200,"parent_record_offset":34168,"ancestor_record_offset":31664,"inlinee":4894,"source_line":451,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length_recip
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":453,"record_offset":34168,"parent_record_offset":34144,"ancestor_record_offset":31664,"inlinee":4893,"source_line":470,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":453,"record_offset":33688,"parent_record_offset":33660,"ancestor_record_offset":31664,"inlinee":24403,"source_line":759,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::normalize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":453,"record_offset":34144,"parent_record_offset":34120,"ancestor_record_offset":31664,"inlinee":13657,"source_line":523,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":453,"record_offset":33716,"parent_record_offset":33688,"ancestor_record_offset":31664,"inlinee":4590,"source_line":1245,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":453,"record_offset":34028,"parent_record_offset":33900,"ancestor_record_offset":31664,"inlinee":4743,"source_line":1473,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nonmax","version":"0.5.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":453,"record_offset":37716,"parent_record_offset":37692,"ancestor_record_offset":31664,"inlinee":4171,"source_line":153,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":453,"record_offset":34264,"parent_record_offset":34232,"ancestor_record_offset":31664,"inlinee":4750,"source_line":369,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::cull_oob_decorators::is_wall_decorator_location_valid::closure$3
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/cull_oob_decorators.rs
//     evidence={"module":453,"record_offset":36480,"parent_record_offset":36448,"ancestor_record_offset":31664,"inlinee":25628,"source_line":963,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::get_coord_at_2d_ws_distance
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":453,"record_offset":32552,"parent_record_offset":32484,"ancestor_record_offset":31664,"inlinee":25132,"source_line":1161,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::get_unnormalized_tangent_at_coord
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":453,"record_offset":33900,"parent_record_offset":33872,"ancestor_record_offset":31664,"inlinee":24691,"source_line":812,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i1
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":453,"record_offset":33524,"parent_record_offset":33384,"ancestor_record_offset":31664,"inlinee":4641,"source_line":1337,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::CurveSlice::end
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":453,"record_offset":32732,"parent_record_offset":32552,"ancestor_record_offset":31664,"inlinee":24439,"source_line":338,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::CurveSlice::extend_right
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":453,"record_offset":32704,"parent_record_offset":32552,"ancestor_record_offset":31664,"inlinee":24465,"source_line":236,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::CurveSlice::from_curve_range
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":453,"record_offset":32588,"parent_record_offset":32552,"ancestor_record_offset":31664,"inlinee":25128,"source_line":89,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::CurveSlice::from_point_range
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":453,"record_offset":32616,"parent_record_offset":32588,"ancestor_record_offset":31664,"inlinee":24435,"source_line":110,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::CurveSlice::remap
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":453,"record_offset":32672,"parent_record_offset":32648,"ancestor_record_offset":31664,"inlinee":24464,"source_line":196,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::CurveSlice::remap_2d
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":453,"record_offset":32648,"parent_record_offset":32552,"ancestor_record_offset":31664,"inlinee":24463,"source_line":220,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle3d::height
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":453,"record_offset":32408,"parent_record_offset":32356,"ancestor_record_offset":31664,"inlinee":10294,"source_line":79,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle3d::width
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":453,"record_offset":32380,"parent_record_offset":32356,"ancestor_record_offset":31664,"inlinee":10292,"source_line":84,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$1::lerp
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":453,"record_offset":33660,"parent_record_offset":33384,"ancestor_record_offset":31664,"inlinee":24442,"source_line":109,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$1::normalize
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":453,"record_offset":34120,"parent_record_offset":33872,"ancestor_record_offset":31664,"inlinee":24692,"source_line":99,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] utils::wall_space::WallSpaceCoord::subtract
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/wall_space.rs
//     evidence={"module":453,"record_offset":36176,"parent_record_offset":36052,"ancestor_record_offset":31664,"inlinee":25627,"source_line":284,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] utils::wall_space::WallSpaceCoord::with_offset_clamped
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/wall_space.rs
//     evidence={"module":453,"record_offset":32520,"parent_record_offset":32484,"ancestor_record_offset":31664,"inlinee":25624,"source_line":257,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::cull_oob_decorators::is_wall_decorator_location_valid::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=5
// 源码行: 728
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6417
// 首条位置: {"module":453,"inlinee":25623}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":453,"record_offset":32764,"parent_record_offset":32484,"ancestor_record_offset":32484,"inlinee":4551,"source_line":890,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::get_coord_at_2d_ws_distance
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":453,"record_offset":32552,"parent_record_offset":32484,"ancestor_record_offset":32484,"inlinee":25132,"source_line":1161,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_direct] utils::wall_space::WallSpaceCoord::with_offset_clamped
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/wall_space.rs
//     evidence={"module":453,"record_offset":32520,"parent_record_offset":32484,"ancestor_record_offset":32484,"inlinee":25624,"source_line":257,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":453,"record_offset":35208,"parent_record_offset":35176,"ancestor_record_offset":35092,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::CurveSlice::end
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":453,"record_offset":32732,"parent_record_offset":32552,"ancestor_record_offset":32484,"inlinee":24439,"source_line":338,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::CurveSlice::extend_right
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":453,"record_offset":32704,"parent_record_offset":32552,"ancestor_record_offset":32484,"inlinee":24465,"source_line":236,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::CurveSlice::from_curve_range
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":453,"record_offset":32588,"parent_record_offset":32552,"ancestor_record_offset":32484,"inlinee":25128,"source_line":89,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::CurveSlice::from_point_range
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":453,"record_offset":32616,"parent_record_offset":32588,"ancestor_record_offset":32484,"inlinee":24435,"source_line":110,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::CurveSlice::remap
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":453,"record_offset":32672,"parent_record_offset":32648,"ancestor_record_offset":32484,"inlinee":24464,"source_line":196,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::CurveSlice::remap_2d
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":453,"record_offset":32648,"parent_record_offset":32552,"ancestor_record_offset":32484,"inlinee":24463,"source_line":220,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::cull_oob_decorators::is_wall_decorator_location_valid::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=7
// 源码行: 849
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1172F
// 首条位置: {"module":1510,"inlinee":71471}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] utils::curve::Curve::get_pos_at_coord
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1510,"record_offset":5140,"parent_record_offset":5108,"ancestor_record_offset":5108,"inlinee":9575,"source_line":799,"archive":"_pdb/B/modules/01510.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$13::index
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1510,"record_offset":5172,"parent_record_offset":5140,"ancestor_record_offset":5108,"inlinee":4358,"source_line":3357,"archive":"_pdb/B/modules/01510.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1510,"record_offset":5648,"parent_record_offset":5620,"ancestor_record_offset":5108,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/01510.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1510,"record_offset":5504,"parent_record_offset":5472,"ancestor_record_offset":5108,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/01510.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1510,"record_offset":5204,"parent_record_offset":5172,"ancestor_record_offset":5108,"inlinee":4360,"source_line":15,"archive":"_pdb/B/modules/01510.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1510,"record_offset":5236,"parent_record_offset":5204,"ancestor_record_offset":5108,"inlinee":4640,"source_line":272,"archive":"_pdb/B/modules/01510.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::lerp
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1510,"record_offset":5444,"parent_record_offset":5416,"ancestor_record_offset":5108,"inlinee":24403,"source_line":759,"archive":"_pdb/B/modules/01510.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1510,"record_offset":5472,"parent_record_offset":5444,"ancestor_record_offset":5108,"inlinee":4590,"source_line":1245,"archive":"_pdb/B/modules/01510.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1510,"record_offset":5620,"parent_record_offset":5444,"ancestor_record_offset":5108,"inlinee":4752,"source_line":1333,"archive":"_pdb/B/modules/01510.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i1
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1510,"record_offset":5280,"parent_record_offset":5140,"ancestor_record_offset":5108,"inlinee":4641,"source_line":1337,"archive":"_pdb/B/modules/01510.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$1::lerp
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":1510,"record_offset":5416,"parent_record_offset":5140,"ancestor_record_offset":5108,"inlinee":24442,"source_line":109,"archive":"_pdb/B/modules/01510.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::cull_oob_decorators::is_wall_decorator_location_valid::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=6
// 源码行: 851
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x118AA
// 首条位置: {"module":1552,"inlinee":71850}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec2::Vec2::normalize
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1552,"record_offset":7132,"parent_record_offset":7052,"ancestor_record_offset":7052,"inlinee":10290,"source_line":466,"archive":"_pdb/B/modules/01552.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1552,"record_offset":7076,"parent_record_offset":7052,"ancestor_record_offset":7052,"inlinee":4743,"source_line":1473,"archive":"_pdb/B/modules/01552.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1552,"record_offset":7284,"parent_record_offset":7156,"ancestor_record_offset":7052,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/01552.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1552,"record_offset":7100,"parent_record_offset":7076,"ancestor_record_offset":7052,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/01552.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1552,"record_offset":7216,"parent_record_offset":7184,"ancestor_record_offset":7052,"inlinee":4749,"source_line":166,"archive":"_pdb/B/modules/01552.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1552,"record_offset":7184,"parent_record_offset":7156,"ancestor_record_offset":7052,"inlinee":4748,"source_line":396,"archive":"_pdb/B/modules/01552.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_recip
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1552,"record_offset":7156,"parent_record_offset":7132,"ancestor_record_offset":7052,"inlinee":4747,"source_line":415,"archive":"_pdb/B/modules/01552.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":1552,"record_offset":7248,"parent_record_offset":7216,"ancestor_record_offset":7052,"inlinee":4750,"source_line":369,"archive":"_pdb/B/modules/01552.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::cull_oob_decorators::is_wall_decorator_location_valid::closure$3
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 963
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x641C
// 首条位置: {"module":453,"inlinee":25628}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] bevy_ecs::system::query::Query::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":453,"record_offset":36508,"parent_record_offset":36480,"ancestor_record_offset":36480,"inlinee":4837,"source_line":1385,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":453,"record_offset":36764,"parent_record_offset":36740,"ancestor_record_offset":36480,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":453,"record_offset":36812,"parent_record_offset":36788,"ancestor_record_offset":36480,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":453,"record_offset":36788,"parent_record_offset":36764,"ancestor_record_offset":36480,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":453,"record_offset":36740,"parent_record_offset":36716,"ancestor_record_offset":36480,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":453,"record_offset":36716,"parent_record_offset":36616,"ancestor_record_offset":36480,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":453,"record_offset":37152,"parent_record_offset":37128,"ancestor_record_offset":36480,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::impl$25::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":453,"record_offset":36916,"parent_record_offset":36584,"ancestor_record_offset":36480,"inlinee":4846,"source_line":76,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::Entities::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/entity/mod.rs
//     evidence={"module":453,"record_offset":36584,"parent_record_offset":36536,"ancestor_record_offset":36480,"inlinee":4840,"source_line":842,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::StorageSwitch::extract
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":453,"record_offset":37908,"parent_record_offset":37884,"ancestor_record_offset":36480,"inlinee":4876,"source_line":2472,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$26::set_archetype
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":453,"record_offset":37388,"parent_record_offset":36536,"ancestor_record_offset":36480,"inlinee":4856,"source_line":1160,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$26::set_table
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":453,"record_offset":37412,"parent_record_offset":37388,"ancestor_record_offset":36480,"inlinee":4857,"source_line":1175,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$27::fetch
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":453,"record_offset":37884,"parent_record_offset":36536,"ancestor_record_offset":36480,"inlinee":4874,"source_line":1229,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$27::fetch::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":453,"record_offset":37932,"parent_record_offset":37908,"ancestor_record_offset":36480,"inlinee":4878,"source_line":1235,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_array::BlobArray::get_sub_slice
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_array.rs
//     evidence={"module":453,"record_offset":37832,"parent_record_offset":37808,"ancestor_record_offset":36480,"inlinee":4872,"source_line":143,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseArray::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":453,"record_offset":37512,"parent_record_offset":37488,"ancestor_record_offset":36480,"inlinee":4864,"source_line":58,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":453,"record_offset":37488,"parent_record_offset":37464,"ancestor_record_offset":36480,"inlinee":4862,"source_line":395,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$16::get::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":453,"record_offset":37692,"parent_record_offset":37668,"ancestor_record_offset":36480,"inlinee":4866,"source_line":396,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_column
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":453,"record_offset":37464,"parent_record_offset":37436,"ancestor_record_offset":36480,"inlinee":4860,"source_line":475,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_data_slice_for
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":453,"record_offset":37436,"parent_record_offset":37412,"ancestor_record_offset":36480,"inlinee":4859,"source_line":363,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":453,"record_offset":37128,"parent_record_offset":36536,"ancestor_record_offset":36480,"inlinee":4854,"source_line":721,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::column::ThinColumn::get_data_slice
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/column.rs
//     evidence={"module":453,"record_offset":37808,"parent_record_offset":37784,"ancestor_record_offset":36480,"inlinee":4870,"source_line":303,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::impl$4::get_data_slice_for::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":453,"record_offset":37784,"parent_record_offset":37760,"ancestor_record_offset":36480,"inlinee":4868,"source_line":368,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::get_inner
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":453,"record_offset":36536,"parent_record_offset":36508,"ancestor_record_offset":36480,"inlinee":4838,"source_line":1591,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ptr::ThinSlicePtr::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ptr","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ptr-0.16.0/src/lib.rs
//     evidence={"module":453,"record_offset":37956,"parent_record_offset":37932,"ancestor_record_offset":36480,"inlinee":4880,"source_line":526,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::bool::impl$0::then
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\bool.rs
//     evidence={"module":453,"record_offset":36976,"parent_record_offset":36948,"ancestor_record_offset":36480,"inlinee":4781,"source_line":61,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$24::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":453,"record_offset":36884,"parent_record_offset":36860,"ancestor_record_offset":36480,"inlinee":4844,"source_line":1742,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::impl$2::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":453,"record_offset":36860,"parent_record_offset":36584,"ancestor_record_offset":36480,"inlinee":4842,"source_line":200,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":453,"record_offset":37604,"parent_record_offset":37540,"ancestor_record_offset":36480,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":453,"record_offset":37628,"parent_record_offset":37604,"ancestor_record_offset":36480,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":453,"record_offset":37668,"parent_record_offset":37488,"ancestor_record_offset":36480,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":453,"record_offset":37080,"parent_record_offset":37048,"ancestor_record_offset":36480,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":453,"record_offset":36616,"parent_record_offset":36584,"ancestor_record_offset":36480,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_noubcheck
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":453,"record_offset":36684,"parent_record_offset":36656,"ancestor_record_offset":36480,"inlinee":4164,"source_line":87,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":453,"record_offset":36656,"parent_record_offset":36616,"ancestor_record_offset":36480,"inlinee":4145,"source_line":226,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::contains
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":453,"record_offset":36948,"parent_record_offset":36536,"ancestor_record_offset":36480,"inlinee":4848,"source_line":392,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::contains_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":453,"record_offset":37024,"parent_record_offset":37000,"ancestor_record_offset":36480,"inlinee":4851,"source_line":408,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::get_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":453,"record_offset":37048,"parent_record_offset":37024,"ancestor_record_offset":36480,"inlinee":4852,"source_line":171,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::impl$2::contains::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":453,"record_offset":37000,"parent_record_offset":36976,"ancestor_record_offset":36480,"inlinee":4850,"source_line":395,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nonmax","version":"0.5.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":453,"record_offset":37716,"parent_record_offset":37692,"ancestor_record_offset":36480,"inlinee":4171,"source_line":153,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::cull_oob_decorators::check_if_dormer_ridge_is_within_roof_bounds::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 1112
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6422
// 首条位置: {"module":453,"inlinee":25634}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] bevy_ecs::system::query::Query::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":453,"record_offset":43512,"parent_record_offset":43488,"ancestor_record_offset":43488,"inlinee":4837,"source_line":1385,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":453,"record_offset":43760,"parent_record_offset":43736,"ancestor_record_offset":43488,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":453,"record_offset":43808,"parent_record_offset":43784,"ancestor_record_offset":43488,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":453,"record_offset":43784,"parent_record_offset":43760,"ancestor_record_offset":43488,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":453,"record_offset":43736,"parent_record_offset":43712,"ancestor_record_offset":43488,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":453,"record_offset":43712,"parent_record_offset":43612,"ancestor_record_offset":43488,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":453,"record_offset":44148,"parent_record_offset":44124,"ancestor_record_offset":43488,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::impl$25::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":453,"record_offset":43912,"parent_record_offset":43580,"ancestor_record_offset":43488,"inlinee":4846,"source_line":76,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::Entities::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/entity/mod.rs
//     evidence={"module":453,"record_offset":43580,"parent_record_offset":43536,"ancestor_record_offset":43488,"inlinee":4840,"source_line":842,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::StorageSwitch::extract
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":453,"record_offset":44860,"parent_record_offset":44836,"ancestor_record_offset":43488,"inlinee":4876,"source_line":2472,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$26::set_archetype
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":453,"record_offset":44384,"parent_record_offset":43536,"ancestor_record_offset":43488,"inlinee":4856,"source_line":1160,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$26::set_table
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":453,"record_offset":44408,"parent_record_offset":44384,"ancestor_record_offset":43488,"inlinee":4857,"source_line":1175,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$27::fetch
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":453,"record_offset":44836,"parent_record_offset":43536,"ancestor_record_offset":43488,"inlinee":4874,"source_line":1229,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$27::fetch::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":453,"record_offset":44884,"parent_record_offset":44860,"ancestor_record_offset":43488,"inlinee":4878,"source_line":1235,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_array::BlobArray::get_sub_slice
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_array.rs
//     evidence={"module":453,"record_offset":44784,"parent_record_offset":44760,"ancestor_record_offset":43488,"inlinee":4872,"source_line":143,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseArray::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":453,"record_offset":44508,"parent_record_offset":44484,"ancestor_record_offset":43488,"inlinee":4864,"source_line":58,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":453,"record_offset":44484,"parent_record_offset":44460,"ancestor_record_offset":43488,"inlinee":4862,"source_line":395,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$16::get::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":453,"record_offset":44644,"parent_record_offset":44620,"ancestor_record_offset":43488,"inlinee":4866,"source_line":396,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_column
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":453,"record_offset":44460,"parent_record_offset":44432,"ancestor_record_offset":43488,"inlinee":4860,"source_line":475,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_data_slice_for
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":453,"record_offset":44432,"parent_record_offset":44408,"ancestor_record_offset":43488,"inlinee":4859,"source_line":363,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":453,"record_offset":44124,"parent_record_offset":43536,"ancestor_record_offset":43488,"inlinee":4854,"source_line":721,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::column::ThinColumn::get_data_slice
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/column.rs
//     evidence={"module":453,"record_offset":44760,"parent_record_offset":44736,"ancestor_record_offset":43488,"inlinee":4870,"source_line":303,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::impl$4::get_data_slice_for::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":453,"record_offset":44736,"parent_record_offset":44712,"ancestor_record_offset":43488,"inlinee":4868,"source_line":368,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::get_inner
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":453,"record_offset":43536,"parent_record_offset":43512,"ancestor_record_offset":43488,"inlinee":4838,"source_line":1591,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ptr::ThinSlicePtr::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ptr","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ptr-0.16.0/src/lib.rs
//     evidence={"module":453,"record_offset":44908,"parent_record_offset":44884,"ancestor_record_offset":43488,"inlinee":4880,"source_line":526,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::bool::impl$0::then
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\bool.rs
//     evidence={"module":453,"record_offset":43972,"parent_record_offset":43944,"ancestor_record_offset":43488,"inlinee":4781,"source_line":61,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$24::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":453,"record_offset":43880,"parent_record_offset":43856,"ancestor_record_offset":43488,"inlinee":4844,"source_line":1742,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::impl$2::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":453,"record_offset":43856,"parent_record_offset":43580,"ancestor_record_offset":43488,"inlinee":4842,"source_line":200,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":453,"record_offset":44556,"parent_record_offset":44532,"ancestor_record_offset":43488,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":453,"record_offset":44532,"parent_record_offset":44508,"ancestor_record_offset":43488,"inlinee":4518,"source_line":1447,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":453,"record_offset":44580,"parent_record_offset":44556,"ancestor_record_offset":43488,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":453,"record_offset":44620,"parent_record_offset":44484,"ancestor_record_offset":43488,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":453,"record_offset":44076,"parent_record_offset":44044,"ancestor_record_offset":43488,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":453,"record_offset":43612,"parent_record_offset":43580,"ancestor_record_offset":43488,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_noubcheck
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":453,"record_offset":43680,"parent_record_offset":43652,"ancestor_record_offset":43488,"inlinee":4164,"source_line":87,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":453,"record_offset":43652,"parent_record_offset":43612,"ancestor_record_offset":43488,"inlinee":4145,"source_line":226,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::contains
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":453,"record_offset":43944,"parent_record_offset":43536,"ancestor_record_offset":43488,"inlinee":4848,"source_line":392,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::contains_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":453,"record_offset":44020,"parent_record_offset":43996,"ancestor_record_offset":43488,"inlinee":4851,"source_line":408,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::get_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":453,"record_offset":44044,"parent_record_offset":44020,"ancestor_record_offset":43488,"inlinee":4852,"source_line":171,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::impl$2::contains::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":453,"record_offset":43996,"parent_record_offset":43972,"ancestor_record_offset":43488,"inlinee":4850,"source_line":395,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nonmax","version":"0.5.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":453,"record_offset":44668,"parent_record_offset":44644,"ancestor_record_offset":43488,"inlinee":4171,"source_line":153,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::cull_oob_decorators::should_snap_to_corner
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=7
// 源码行: 1168
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x641A
// 首条位置: {"module":453,"inlinee":25626}
// 编译实例 RVAs: 0x811900
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1408c5670
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: cull_oob_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t736542","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377169664,"raw_hex":"e81e3d0b00","size":5,"soft_fail":false,"target_va":5377906288,"va":5377169741,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c91f60
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: cull_oob_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4720080","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377169664,"raw_hex":"e8d0054800","size":5,"soft_fail":false,"target_va":5381889888,"va":5377169803,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator::DecoratorType::should_offer_snap_to_corner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t137361","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377169664,"raw_hex":"e891180200","size":5,"soft_fail":false,"target_va":5377307072,"va":5377169706,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_wall_state::PublicWallState::wall_space
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3247562","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377169664,"raw_hex":"e8ca8d3100","size":5,"soft_fail":false,"target_va":5380417344,"va":5377169777,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::f32::impl$0::abs
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":453,"record_offset":36204,"parent_record_offset":36052,"ancestor_record_offset":36052,"inlinee":10511,"source_line":1446,"archive":"_pdb/B/modules/00453.jsonl.gz"}
//   [pdb_inline_direct] utils::wall_space::WallSpaceCoord::subtract
//     owner=private_source_ownership_candidate; count=8
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/wall_space.rs
//     evidence={"module":453,"record_offset":36176,"parent_record_offset":36052,"ancestor_record_offset":36052,"inlinee":25627,"source_line":284,"archive":"_pdb/B/modules/00453.jsonl.gz"}
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
// [PDB 泛型关系，非字段关系] {"type": "std::collections::hash::map::HashMap<country_core::resources::walls::decorator::DecoratorLocator,system_decorator::cull_oob_decorators::cull_oob_decorators::DecoratorRemoveInfo,std::hash::random::RandomState>", "constructor": "std::collections::hash::map::HashMap", "arguments": ["country_core::resources::walls::decorator::DecoratorLocator", "system_decorator::cull_oob_decorators::cull_oob_decorators::DecoratorRemoveInfo", "std::hash::random::RandomState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<std::collections::hash::map::HashMap<country_core::resources::walls::decorator::DecoratorLocator,system_decorator::cull_oob_decorators::cull_oob_decorators::DecoratorRemoveInfo,std::hash::random::RandomState> >", "constructor": "core::ptr::drop_in_place", "arguments": ["std::collections::hash::map::HashMap<country_core::resources::walls::decorator::DecoratorLocator,system_decorator::cull_oob_decorators::cull_oob_decorators::DecoratorRemoveInfo,std::hash::random::RandomState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<country_core::resources::walls::decorator::DecoratorLocator,system_decorator::cull_oob_decorators::cull_oob_decorators::DecoratorRemoveInfo>", "constructor": "tuple$", "arguments": ["country_core::resources::walls::decorator::DecoratorLocator", "system_decorator::cull_oob_decorators::cull_oob_decorators::DecoratorRemoveInfo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::make_hasher::closure_env$0<country_core::resources::walls::decorator::DecoratorLocator,system_decorator::cull_oob_decorators::cull_oob_decorators::DecoratorRemoveInfo,std::hash::random::RandomState>", "constructor": "hashbrown::map::make_hasher::closure_env$0", "arguments": ["country_core::resources::walls::decorator::DecoratorLocator", "system_decorator::cull_oob_decorators::cull_oob_decorators::DecoratorRemoveInfo", "std::hash::random::RandomState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTable::reserve_rehash<tuple$<country_core::resources::walls::decorator::DecoratorLocator,system_decorator::cull_oob_decorators::cull_oob_decorators::DecoratorRemoveInfo>,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<country_core::resources::walls::decorator::DecoratorLocator,system_decorator::cull_oob_decorators::cull_oob_decorators::DecoratorRemoveInfo,std::hash::random::RandomState> >", "constructor": "hashbrown::raw::RawTable::reserve_rehash", "arguments": ["tuple$<country_core::resources::walls::decorator::DecoratorLocator,system_decorator::cull_oob_decorators::cull_oob_decorators::DecoratorRemoveInfo>", "alloc::alloc::Global", "hashbrown::map::make_hasher::closure_env$0<country_core::resources::walls::decorator::DecoratorLocator,system_decorator::cull_oob_decorators::cull_oob_decorators::DecoratorRemoveInfo,std::hash::random::RandomState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::HashMap::insert<country_core::resources::walls::decorator::DecoratorLocator,system_decorator::cull_oob_decorators::cull_oob_decorators::DecoratorRemoveInfo,std::hash::random::RandomState,alloc::alloc::Global>", "constructor": "hashbrown::map::HashMap::insert", "arguments": ["country_core::resources::walls::decorator::DecoratorLocator", "system_decorator::cull_oob_decorators::cull_oob_decorators::DecoratorRemoveInfo", "std::hash::random::RandomState", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] system_decorator::cull_oob_decorators::cull_stair_attached_decorators_impl::DecoratorRemoveInfo
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 1
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::HashMap::insert<country_core::resources::walls::decorator::DecoratorLocator,system_decorator::cull_oob_decorators::cull_stair_attached_decorators_impl::DecoratorRemoveInfo,std::hash::random::RandomState,alloc::alloc::Global>", "constructor": "hashbrown::map::HashMap::insert", "arguments": ["country_core::resources::walls::decorator::DecoratorLocator", "system_decorator::cull_oob_decorators::cull_stair_attached_decorators_impl::DecoratorRemoveInfo", "std::hash::random::RandomState", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
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
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 8; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] country_core::resources::walls::WallId (*)(ref$<country_core::resources::walls::WallId>)
// 候选参数类型: ["ref$<country_core::resources::walls::WallId>"]
// [文件级接口候选，具体函数归属 unknown] glam::f32::vec2::Vec2 (*)(glam::f32::vec3::Vec3)
// 候选参数类型: ["glam::f32::vec3::Vec3"]
// [文件级接口候选，具体函数归属 unknown] glam::f32::vec2::Vec2 (*)(glam::f32::vec2::Vec2)
// 候选参数类型: ["glam::f32::vec2::Vec2"]
// [文件级接口候选，具体函数归属 unknown] glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)
// 候选参数类型: ["usize", "ref$<glam::f32::vec3::Vec3>"]
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::event::reader::EventReader<country_core::systems::wall::adjust_wall_height::adjust_wall_height_impl::AdjustWallHeightCmd>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::history::History>", "bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>", "bevy_ecs::event::reader::EventReader<country_core::systems::wall::adjust_wall_height::adjust_wall_height_impl::AdjustWallHeightCmd>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::history::History>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::systems::wall::adjust_wall_height::adjust_wall_height_impl::AdjustWallHeightCmd>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator::DeferredDecoratorOpEvents>,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::event::reader::EventReader<country_core::systems::wall::adjust_wall_height::adjust_wall_height_impl::AdjustWallHeightCmd>,bevy_ecs::event::reader::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>,bevy_ecs::event::reader::EventReader<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::change_detection::Res<country_core::resources::history::History>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>", "bevy_ecs::change_detection::Res<country_core::resources::walls::decorator::DeferredDecoratorOpEvents>", "bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>", "bevy_ecs::event::reader::EventReader<country_core::systems::wall::adjust_wall_height::adjust_wall_height_impl::AdjustWallHeightCmd>", "bevy_ecs::event::reader::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>", "bevy_ecs::event::reader::EventReader<country_core::resources::roofs::commands::EditRoofCmd>", "bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >", "country_core::resources::walls::decorator_archivist::DecoratorArchivalResources", "country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources", "bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>", "bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::history::History>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::decorator::DeferredDecoratorOpEvents>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::systems::wall::adjust_wall_height::adjust_wall_height_impl::AdjustWallHeightCmd>
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::resources::roofs::commands::EditRoofCmd>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator::DeferredDecoratorOpEvents>,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_state::StairsState>,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::history::History>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>", "bevy_ecs::change_detection::Res<country_core::resources::walls::decorator::DeferredDecoratorOpEvents>", "bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>", "bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_state::StairsState>", "country_core::resources::walls::decorator_archivist::DecoratorArchivalResources", "bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::history::History>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::decorator::DeferredDecoratorOpEvents>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_state::StairsState>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator::DeferredDecoratorOpEvents>,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyRequests>,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::history::History>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>", "bevy_ecs::change_detection::Res<country_core::resources::walls::decorator::DeferredDecoratorOpEvents>", "bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>", "bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyRequests>", "country_core::resources::walls::decorator_archivist::DecoratorArchivalResources", "bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::history::History>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::decorator::DeferredDecoratorOpEvents>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyRequests>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>

// ------------------------------------------------------------------
// 函数: system_decorator::cull_oob_decorators::DecoratorLocationCheckResult::get_invalid_reason
// 信息 ID: 4ba68a5962a4ac9cb1200163
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::iter::adapters::copied::impl$1::try_fold<std::collections::hash::set::Iter<country_core::resources::walls::decorator::DecoratorLocator>,country_core::resources::walls::decorator::DecoratorLocator,tuple$<>,core::iter::traits::iterator::Iterator::all::check::closure_env$0<country_core::resources::walls::decorator::DecoratorLocator,system_decorator::blueprint::validate_blueprints::closure_env$0>,enum2$<core::ops::control_flow::ControlFlow<tuple$<>,tuple$<> > > >; module=1576, inline_record=9872; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::cull_oob_decorators::DecoratorLocationCheckResult::is_valid
// 信息 ID: bf6813192e987b506d214567
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::cull_oob_decorators::cull_oob_decorators; module=453, inline_record=17080; 不是独立调用边。
// [PDB 内联宿主] core::iter::adapters::copied::impl$1::try_fold<std::collections::hash::set::Iter<country_core::resources::walls::decorator::DecoratorLocator>,country_core::resources::walls::decorator::DecoratorLocator,tuple$<>,core::iter::traits::iterator::Iterator::all::check::closure_env$0<country_core::resources::walls::decorator::DecoratorLocator,system_decorator::blueprint::validate_blueprints::closure_env$0>,enum2$<core::ops::control_flow::ControlFlow<tuple$<>,tuple$<> > > >; module=1576, inline_record=9844; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::cull_oob_decorators::auto_allow_decorator_culling
// 信息 ID: 1760baffb27d2a1136e798c5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] af0f2186490ec4108332a0f9: void (*)(bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::event::reader::EventReader<country_core::systems::wall::adjust_wall_height::adjust_wall_height_impl::AdjustWallHeightCmd>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::schedule_system::impl$1::run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::event::reader::EventReader<country_core::systems::wall::adjust_wall_height::adjust_wall_height_impl::AdjustWallHeightCmd>),void (*)(bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::event::reader::EventReader<country_core::systems::wall::adjust_wall_height::adjust_wall_height_impl::AdjustWallHeightCmd>)> > @ 0x10370c -> 0x80d3d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::event::reader::EventReader<country_core::systems::wall::adjust_wall_height::adjust_wall_height_impl::AdjustWallHeightCmd>),void (*)(bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::event::reader::EventReader<country_core::systems::wall::adjust_wall_height::adjust_wall_height_impl::AdjustWallHeightCmd>)> > > @ 0x4475ee -> 0x80d3d0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::cull_oob_decorators::check_if_dormer_ridge_is_within_roof_bounds
// 信息 ID: 4f4b230b2df32c7af53bc8a8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::cull_oob_decorators::check_if_dormer_ridge_is_within_roof_bounds::closure$0
// 信息 ID: 28041baa57a3d5ae546ddd9c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_decorator::cull_oob_decorators::check_if_dormer_ridge_is_within_roof_bounds", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::cull_oob_decorators::check_if_dormer_ridge_is_within_roof_bounds; module=453, inline_record=43488; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::cull_oob_decorators::cull_oob_decorators
// 信息 ID: f7235d80cc9f40dab324ea96
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] fc775139467820dc9371399b: void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator::DeferredDecoratorOpEvents>,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::event::reader::EventReader<country_core::systems::wall::adjust_wall_height::adjust_wall_height_impl::AdjustWallHeightCmd>,bevy_ecs::event::reader::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>,bevy_ecs::event::reader::EventReader<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::schedule_system::impl$1::run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator::DeferredDecoratorOpEvents>,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::event::reader::EventReader<country_core::systems::wall::adjust_wall_height::adjust_wall_height_impl::AdjustWallHeightCmd>,bevy_ecs::event::reader::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>,bevy_ecs::event::reader::EventReader<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>),void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator::DeferredDecoratorOpEvents>,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::event::reader::EventReader<country_core::systems::wall::adjust_wall_height::adjust_wall_height_impl::AdjustWallHeightCmd>,bevy_ecs::event::reader::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>,bevy_ecs::event::reader::EventReader<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>)> > @ 0x1022ee -> 0x80d540 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator::DeferredDecoratorOpEvents>,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::event::reader::EventReader<country_core::systems::wall::adjust_wall_height::adjust_wall_height_impl::AdjustWallHeightCmd>,bevy_ecs::event::reader::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>,bevy_ecs::event::reader::EventReader<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>),void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator::DeferredDecoratorOpEvents>,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::event::reader::EventReader<country_core::systems::wall::adjust_wall_height::adjust_wall_height_impl::AdjustWallHeightCmd>,bevy_ecs::event::reader::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>,bevy_ecs::event::reader::EventReader<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>)> > > @ 0x44c4d9 -> 0x80d540 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::cull_oob_decorators::cull_oob_decorators::closure$0
// 信息 ID: e1082746465fda5879fb5c60
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 2ce84ca3267526f881835269: country_core::resources::walls::WallId (*)(ref$<country_core::resources::walls::WallId>)
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_decorator::cull_oob_decorators::cull_oob_decorators", "captures": "unknown"}
// [闭包环境类型引用] system_decorator::cull_oob_decorators::cull_oob_decorators::closure_env$0 <- hashbrown::raw::RawIterRange::fold_impl<tuple$<country_core::resources::walls::WallId,country_core::resources::walls::decorator_storage::Decorators>,hashbrown::map::impl$44::fold::closure_env$0<country_core::resources::walls::WallId,country_core::resources::walls::decorator_storage::Decorators,tuple$<>,std::collections::hash::map::impl$58::fold::closure_env$0<country_core::resources::walls::WallId,country_core::resources::walls::decorator_storage::Decorators,tuple$<>,core::iter::adapters::map::map_fold::closure_env$0<ref$<country_core::resources::walls::WallId>,country_core::resources::walls::WallId,tuple$<>,country_core::resources::walls::WallId (*)(ref$<country_core::resources::walls::WallId>),core::iter::adapters::filter::filter_fold::closure_env$0<country_core::resources::walls::WallId,tuple$<>,system_decorator::cull_oob_decorators::cull_oob_decorators::closure_env$0,core::iter::adapters::map::map_fold::closure_env$0<country_core::resources::walls::WallId,tuple$<country_core::resources::walls::WallId,tuple$<> >,tuple$<>,hashbrown::set::impl$10::extend::closure_env$0<country_core::resources::walls::WallId,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::filter::Filter<core::iter::adapters::cloned::Cloned<std::collections::hash::map::Keys<country_core::resources::walls::WallId,country_core::resources::walls::decorator_storage::Decorators> >,system_decorator::cull_oob_decorators::cull_oob_decorators::closure_env$0> >,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<country_core::resources::walls::WallId,tuple$<> >,hashbrown::map::impl$82::extend::closure_env$0<country_core::resources::walls::WallId,tuple$<>,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::filter::Filter<core::iter::adapters::cloned::Cloned<std::collections::hash::map::Keys<country_core::resources::walls::WallId,country_core::resources::walls::decorator_storage::Decorators> >,system_decorator::cull_oob_decorators::cull_oob_decorators::closure_env$0>,hashbrown::set::impl$10::extend::closure_env$0<country_core::resources::walls::WallId,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::filter::Filter<core::iter::adapters::cloned::Cloned<std::collections::hash::map::Keys<country_core::resources::walls::WallId,country_core::resources::walls::decorator_storage::Decorators> >,system_decorator::cull_oob_decorators::cull_oob_decorators::closure_env$0> > > > > > > > > >,tuple$<> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_decorator::cull_oob_decorators::cull_oob_decorators::closure_env$0 <- hashbrown::raw::RawIterRange::fold_impl<tuple$<country_core::resources::walls::WallId,country_core::resources::walls::decorator_storage::Decorators>,hashbrown::map::impl$44::fold::closure_env$0<country_core::resources::walls::WallId,country_core::resources::walls::decorator_storage::Decorators,tuple$<>,std::collections::hash::map::impl$58::fold::closure_env$0<country_core::resources::walls::WallId,country_core::resources::walls::decorator_storage::Decorators,tuple$<>,core::iter::adapters::map::map_fold::closure_env$0<ref$<country_core::resources::walls::WallId>,country_core::resources::walls::WallId,tuple$<>,country_core::resources::walls::WallId (*)(ref$<country_core::resources::walls::WallId>),core::iter::adapters::filter::filter_fold::closure_env$0<country_core::resources::walls::WallId,tuple$<>,system_decorator::cull_oob_decorators::cull_oob_decorators::closure_env$0,core::iter::adapters::map::map_fold::closure_env$0<country_core::resources::walls::WallId,tuple$<country_core::resources::walls::WallId,tuple$<> >,tuple$<>,hashbrown::set::impl$10::extend::closure_env$0<country_core::resources::walls::WallId,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::filter::Filter<core::iter::adapters::cloned::Cloned<std::collections::hash::map::Keys<country_core::resources::walls::WallId,country_core::resources::walls::decorator_storage::Decorators> >,system_decorator::cull_oob_decorators::cull_oob_decorators::closure_env$0> >,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<country_core::resources::walls::WallId,tuple$<> >,hashbrown::map::impl$82::extend::closure_env$0<country_core::resources::walls::WallId,tuple$<>,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::filter::Filter<core::iter::adapters::cloned::Cloned<std::collections::hash::map::Keys<country_core::resources::walls::WallId,country_core::resources::walls::decorator_storage::Decorators> >,system_decorator::cull_oob_decorators::cull_oob_decorators::closure_env$0>,hashbrown::set::impl$10::extend::closure_env$0<country_core::resources::walls::WallId,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::filter::Filter<core::iter::adapters::cloned::Cloned<std::collections::hash::map::Keys<country_core::resources::walls::WallId,country_core::resources::walls::decorator_storage::Decorators> >,system_decorator::cull_oob_decorators::cull_oob_decorators::closure_env$0> > > > > > > > > >,tuple$<> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_decorator::cull_oob_decorators::cull_oob_decorators::closure_env$0 <- hashbrown::raw::RawIterRange::fold_impl<tuple$<country_core::resources::walls::WallId,country_core::resources::walls::decorator_storage::Decorators>,hashbrown::map::impl$44::fold::closure_env$0<country_core::resources::walls::WallId,country_core::resources::walls::decorator_storage::Decorators,tuple$<>,std::collections::hash::map::impl$58::fold::closure_env$0<country_core::resources::walls::WallId,country_core::resources::walls::decorator_storage::Decorators,tuple$<>,core::iter::adapters::map::map_fold::closure_env$0<ref$<country_core::resources::walls::WallId>,country_core::resources::walls::WallId,tuple$<>,country_core::resources::walls::WallId (*)(ref$<country_core::resources::walls::WallId>),core::iter::adapters::filter::filter_fold::closure_env$0<country_core::resources::walls::WallId,tuple$<>,system_decorator::cull_oob_decorators::cull_oob_decorators::closure_env$0,core::iter::adapters::map::map_fold::closure_env$0<country_core::resources::walls::WallId,tuple$<country_core::resources::walls::WallId,tuple$<> >,tuple$<>,hashbrown::set::impl$10::extend::closure_env$0<country_core::resources::walls::WallId,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::filter::Filter<core::iter::adapters::cloned::Cloned<std::collections::hash::map::Keys<country_core::resources::walls::WallId,country_core::resources::walls::decorator_storage::Decorators> >,system_decorator::cull_oob_decorators::cull_oob_decorators::closure_env$0> >,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<country_core::resources::walls::WallId,tuple$<> >,hashbrown::map::impl$82::extend::closure_env$0<country_core::resources::walls::WallId,tuple$<>,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::filter::Filter<core::iter::adapters::cloned::Cloned<std::collections::hash::map::Keys<country_core::resources::walls::WallId,country_core::resources::walls::decorator_storage::Decorators> >,system_decorator::cull_oob_decorators::cull_oob_decorators::closure_env$0>,hashbrown::set::impl$10::extend::closure_env$0<country_core::resources::walls::WallId,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::filter::Filter<core::iter::adapters::cloned::Cloned<std::collections::hash::map::Keys<country_core::resources::walls::WallId,country_core::resources::walls::decorator_storage::Decorators> >,system_decorator::cull_oob_decorators::cull_oob_decorators::closure_env$0> > > > > > > > > >,tuple$<> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_decorator::cull_oob_decorators::cull_oob_decorators::closure_env$0 <- hashbrown::raw::RawIterRange::fold_impl<tuple$<country_core::resources::walls::WallId,country_core::resources::walls::decorator_storage::Decorators>,hashbrown::map::impl$44::fold::closure_env$0<country_core::resources::walls::WallId,country_core::resources::walls::decorator_storage::Decorators,tuple$<>,std::collections::hash::map::impl$58::fold::closure_env$0<country_core::resources::walls::WallId,country_core::resources::walls::decorator_storage::Decorators,tuple$<>,core::iter::adapters::map::map_fold::closure_env$0<ref$<country_core::resources::walls::WallId>,country_core::resources::walls::WallId,tuple$<>,country_core::resources::walls::WallId (*)(ref$<country_core::resources::walls::WallId>),core::iter::adapters::filter::filter_fold::closure_env$0<country_core::resources::walls::WallId,tuple$<>,system_decorator::cull_oob_decorators::cull_oob_decorators::closure_env$0,core::iter::adapters::map::map_fold::closure_env$0<country_core::resources::walls::WallId,tuple$<country_core::resources::walls::WallId,tuple$<> >,tuple$<>,hashbrown::set::impl$10::extend::closure_env$0<country_core::resources::walls::WallId,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::filter::Filter<core::iter::adapters::cloned::Cloned<std::collections::hash::map::Keys<country_core::resources::walls::WallId,country_core::resources::walls::decorator_storage::Decorators> >,system_decorator::cull_oob_decorators::cull_oob_decorators::closure_env$0> >,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<country_core::resources::walls::WallId,tuple$<> >,hashbrown::map::impl$82::extend::closure_env$0<country_core::resources::walls::WallId,tuple$<>,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::filter::Filter<core::iter::adapters::cloned::Cloned<std::collections::hash::map::Keys<country_core::resources::walls::WallId,country_core::resources::walls::decorator_storage::Decorators> >,system_decorator::cull_oob_decorators::cull_oob_decorators::closure_env$0>,hashbrown::set::impl$10::extend::closure_env$0<country_core::resources::walls::WallId,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::filter::Filter<core::iter::adapters::cloned::Cloned<std::collections::hash::map::Keys<country_core::resources::walls::WallId,country_core::resources::walls::decorator_storage::Decorators> >,system_decorator::cull_oob_decorators::cull_oob_decorators::closure_env$0> > > > > > > > > >,tuple$<> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 4 条。
// [PDB 内联宿主] hashbrown::raw::RawIterRange::fold_impl<tuple$<country_core::resources::walls::WallId,country_core::resources::walls::decorator_storage::Decorators>,hashbrown::map::impl$44::fold::closure_env$0<country_core::resources::walls::WallId,country_core::resources::walls::decorator_storage::Decorators,tuple$<>,std::collections::hash::map::impl$58::fold::closure_env$0<country_core::resources::walls::WallId,country_core::resources::walls::decorator_storage::Decorators,tuple$<>,core::iter::adapters::map::map_fold::closure_env$0<ref$<country_core::resources::walls::WallId>,country_core::resources::walls::WallId,tuple$<>,country_core::resources::walls::WallId (*)(ref$<country_core::resources::walls::WallId>),core::iter::adapters::filter::filter_fold::closure_env$0<country_core::resources::walls::WallId,tuple$<>,system_decorator::cull_oob_decorators::cull_oob_decorators::closure_env$0,core::iter::adapters::map::map_fold::closure_env$0<country_core::resources::walls::WallId,tuple$<country_core::resources::walls::WallId,tuple$<> >,tuple$<>,hashbrown::set::impl$10::extend::closure_env$0<country_core::resources::walls::WallId,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::filter::Filter<core::iter::adapters::cloned::Cloned<std::collections::hash::map::Keys<country_core::resources::walls::WallId,country_core::resources::walls::decorator_storage::Decorators> >,system_decorator::cull_oob_decorators::cull_oob_decorators::closure_env$0> >,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<country_core::resources::walls::WallId,tuple$<> >,hashbrown::map::impl$82::extend::closure_env$0<country_core::resources::walls::WallId,tuple$<>,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::filter::Filter<core::iter::adapters::cloned::Cloned<std::collections::hash::map::Keys<country_core::resources::walls::WallId,country_core::resources::walls::decorator_storage::Decorators> >,system_decorator::cull_oob_decorators::cull_oob_decorators::closure_env$0>,hashbrown::set::impl$10::extend::closure_env$0<country_core::resources::walls::WallId,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::filter::Filter<core::iter::adapters::cloned::Cloned<std::collections::hash::map::Keys<country_core::resources::walls::WallId,country_core::resources::walls::decorator_storage::Decorators> >,system_decorator::cull_oob_decorators::cull_oob_decorators::closure_env$0> > > > > > > > > >,tuple$<> >; module=1504, inline_record=51272; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::cull_oob_decorators::cull_oob_decorators::closure$1
// 信息 ID: 78013478f3b2798c90b26943
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "system_decorator::cull_oob_decorators::cull_oob_decorators", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::cull_oob_decorators::cull_oob_decorators; module=453, inline_record=18160; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::cull_oob_decorators::cull_stair_attached_decorators
// 信息 ID: 0f0ca5cd03e90ba5156ff591
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] f8fbec6b254876c7589a70dc: void (*)(bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator::DeferredDecoratorOpEvents>,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_state::StairsState>,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::schedule_system::impl$1::run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator::DeferredDecoratorOpEvents>,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_state::StairsState>,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>),void (*)(bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator::DeferredDecoratorOpEvents>,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_state::StairsState>,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>)> > @ 0x1032af -> 0x80ec10 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator::DeferredDecoratorOpEvents>,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_state::StairsState>,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>),void (*)(bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator::DeferredDecoratorOpEvents>,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_state::StairsState>,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>)> > > @ 0x467e89 -> 0x80ec10 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::cull_oob_decorators::cull_stair_attached_decorators::closure$0
// 信息 ID: e310213b52645e144c57711d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_decorator::cull_oob_decorators::cull_stair_attached_decorators", "captures": "unknown"}
// [闭包环境类型引用] system_decorator::cull_oob_decorators::cull_stair_attached_decorators::closure_env$0 <- system_decorator::cull_oob_decorators::cull_stair_attached_decorators_impl<system_decorator::cull_oob_decorators::cull_stair_attached_decorators::closure_env$0>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] system_decorator::cull_oob_decorators::cull_stair_attached_decorators_impl<system_decorator::cull_oob_decorators::cull_stair_attached_decorators::closure_env$0>; module=1570, inline_record=10004; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::cull_oob_decorators::cull_stair_attached_decorators_impl<system_decorator::cull_oob_decorators::cull_stair_attached_decorators::closure_env$0>
// 信息 ID: 0b5c7c2b9816797fc0a898a3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] system_decorator::cull_oob_decorators::cull_stair_attached_decorators_impl -> ["system_decorator::cull_oob_decorators::cull_stair_attached_decorators::closure_env$0"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::cull_oob_decorators::cull_stair_attached_decorators @ 0x80eeb5 -> 0x12b6c30 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::cull_oob_decorators::cull_stair_attached_decorators_impl<system_decorator::cull_oob_decorators::cull_stair_attached_decorators_on_invalid_stairs::closure_env$0>
// 信息 ID: 7eb8d2649c3b5242eb353fa3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] system_decorator::cull_oob_decorators::cull_stair_attached_decorators_impl -> ["system_decorator::cull_oob_decorators::cull_stair_attached_decorators_on_invalid_stairs::closure_env$0"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::cull_oob_decorators::cull_stair_attached_decorators_on_invalid_stairs @ 0x80f2e5 -> 0x12b6420 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::cull_oob_decorators::cull_stair_attached_decorators_on_invalid_stairs
// 信息 ID: d7f40b646221821f534b0a69
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] fa264df0c8e08bb0c427939d: void (*)(bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator::DeferredDecoratorOpEvents>,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyRequests>,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::schedule_system::impl$1::run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator::DeferredDecoratorOpEvents>,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyRequests>,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>),void (*)(bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator::DeferredDecoratorOpEvents>,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyRequests>,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>)> > @ 0x102b6f -> 0x80f040 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator::DeferredDecoratorOpEvents>,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyRequests>,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>),void (*)(bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator::DeferredDecoratorOpEvents>,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyRequests>,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>)> > > @ 0x47beb9 -> 0x80f040 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::cull_oob_decorators::cull_stair_attached_decorators_on_invalid_stairs::closure$0
// 信息 ID: d487dfe10086784ca2a8c18c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_decorator::cull_oob_decorators::cull_stair_attached_decorators_on_invalid_stairs", "captures": "unknown"}
// [闭包环境类型引用] system_decorator::cull_oob_decorators::cull_stair_attached_decorators_on_invalid_stairs::closure_env$0 <- system_decorator::cull_oob_decorators::cull_stair_attached_decorators_impl<system_decorator::cull_oob_decorators::cull_stair_attached_decorators_on_invalid_stairs::closure_env$0>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] system_decorator::cull_oob_decorators::cull_stair_attached_decorators_impl<system_decorator::cull_oob_decorators::cull_stair_attached_decorators_on_invalid_stairs::closure_env$0>; module=1570, inline_record=3892; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::cull_oob_decorators::cull_stair_attached_decorators_on_invalid_stairs::closure$0::closure$0
// 信息 ID: ec14e7e3cf6d3ab7a747f0c2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_decorator::cull_oob_decorators::cull_stair_attached_decorators_on_invalid_stairs", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::cull_oob_decorators::cull_stair_attached_decorators_impl<system_decorator::cull_oob_decorators::cull_stair_attached_decorators_on_invalid_stairs::closure_env$0>; module=1570, inline_record=4100; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::cull_oob_decorators::impl$1::clone
// 信息 ID: 56c337de4e2cc2802ef4a542
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::iter::adapters::copied::impl$1::try_fold<std::collections::hash::set::Iter<country_core::resources::walls::decorator::DecoratorLocator>,country_core::resources::walls::decorator::DecoratorLocator,tuple$<>,core::iter::traits::iterator::Iterator::all::check::closure_env$0<country_core::resources::walls::decorator::DecoratorLocator,system_decorator::blueprint::validate_blueprints::closure_env$0>,enum2$<core::ops::control_flow::ControlFlow<tuple$<>,tuple$<> > > >; module=1576, inline_record=9920; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::cull_oob_decorators::is_decorator_location_valid
// 信息 ID: bdec725960f442453d676fde
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::cull_oob_decorators::cull_oob_decorators @ 0x80e1ba -> 0x80f470 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::iter::adapters::copied::impl$1::try_fold<std::collections::hash::set::Iter<country_core::resources::walls::decorator::DecoratorLocator>,country_core::resources::walls::decorator::DecoratorLocator,tuple$<>,core::iter::traits::iterator::Iterator::all::check::closure_env$0<country_core::resources::walls::decorator::DecoratorLocator,system_decorator::blueprint::validate_blueprints::closure_env$0>,enum2$<core::ops::control_flow::ControlFlow<tuple$<>,tuple$<> > > > @ 0x12b8815 -> 0x80f470 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::cull_oob_decorators::is_wall_decorator_location_valid
// 信息 ID: 5d28f10e3e2291a91440cd3b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::cull_oob_decorators::is_decorator_location_valid; module=453, inline_record=31664; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::cull_oob_decorators::is_wall_decorator_location_valid::closure$0
// 信息 ID: d438e66b563b0556b2c9e6c5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_decorator::cull_oob_decorators::is_wall_decorator_location_valid", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::cull_oob_decorators::is_decorator_location_valid; module=453, inline_record=32484; 不是独立调用边。
// [PDB 内联宿主] system_decorator::cull_oob_decorators::is_decorator_location_valid; module=453, inline_record=32796; 不是独立调用边。
// [PDB 内联宿主] system_decorator::cull_oob_decorators::is_decorator_location_valid; module=453, inline_record=33104; 不是独立调用边。
// [PDB 内联宿主] system_decorator::cull_oob_decorators::is_decorator_location_valid; module=453, inline_record=35092; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::cull_oob_decorators::is_wall_decorator_location_valid::closure$1
// 信息 ID: f84e4353b95ffad348bb6f70
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "system_decorator::cull_oob_decorators::is_wall_decorator_location_valid", "captures": "unknown"}
// [闭包环境类型引用] system_decorator::cull_oob_decorators::is_wall_decorator_location_valid::closure_env$1 <- itertools::tuple_impl::impl$31::collect_from_iter_no_buf<glam::f32::vec3::Vec3,core::iter::adapters::chain::Chain<core::iter::sources::once::Once<glam::f32::vec3::Vec3>,ref_mut$<core::iter::adapters::map::Map<core::slice::iter::Iter<utils::curve::CurveInternalCoord<utils::curve::CurveCoordSemanticThresholded3D> >,system_decorator::cull_oob_decorators::is_wall_decorator_location_valid::closure_env$1> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_decorator::cull_oob_decorators::is_wall_decorator_location_valid::closure_env$1 <- itertools::tuple_impl::impl$7::next<core::iter::adapters::map::Map<core::slice::iter::Iter<utils::curve::CurveInternalCoord<utils::curve::CurveCoordSemanticThresholded3D> >,system_decorator::cull_oob_decorators::is_wall_decorator_location_valid::closure_env$1>,tuple$<glam::f32::vec3::Vec3,glam::f32::vec3::Vec3> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_decorator::cull_oob_decorators::is_wall_decorator_location_valid::closure_env$1 <- itertools::tuple_impl::impl$7::next<core::iter::adapters::map::Map<itertools::tuple_impl::TupleWindows<core::iter::adapters::map::Map<core::slice::iter::Iter<utils::curve::CurveInternalCoord<utils::curve::CurveCoordSemanticThresholded3D> >,system_decorator::cull_oob_decorators::is_wall_decorator_location_valid::closure_env$1>,tuple$<glam::f32::vec3::Vec3,glam::f32::vec3::Vec3> >,system_decorator::cull_oob_decorators::is_wall_decorator_location_valid::closure_env$2>,tuple$<glam::f32::vec2::Vec2,glam::f32::vec2::Vec2> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_decorator::cull_oob_decorators::is_wall_decorator_location_valid::closure_env$1 <- itertools::tuple_impl::impl$31::collect_from_iter_no_buf<glam::f32::vec2::Vec2,core::iter::adapters::chain::Chain<core::iter::sources::once::Once<glam::f32::vec2::Vec2>,ref_mut$<core::iter::adapters::map::Map<itertools::tuple_impl::TupleWindows<core::iter::adapters::map::Map<core::slice::iter::Iter<utils::curve::CurveInternalCoord<utils::curve::CurveCoordSemanticThresholded3D> >,system_decorator::cull_oob_decorators::is_wall_decorator_location_valid::closure_env$1>,tuple$<glam::f32::vec3::Vec3,glam::f32::vec3::Vec3> >,system_decorator::cull_oob_decorators::is_wall_decorator_location_valid::closure_env$2> > > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 4 条。
// [PDB 内联宿主] itertools::tuple_impl::impl$31::collect_from_iter_no_buf<glam::f32::vec3::Vec3,core::iter::adapters::chain::Chain<core::iter::sources::once::Once<glam::f32::vec3::Vec3>,ref_mut$<core::iter::adapters::map::Map<core::slice::iter::Iter<utils::curve::CurveInternalCoord<utils::curve::CurveCoordSemanticThresholded3D> >,system_decorator::cull_oob_decorators::is_wall_decorator_location_valid::closure_env$1> > > >; module=1510, inline_record=5108; 不是独立调用边。
// [PDB 内联宿主] itertools::tuple_impl::impl$31::collect_from_iter_no_buf<glam::f32::vec3::Vec3,core::iter::adapters::chain::Chain<core::iter::sources::once::Once<glam::f32::vec3::Vec3>,ref_mut$<core::iter::adapters::map::Map<core::slice::iter::Iter<utils::curve::CurveInternalCoord<utils::curve::CurveCoordSemanticThresholded3D> >,system_decorator::cull_oob_decorators::is_wall_decorator_location_valid::closure_env$1> > > >; module=1510, inline_record=6168; 不是独立调用边。
// [PDB 内联宿主] itertools::tuple_impl::impl$7::next<core::iter::adapters::map::Map<core::slice::iter::Iter<utils::curve::CurveInternalCoord<utils::curve::CurveCoordSemanticThresholded3D> >,system_decorator::cull_oob_decorators::is_wall_decorator_location_valid::closure_env$1>,tuple$<glam::f32::vec3::Vec3,glam::f32::vec3::Vec3> >; module=1552, inline_record=2692; 不是独立调用边。
// [PDB 内联宿主] itertools::tuple_impl::impl$7::next<core::iter::adapters::map::Map<itertools::tuple_impl::TupleWindows<core::iter::adapters::map::Map<core::slice::iter::Iter<utils::curve::CurveInternalCoord<utils::curve::CurveCoordSemanticThresholded3D> >,system_decorator::cull_oob_decorators::is_wall_decorator_location_valid::closure_env$1>,tuple$<glam::f32::vec3::Vec3,glam::f32::vec3::Vec3> >,system_decorator::cull_oob_decorators::is_wall_decorator_location_valid::closure_env$2>,tuple$<glam::f32::vec2::Vec2,glam::f32::vec2::Vec2> >; module=1552, inline_record=6080; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::cull_oob_decorators::is_wall_decorator_location_valid::closure$2
// 信息 ID: 71860952125638575d39517d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "system_decorator::cull_oob_decorators::is_wall_decorator_location_valid", "captures": "unknown"}
// [闭包环境类型引用] system_decorator::cull_oob_decorators::is_wall_decorator_location_valid::closure_env$2 <- itertools::tuple_impl::impl$7::next<core::iter::adapters::map::Map<itertools::tuple_impl::TupleWindows<core::iter::adapters::map::Map<core::slice::iter::Iter<utils::curve::CurveInternalCoord<utils::curve::CurveCoordSemanticThresholded3D> >,system_decorator::cull_oob_decorators::is_wall_decorator_location_valid::closure_env$1>,tuple$<glam::f32::vec3::Vec3,glam::f32::vec3::Vec3> >,system_decorator::cull_oob_decorators::is_wall_decorator_location_valid::closure_env$2>,tuple$<glam::f32::vec2::Vec2,glam::f32::vec2::Vec2> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_decorator::cull_oob_decorators::is_wall_decorator_location_valid::closure_env$2 <- itertools::tuple_impl::impl$31::collect_from_iter_no_buf<glam::f32::vec2::Vec2,core::iter::adapters::chain::Chain<core::iter::sources::once::Once<glam::f32::vec2::Vec2>,ref_mut$<core::iter::adapters::map::Map<itertools::tuple_impl::TupleWindows<core::iter::adapters::map::Map<core::slice::iter::Iter<utils::curve::CurveInternalCoord<utils::curve::CurveCoordSemanticThresholded3D> >,system_decorator::cull_oob_decorators::is_wall_decorator_location_valid::closure_env$1>,tuple$<glam::f32::vec3::Vec3,glam::f32::vec3::Vec3> >,system_decorator::cull_oob_decorators::is_wall_decorator_location_valid::closure_env$2> > > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 2 条。
// [PDB 内联宿主] itertools::tuple_impl::impl$7::next<core::iter::adapters::map::Map<itertools::tuple_impl::TupleWindows<core::iter::adapters::map::Map<core::slice::iter::Iter<utils::curve::CurveInternalCoord<utils::curve::CurveCoordSemanticThresholded3D> >,system_decorator::cull_oob_decorators::is_wall_decorator_location_valid::closure_env$1>,tuple$<glam::f32::vec3::Vec3,glam::f32::vec3::Vec3> >,system_decorator::cull_oob_decorators::is_wall_decorator_location_valid::closure_env$2>,tuple$<glam::f32::vec2::Vec2,glam::f32::vec2::Vec2> >; module=1552, inline_record=7052; 不是独立调用边。
// [PDB 内联宿主] itertools::tuple_impl::impl$31::collect_from_iter_no_buf<glam::f32::vec2::Vec2,core::iter::adapters::chain::Chain<core::iter::sources::once::Once<glam::f32::vec2::Vec2>,ref_mut$<core::iter::adapters::map::Map<itertools::tuple_impl::TupleWindows<core::iter::adapters::map::Map<core::slice::iter::Iter<utils::curve::CurveInternalCoord<utils::curve::CurveCoordSemanticThresholded3D> >,system_decorator::cull_oob_decorators::is_wall_decorator_location_valid::closure_env$1>,tuple$<glam::f32::vec3::Vec3,glam::f32::vec3::Vec3> >,system_decorator::cull_oob_decorators::is_wall_decorator_location_valid::closure_env$2> > > >; module=1562, inline_record=4732; 不是独立调用边。
// [PDB 内联宿主] itertools::tuple_impl::impl$31::collect_from_iter_no_buf<glam::f32::vec2::Vec2,core::iter::adapters::chain::Chain<core::iter::sources::once::Once<glam::f32::vec2::Vec2>,ref_mut$<core::iter::adapters::map::Map<itertools::tuple_impl::TupleWindows<core::iter::adapters::map::Map<core::slice::iter::Iter<utils::curve::CurveInternalCoord<utils::curve::CurveCoordSemanticThresholded3D> >,system_decorator::cull_oob_decorators::is_wall_decorator_location_valid::closure_env$1>,tuple$<glam::f32::vec3::Vec3,glam::f32::vec3::Vec3> >,system_decorator::cull_oob_decorators::is_wall_decorator_location_valid::closure_env$2> > > >; module=1562, inline_record=5392; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::cull_oob_decorators::is_wall_decorator_location_valid::closure$3
// 信息 ID: 7dc186a606c11e97a72f34bb
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 3, "lexical_owner": "system_decorator::cull_oob_decorators::is_wall_decorator_location_valid", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::cull_oob_decorators::is_decorator_location_valid; module=453, inline_record=36480; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::cull_oob_decorators::should_snap_to_corner
// 信息 ID: 7572e312f62c058499eed8e2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_interaction_intent @ 0x127e5bc -> 0x811900 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] system_decorator::cull_oob_decorators::is_decorator_location_valid; module=453, inline_record=36052; 不是独立调用边。
// [PDB 内联宿主] system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_interaction_intent; module=1522, inline_record=3152; 不是独立调用边。
// [PDB 内联宿主] system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_interaction_intent; module=1522, inline_record=3288; 不是独立调用边。
// [PDB 内联宿主] system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_interaction_intent; module=1522, inline_record=3424; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
