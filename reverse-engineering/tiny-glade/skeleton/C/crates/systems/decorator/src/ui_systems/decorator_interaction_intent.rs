// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/systems/decorator/src/ui_systems/decorator_interaction_intent.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: 1cdfe72ba7eb2b535e22eb8c995b3b4d5fd4538369cce62dc9e7d2baa23ee8dc
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: decorator_interaction_intent.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 1579, 1631, 3184, 3185, 3206
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use bevy_ecs; // namespace candidate
//   package identities: [{"name":"bevy_ecs","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.3"}]
// use bevy_ptr; // namespace candidate
//   package identities: [{"name":"bevy_ptr","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.3"}]
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use fixedbitset; // namespace candidate
//   package identities: [{"name":"fixedbitset","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.5.7"}]
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.30.10"}]
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.3"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.5"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.15.2"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.1"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.1"}]
// use itertools; // namespace candidate
//   package identities: [{"name":"itertools","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.0"}]
// use nonmax; // namespace candidate
//   package identities: [{"name":"nonmax","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.5.5"}]
// use std; // namespace candidate
//   package identities: []
// use system_decorator; // namespace candidate
//   package identities: [{"name":"system-decorator","source":"local_or_path","version":"0.1.0"}]
// use utils; // namespace candidate
//   package identities: [{"name":"utils","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_decorator::ui_systems::decorator_interaction_intent::snap_balcony_door
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1579,"record_offset":780,"rva":21848768,"va":5390557888}
// 编译实例 RVAs: 0x14d62c0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140df0580
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decorator_interaction_intent.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7235258","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390557888,"raw_hex":"e8469991ff","size":5,"soft_fail":false,"target_va":5383325056,"va":5390560309,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1413fd3a0
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: decorator_interaction_intent.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-888803","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390557888,"raw_hex":"e81d70f2ff","size":5,"soft_fail":false,"target_va":5389669280,"va":5390558078,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1414dff60
//     owner=unresolved_namespace; count=2
//     target_aliases=3 candidates; complete list: decorator_interaction_intent.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t37878","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390557888,"raw_hex":"e8f6930000","size":5,"soft_fail":false,"target_va":5390597984,"va":5390560101,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26060667","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390557888,"raw_hex":"e87ba78d01","size":5,"soft_fail":false,"target_va":5416621600,"va":5390560928,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26060840","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390557888,"raw_hex":"e828a88d01","size":5,"soft_fail":false,"target_va":5416622208,"va":5390561363,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t26060382","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390557888,"raw_hex":"e85ea68d01","size":5,"soft_fail":false,"target_va":5416621762,"va":5390561375,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26063143","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390557888,"raw_hex":"e827b18d01","size":5,"soft_fail":false,"target_va":5416622240,"va":5390559092,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::roofs::roof_shape::Roof::is_flat
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-9362165","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390557888,"raw_hex":"e80b2571ff","size":5,"soft_fail":false,"target_va":5381197456,"va":5390559616,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator_subtype::DecoratorLibKey::new
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-8907186","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390557888,"raw_hex":"e84e1678ff","size":5,"soft_fail":false,"target_va":5381650864,"va":5390558045,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::collision::world::RaycastWorld::raycast_simple
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-11401509","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390557888,"raw_hex":"e8db0652ff","size":5,"soft_fail":false,"target_va":5379158864,"va":5390560368,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::circle::Circle2d::signed_distance
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-7254947","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390557888,"raw_hex":"e85d4c91ff","size":5,"soft_fail":false,"target_va":5383304928,"va":5390559870,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::contains_point_padding
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-7243014","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390557888,"raw_hex":"e8fa7a91ff","size":5,"soft_fail":false,"target_va":5383316784,"va":5390559793,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::query::iter::impl$1::next
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs
//     evidence={"module":1579,"record_offset":5348,"parent_record_offset":780,"ancestor_record_offset":780,"inlinee":10317,"source_line":909,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::query::Query::iter
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":1579,"record_offset":4912,"parent_record_offset":780,"ancestor_record_offset":780,"inlinee":10312,"source_line":673,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1579,"record_offset":7752,"parent_record_offset":780,"ancestor_record_offset":780,"inlinee":5477,"source_line":991,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_direct] core::ops::range::Range::contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":1579,"record_offset":7696,"parent_record_offset":780,"ancestor_record_offset":780,"inlinee":11437,"source_line":123,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary::get_collision_bbx_ls
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":1579,"record_offset":984,"parent_record_offset":780,"ancestor_record_offset":780,"inlinee":26698,"source_line":223,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::impl$10::eq
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":1579,"record_offset":7668,"parent_record_offset":780,"ancestor_record_offset":780,"inlinee":12117,"source_line":82,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1579,"record_offset":4736,"parent_record_offset":780,"ancestor_record_offset":780,"inlinee":8712,"source_line":1309,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1579,"record_offset":4824,"parent_record_offset":780,"ancestor_record_offset":780,"inlinee":8716,"source_line":1394,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::get_coord_at_2d_ws_distance
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1579,"record_offset":7780,"parent_record_offset":780,"ancestor_record_offset":780,"inlinee":26153,"source_line":1161,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::impl$12::sample_at_coord
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1579,"record_offset":8020,"parent_record_offset":780,"ancestor_record_offset":780,"inlinee":8731,"source_line":1392,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle3d::height
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":1579,"record_offset":3464,"parent_record_offset":780,"ancestor_record_offset":780,"inlinee":11330,"source_line":79,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1579,"record_offset":5180,"parent_record_offset":5156,"ancestor_record_offset":780,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1579,"record_offset":5228,"parent_record_offset":5204,"ancestor_record_offset":780,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1579,"record_offset":5204,"parent_record_offset":5180,"ancestor_record_offset":780,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1579,"record_offset":5156,"parent_record_offset":5132,"ancestor_record_offset":780,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1579,"record_offset":5132,"parent_record_offset":5100,"ancestor_record_offset":780,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1579,"record_offset":5652,"parent_record_offset":5628,"ancestor_record_offset":780,"inlinee":5735,"source_line":3021,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1579,"record_offset":5676,"parent_record_offset":5652,"ancestor_record_offset":780,"inlinee":4800,"source_line":2996,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1579,"record_offset":5100,"parent_record_offset":5072,"ancestor_record_offset":780,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::entities
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs
//     evidence={"module":1579,"record_offset":7100,"parent_record_offset":5372,"ancestor_record_offset":780,"inlinee":10334,"source_line":484,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::is_empty
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs
//     evidence={"module":1579,"record_offset":6392,"parent_record_offset":5372,"ancestor_record_offset":780,"inlinee":10331,"source_line":655,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::table_id
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs
//     evidence={"module":1579,"record_offset":6476,"parent_record_offset":5372,"ancestor_record_offset":780,"inlinee":8568,"source_line":478,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::ArchetypeEntity::table_row
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs
//     evidence={"module":1579,"record_offset":7352,"parent_record_offset":5372,"ancestor_record_offset":780,"inlinee":10337,"source_line":346,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetypes::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs
//     evidence={"module":1579,"record_offset":6308,"parent_record_offset":5372,"ancestor_record_offset":780,"inlinee":10330,"source_line":864,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::StorageSwitch::extract
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":1579,"record_offset":7404,"parent_record_offset":7380,"ancestor_record_offset":780,"inlinee":9590,"source_line":2964,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::set_archetype
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":1579,"record_offset":6568,"parent_record_offset":5372,"ancestor_record_offset":780,"inlinee":10333,"source_line":1530,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::set_table
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":1579,"record_offset":5712,"parent_record_offset":5372,"ancestor_record_offset":780,"inlinee":10322,"source_line":1545,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$37::fetch
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":1579,"record_offset":7380,"parent_record_offset":5372,"ancestor_record_offset":780,"inlinee":10341,"source_line":1598,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$37::fetch::closure$0
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":1579,"record_offset":7428,"parent_record_offset":7404,"ancestor_record_offset":780,"inlinee":9575,"source_line":1605,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIter::new
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs
//     evidence={"module":1579,"record_offset":5048,"parent_record_offset":4936,"ancestor_record_offset":780,"inlinee":10316,"source_line":40,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::init
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs
//     evidence={"module":1579,"record_offset":5072,"parent_record_offset":5048,"ancestor_record_offset":780,"inlinee":9124,"source_line":2408,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::next
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs
//     evidence={"module":1579,"record_offset":5372,"parent_record_offset":5348,"ancestor_record_offset":780,"inlinee":10318,"source_line":2511,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_array::BlobArray::get_sub_slice
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_array.rs
//     evidence={"module":1579,"record_offset":6168,"parent_record_offset":6144,"ancestor_record_offset":780,"inlinee":10327,"source_line":143,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseArray::get
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":1579,"record_offset":5812,"parent_record_offset":5788,"ancestor_record_offset":780,"inlinee":8615,"source_line":56,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":1579,"record_offset":5788,"parent_record_offset":5764,"ancestor_record_offset":780,"inlinee":8626,"source_line":405,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$16::get::closure$0
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":1579,"record_offset":5992,"parent_record_offset":5968,"ancestor_record_offset":780,"inlinee":8628,"source_line":406,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_column
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":1579,"record_offset":5764,"parent_record_offset":5736,"ancestor_record_offset":780,"inlinee":8625,"source_line":489,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_data_slice_for
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":1579,"record_offset":5736,"parent_record_offset":5712,"ancestor_record_offset":780,"inlinee":10323,"source_line":377,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::is_empty
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":1579,"record_offset":5628,"parent_record_offset":5372,"ancestor_record_offset":780,"inlinee":10320,"source_line":646,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::TableRow::index
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":1579,"record_offset":7452,"parent_record_offset":7428,"ancestor_record_offset":780,"inlinee":8632,"source_line":114,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":1579,"record_offset":5544,"parent_record_offset":5372,"ancestor_record_offset":780,"inlinee":10319,"source_line":752,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::column::ThinColumn::get_data_slice
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/column.rs
//     evidence={"module":1579,"record_offset":6144,"parent_record_offset":6120,"ancestor_record_offset":780,"inlinee":10326,"source_line":297,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::impl$4::get_data_slice_for::closure$0
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":1579,"record_offset":6120,"parent_record_offset":6096,"ancestor_record_offset":780,"inlinee":10325,"source_line":382,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::as_readonly
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":1579,"record_offset":4936,"parent_record_offset":4912,"ancestor_record_offset":780,"inlinee":11849,"source_line":545,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::copy_unsafe
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":1579,"record_offset":4988,"parent_record_offset":4964,"ancestor_record_offset":780,"inlinee":11851,"source_line":639,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":1579,"record_offset":5012,"parent_record_offset":4988,"ancestor_record_offset":780,"inlinee":11852,"source_line":522,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::reborrow_unsafe
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":1579,"record_offset":4964,"parent_record_offset":4936,"ancestor_record_offset":780,"inlinee":11850,"source_line":621,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::impl$4::into_iter
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":1579,"record_offset":9156,"parent_record_offset":9132,"ancestor_record_offset":780,"inlinee":10314,"source_line":2505,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ptr::ThinSlicePtr::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ptr","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ptr-0.17.3/src/lib.rs
//     evidence={"module":1579,"record_offset":7592,"parent_record_offset":7568,"ancestor_record_offset":780,"inlinee":9579,"source_line":1073,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1579,"record_offset":1892,"parent_record_offset":1864,"ancestor_record_offset":780,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1579,"record_offset":1376,"parent_record_offset":1348,"ancestor_record_offset":780,"inlinee":4309,"source_line":1333,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1579,"record_offset":1472,"parent_record_offset":1444,"ancestor_record_offset":780,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":1579,"record_offset":2128,"parent_record_offset":1200,"ancestor_record_offset":780,"inlinee":5448,"source_line":433,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":1579,"record_offset":1660,"parent_record_offset":1624,"ancestor_record_offset":780,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":1579,"record_offset":1624,"parent_record_offset":1588,"ancestor_record_offset":780,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1579,"record_offset":4852,"parent_record_offset":4824,"ancestor_record_offset":780,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1579,"record_offset":4764,"parent_record_offset":4736,"ancestor_record_offset":780,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1579,"record_offset":8260,"parent_record_offset":8236,"ancestor_record_offset":780,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":1579,"record_offset":5904,"parent_record_offset":5840,"ancestor_record_offset":780,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":1579,"record_offset":7720,"parent_record_offset":7696,"ancestor_record_offset":780,"inlinee":6826,"source_line":868,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1579,"record_offset":5928,"parent_record_offset":5904,"ancestor_record_offset":780,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1579,"record_offset":5968,"parent_record_offset":5788,"ancestor_record_offset":780,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1579,"record_offset":11968,"parent_record_offset":11936,"ancestor_record_offset":780,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1579,"record_offset":3404,"parent_record_offset":3380,"ancestor_record_offset":780,"inlinee":4428,"source_line":1061,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1579,"record_offset":1404,"parent_record_offset":1376,"ancestor_record_offset":780,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1579,"record_offset":5296,"parent_record_offset":5272,"ancestor_record_offset":780,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1579,"record_offset":1796,"parent_record_offset":1768,"ancestor_record_offset":780,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1579,"record_offset":5512,"parent_record_offset":5452,"ancestor_record_offset":780,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1579,"record_offset":5484,"parent_record_offset":5452,"ancestor_record_offset":780,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1579,"record_offset":5568,"parent_record_offset":5544,"ancestor_record_offset":780,"inlinee":4165,"source_line":572,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1579,"record_offset":7296,"parent_record_offset":5372,"ancestor_record_offset":780,"inlinee":4887,"source_line":639,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1579,"record_offset":8064,"parent_record_offset":8020,"ancestor_record_offset":780,"inlinee":5467,"source_line":18,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1579,"record_offset":5592,"parent_record_offset":5568,"ancestor_record_offset":780,"inlinee":4167,"source_line":218,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1579,"record_offset":6048,"parent_record_offset":6016,"ancestor_record_offset":780,"inlinee":4183,"source_line":239,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1579,"record_offset":8096,"parent_record_offset":8064,"ancestor_record_offset":780,"inlinee":5468,"source_line":270,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":1579,"record_offset":5272,"parent_record_offset":5100,"ancestor_record_offset":780,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1579,"record_offset":5452,"parent_record_offset":5372,"ancestor_record_offset":780,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::impl$24::eq
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":1579,"record_offset":1920,"parent_record_offset":1892,"ancestor_record_offset":780,"inlinee":26681,"source_line":94,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::impl$4::get_collision_bbx_ls::closure$0
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":1579,"record_offset":3428,"parent_record_offset":3404,"ancestor_record_offset":780,"inlinee":26707,"source_line":226,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1579,"record_offset":8292,"parent_record_offset":8020,"ancestor_record_offset":780,"inlinee":9798,"source_line":1501,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1579,"record_offset":8348,"parent_record_offset":8020,"ancestor_record_offset":780,"inlinee":9800,"source_line":1589,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1579,"record_offset":8236,"parent_record_offset":8020,"ancestor_record_offset":780,"inlinee":9774,"source_line":1729,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":1579,"record_offset":1544,"parent_record_offset":1508,"ancestor_record_offset":780,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":1579,"record_offset":1588,"parent_record_offset":1544,"ancestor_record_offset":780,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":1579,"record_offset":1508,"parent_record_offset":1200,"ancestor_record_offset":780,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":1579,"record_offset":1348,"parent_record_offset":1200,"ancestor_record_offset":780,"inlinee":4307,"source_line":51,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":1579,"record_offset":1968,"parent_record_offset":1200,"ancestor_record_offset":780,"inlinee":4337,"source_line":94,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":1579,"record_offset":1444,"parent_record_offset":1200,"ancestor_record_offset":780,"inlinee":4311,"source_line":76,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\tag.rs
//     evidence={"module":1579,"record_offset":1292,"parent_record_offset":1200,"ancestor_record_offset":780,"inlinee":4405,"source_line":36,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\lib.rs
//     evidence={"module":1579,"record_offset":1864,"parent_record_offset":1836,"ancestor_record_offset":780,"inlinee":5439,"source_line":173,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":1579,"record_offset":1060,"parent_record_offset":1024,"ancestor_record_offset":780,"inlinee":4299,"source_line":1304,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":1579,"record_offset":1836,"parent_record_offset":1708,"ancestor_record_offset":780,"inlinee":4327,"source_line":224,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":1579,"record_offset":1100,"parent_record_offset":1060,"ancestor_record_offset":780,"inlinee":4403,"source_line":253,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1579,"record_offset":1768,"parent_record_offset":1740,"ancestor_record_offset":780,"inlinee":4324,"source_line":312,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1579,"record_offset":2088,"parent_record_offset":1200,"ancestor_record_offset":780,"inlinee":4339,"source_line":83,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1579,"record_offset":1740,"parent_record_offset":1708,"ancestor_record_offset":780,"inlinee":4322,"source_line":785,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1579,"record_offset":1164,"parent_record_offset":1128,"ancestor_record_offset":780,"inlinee":4302,"source_line":1226,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1579,"record_offset":1128,"parent_record_offset":1060,"ancestor_record_offset":780,"inlinee":4301,"source_line":1249,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1579,"record_offset":1200,"parent_record_offset":1164,"ancestor_record_offset":780,"inlinee":4304,"source_line":2049,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1579,"record_offset":1320,"parent_record_offset":1200,"ancestor_record_offset":780,"inlinee":4305,"source_line":2488,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1579,"record_offset":1708,"parent_record_offset":1200,"ancestor_record_offset":780,"inlinee":4321,"source_line":1236,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxU32::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"nonmax","version":"0.5.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":1579,"record_offset":7476,"parent_record_offset":7452,"ancestor_record_offset":780,"inlinee":8634,"source_line":153,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"nonmax","version":"0.5.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":1579,"record_offset":6016,"parent_record_offset":5992,"ancestor_record_offset":780,"inlinee":4193,"source_line":153,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":1579,"record_offset":1024,"parent_record_offset":984,"ancestor_record_offset":780,"inlinee":7239,"source_line":997,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i1
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1579,"record_offset":8136,"parent_record_offset":8020,"ancestor_record_offset":780,"inlinee":8706,"source_line":1337,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::CurveSlice::end
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":1579,"record_offset":7960,"parent_record_offset":7780,"ancestor_record_offset":780,"inlinee":15960,"source_line":338,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::CurveSlice::extend_right
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":1579,"record_offset":7932,"parent_record_offset":7780,"ancestor_record_offset":780,"inlinee":17828,"source_line":236,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::CurveSlice::from_curve_range
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":1579,"record_offset":7816,"parent_record_offset":7780,"ancestor_record_offset":780,"inlinee":15962,"source_line":89,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::CurveSlice::from_point_range
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":1579,"record_offset":7844,"parent_record_offset":7816,"ancestor_record_offset":780,"inlinee":15948,"source_line":110,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::CurveSlice::remap
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":1579,"record_offset":7900,"parent_record_offset":7876,"ancestor_record_offset":780,"inlinee":15961,"source_line":196,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::CurveSlice::remap_2d
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":1579,"record_offset":7876,"parent_record_offset":7780,"ancestor_record_offset":780,"inlinee":26154,"source_line":220,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_interaction_intent
// 记录形式: compiled_procedure; instances=1
// 源码行: 96
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1579,"record_offset":12012,"rva":21852304,"va":5390561424}
// 编译实例 RVAs: 0x14d7090
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=5
//     evidence={"assembly":"\tcallq\t-14883743","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390561424,"raw_hex":"e861e41cff","size":5,"soft_fail":false,"target_va":5375678704,"va":5390562442,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=1
//     target_aliases=1699 candidates; complete list: decorator_interaction_intent.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-21839884","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390561424,"raw_hex":"e8f4bfb2fe","size":5,"soft_fail":false,"target_va":5368722192,"va":5390562071,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140104100
//     owner=unresolved_namespace; count=3
//     target_aliases=583 candidates; complete list: decorator_interaction_intent.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-20787752","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390561424,"raw_hex":"e8d8cdc2fe","size":5,"soft_fail":false,"target_va":5369774336,"va":5390562083,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408f9b30
//     owner=unresolved_namespace; count=3
//     target_aliases=3 candidates; complete list: decorator_interaction_intent.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12446304","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390561424,"raw_hex":"e8a01542ff","size":5,"soft_fail":false,"target_va":5378120496,"va":5390566795,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408fb460
//     owner=unresolved_namespace; count=2
//     target_aliases=20 candidates; complete list: decorator_interaction_intent.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12434969","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390561424,"raw_hex":"e8e74142ff","size":5,"soft_fail":false,"target_va":5378126944,"va":5390561908,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408fb490
//     owner=unresolved_namespace; count=2
//     target_aliases=3 candidates; complete list: decorator_interaction_intent.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12435712","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390561424,"raw_hex":"e8003f42ff","size":5,"soft_fail":false,"target_va":5378126992,"va":5390562699,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a47e70
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: decorator_interaction_intent.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11072794","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390561424,"raw_hex":"e8e60a57ff","size":5,"soft_fail":false,"target_va":5379489392,"va":5390562181,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140de61f0
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: decorator_interaction_intent.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7283542","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390561424,"raw_hex":"e8aadc90ff","size":5,"soft_fail":false,"target_va":5383283184,"va":5390566721,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140de62e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decorator_interaction_intent.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7285556","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390561424,"raw_hex":"e8ccd490ff","size":5,"soft_fail":false,"target_va":5383283424,"va":5390568975,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140de63a0
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: decorator_interaction_intent.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7284108","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390561424,"raw_hex":"e874da90ff","size":5,"soft_fail":false,"target_va":5383283616,"va":5390567719,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140df0580
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decorator_interaction_intent.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7243528","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390561424,"raw_hex":"e8f87891ff","size":5,"soft_fail":false,"target_va":5383325056,"va":5390568579,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1414991f0
//     owner=unresolved_namespace; count=8
//     target_aliases=5 candidates; complete list: decorator_interaction_intent.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-255497","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390561424,"raw_hex":"e8f719fcff","size":5,"soft_fail":false,"target_va":5390307824,"va":5390563316,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141499930
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: decorator_interaction_intent.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-252358","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390561424,"raw_hex":"e83a26fcff","size":5,"soft_fail":false,"target_va":5390309680,"va":5390562033,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1414a11e0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: decorator_interaction_intent.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-221387","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390561424,"raw_hex":"e8359ffcff","size":5,"soft_fail":false,"target_va":5390340576,"va":5390561958,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1424f13c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decorator_interaction_intent.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t16878542","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390561424,"raw_hex":"e8ce8b0101","size":5,"soft_fail":false,"target_va":5407445952,"va":5390567405,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1424f1e00
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decorator_interaction_intent.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t16886299","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390561424,"raw_hex":"e81baa0101","size":5,"soft_fail":false,"target_va":5407448576,"va":5390562272,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-12234151","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390561424,"raw_hex":"e8595245ff","size":5,"soft_fail":false,"target_va":5378335216,"va":5390569362,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::Option::and_then<ref$<country_core::resources::walls::decorator_storage::Decorators>,system_decorator::merge_proposal_roof_decorators::RoofDecoratorMergeProposal,system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_interaction_intent::closure_env$2>
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t3188","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390561424,"raw_hex":"e8740c0000","size":5,"soft_fail":false,"target_va":5390570704,"va":5390567511,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t26052327","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390561424,"raw_hex":"e8e7868d01","size":5,"soft_fail":false,"target_va":5416621600,"va":5390569268,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t26052492","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390561424,"raw_hex":"e88c878d01","size":5,"soft_fail":false,"target_va":5416622208,"va":5390569711,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=12
//     evidence={"assembly":"\tcallq\t26051932","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390561424,"raw_hex":"e85c858d01","size":5,"soft_fail":false,"target_va":5416621762,"va":5390569825,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t26052913","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390561424,"raw_hex":"e831898d01","size":5,"soft_fail":false,"target_va":5416622240,"va":5390569322,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26051682","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390561424,"raw_hex":"e862848d01","size":5,"soft_fail":false,"target_va":5416621632,"va":5390569945,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::roofs::roof_shape::Roof::is_flat
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-9368110","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390561424,"raw_hex":"e8d20d71ff","size":5,"soft_fail":false,"target_va":5381197456,"va":5390565561,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::roofs::roof_shape::Roof::rotation_yaw_radians
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-9370441","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390561424,"raw_hex":"e8b70471ff","size":5,"soft_fail":false,"target_va":5381195200,"va":5390565636,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::WallId::is_valid
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-12341963","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390561424,"raw_hex":"e835ad43ff","size":5,"soft_fail":false,"target_va":5378226912,"va":5390568870,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator::DecoratorInfo::try_dst_as_wall
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-12441927","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390561424,"raw_hex":"e8b92642ff","size":5,"soft_fail":false,"target_va":5378126928,"va":5390568850,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator::DecoratorRotationXZ::from_quat
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-12434752","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390561424,"raw_hex":"e8c04242ff","size":5,"soft_fail":false,"target_va":5378127840,"va":5390562587,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator::DecoratorRotationXZ::from_xz_forward_vector
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-12438755","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390561424,"raw_hex":"e81d3342ff","size":5,"soft_fail":false,"target_va":5378127680,"va":5390566430,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator::DecoratorRotationXZ::offset
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-12438742","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390561424,"raw_hex":"e82a3342ff","size":5,"soft_fail":false,"target_va":5378127712,"va":5390566449,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator::DecoratorType::can_place_on_terrain
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-12435568","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390561424,"raw_hex":"e8903f42ff","size":5,"soft_fail":false,"target_va":5378127008,"va":5390562571,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator::DecoratorType::max_rank
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-12440026","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390561424,"raw_hex":"e8262e42ff","size":5,"soft_fail":false,"target_va":5378127056,"va":5390567077,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator::DecoratorType::should_offer_snap_to_corner
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-12439716","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390561424,"raw_hex":"e85c2f42ff","size":5,"soft_fail":false,"target_va":5378127040,"va":5390566751,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator_subtype::DecoratorLibKey::new
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-8911065","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390561424,"raw_hex":"e8270778ff","size":5,"soft_fail":false,"target_va":5381650864,"va":5390561924,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_wall_state::PublicWallState::wall_space
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-9330074","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390561424,"raw_hex":"e866a271ff","size":5,"soft_fail":false,"target_va":5381231744,"va":5390561813,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_walls::PublicWalls::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-9658262","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390561424,"raw_hex":"e86aa06cff","size":5,"soft_fail":false,"target_va":5380903536,"va":5390561793,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_walls::PublicWalls::roof_entity
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-9662919","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390561424,"raw_hex":"e8398e6cff","size":5,"soft_fail":false,"target_va":5380902448,"va":5390565362,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::wall_attachment::WallCornerAttachment::from_rectangle_corner_id
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-10352102","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390561424,"raw_hex":"e81a0a62ff","size":5,"soft_fail":false,"target_va":5380216288,"va":5390568385,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::wall::shapes::shape_placement::snap_to_roof_ridge
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-9929375","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390561424,"raw_hex":"e8617d68ff","size":5,"soft_fail":false,"target_va":5380636384,"va":5390565754,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::utils::debug_viz::DebugLine::dashed
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-10890343","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390561424,"raw_hex":"e899d359ff","size":5,"soft_fail":false,"target_va":5379679152,"va":5390569490,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::utils::debug_viz::DebugLine::line_width
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-10890508","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390561424,"raw_hex":"e8f4d259ff","size":5,"soft_fail":false,"target_va":5379678976,"va":5390569479,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::utils::debug_viz::DebugLine::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-10890434","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390561424,"raw_hex":"e83ed359ff","size":5,"soft_fail":false,"target_va":5379679024,"va":5390569453,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::utils::debug_viz::DebugLine::show
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-10890422","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390561424,"raw_hex":"e84ad359ff","size":5,"soft_fail":false,"target_va":5379679088,"va":5390569505,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::utils::debug_viz::DebugSphere::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-10890652","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390561424,"raw_hex":"e864d259ff","size":5,"soft_fail":false,"target_va":5379675184,"va":5390565831,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::utils::debug_viz::DebugSphere::radius
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-10890550","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390561424,"raw_hex":"e8cad259ff","size":5,"soft_fail":false,"target_va":5379675312,"va":5390565857,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::utils::debug_viz::DebugSphere::show
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-10890656","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390561424,"raw_hex":"e860d259ff","size":5,"soft_fail":false,"target_va":5379675232,"va":5390565883,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::utils::debug_viz::DebugSphere::solid
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-10890593","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390561424,"raw_hex":"e89fd259ff","size":5,"soft_fail":false,"target_va":5379675280,"va":5390565868,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_decorator::cull_oob_decorators::should_snap_to_corner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-12600518","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390561424,"raw_hex":"e83abb3fff","size":5,"soft_fail":false,"target_va":5377967856,"va":5390568369,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_decorator::derive_decorator_subtype::derive_decorator_subtype
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-12576186","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390561424,"raw_hex":"e8461a40ff","size":5,"soft_fail":false,"target_va":5377985712,"va":5390561893,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_decorator::merge_proposal_wall_decorators::find_best_wall_decorator_merge_position
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t16987147","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390561424,"raw_hex":"e80b340301","size":5,"soft_fail":false,"target_va":5407549552,"va":5390562400,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_interaction_intent::closure$4
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t26178865","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390561424,"raw_hex":"e831758f01","size":5,"soft_fail":false,"target_va":5416748752,"va":5390569882,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_decorator::ui_systems::decorator_interaction_intent::snap_balcony_door
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-10929","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390561424,"raw_hex":"e84fd5ffff","size":5,"soft_fail":false,"target_va":5390557888,"va":5390568812,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::get_point_u
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-7257566","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390561424,"raw_hex":"e8224291ff","size":5,"soft_fail":false,"target_va":5383309120,"va":5390566681,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::boxed::box_assume_init_into_vec_unsafe
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":1579,"record_offset":25524,"parent_record_offset":12012,"ancestor_record_offset":12012,"inlinee":4459,"source_line":262,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_direct] alloc::boxed::impl$0::new_uninit
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":1579,"record_offset":25384,"parent_record_offset":12012,"ancestor_record_offset":12012,"inlinee":4443,"source_line":311,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1579,"record_offset":24964,"parent_record_offset":12012,"ancestor_record_offset":12012,"inlinee":6153,"source_line":463,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::query::Query::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":1579,"record_offset":21980,"parent_record_offset":12012,"ancestor_record_offset":12012,"inlinee":11704,"source_line":1388,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1579,"record_offset":23376,"parent_record_offset":12012,"ancestor_record_offset":12012,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1579,"record_offset":21916,"parent_record_offset":12012,"ancestor_record_offset":12012,"inlinee":4428,"source_line":1061,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1579,"record_offset":25092,"parent_record_offset":12012,"ancestor_record_offset":12012,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1579,"record_offset":25636,"parent_record_offset":12012,"ancestor_record_offset":12012,"inlinee":4983,"source_line":1227,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$171::find
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1579,"record_offset":15888,"parent_record_offset":12012,"ancestor_record_offset":12012,"inlinee":6791,"source_line":343,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator::impl$65::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":1579,"record_offset":24992,"parent_record_offset":12012,"ancestor_record_offset":12012,"inlinee":26911,"source_line":277,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo::dims_wall_space
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":1579,"record_offset":24872,"parent_record_offset":12012,"ancestor_record_offset":12012,"inlinee":27083,"source_line":285,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::quat::Quat::is_finite
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/quat.rs
//     evidence={"module":1579,"record_offset":12340,"parent_record_offset":12012,"ancestor_record_offset":12012,"inlinee":27906,"source_line":596,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1579,"record_offset":24844,"parent_record_offset":12012,"ancestor_record_offset":12012,"inlinee":9814,"source_line":185,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::normalize_or_zero
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1579,"record_offset":24164,"parent_record_offset":12012,"ancestor_record_offset":12012,"inlinee":9810,"source_line":581,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::perp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1579,"record_offset":24388,"parent_record_offset":12012,"ancestor_record_offset":12012,"inlinee":9809,"source_line":994,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1579,"record_offset":24756,"parent_record_offset":12012,"ancestor_record_offset":12012,"inlinee":8708,"source_line":1529,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::is_finite
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1579,"record_offset":12280,"parent_record_offset":12012,"ancestor_record_offset":12012,"inlinee":27905,"source_line":519,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1579,"record_offset":25412,"parent_record_offset":12012,"ancestor_record_offset":12012,"inlinee":9800,"source_line":1589,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_wall_interaction_intent
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/ui_systems/decorator_interaction_intent.rs
//     evidence={"module":1579,"record_offset":12536,"parent_record_offset":12012,"ancestor_record_offset":12012,"inlinee":80546,"source_line":477,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::ui_systems::decorator_interaction_intent::find_closest_norm_curve_t
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/ui_systems/decorator_interaction_intent.rs
//     evidence={"module":1579,"record_offset":16060,"parent_record_offset":12012,"ancestor_record_offset":12012,"inlinee":80553,"source_line":408,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::get_pos_at_coord
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1579,"record_offset":23412,"parent_record_offset":12012,"ancestor_record_offset":12012,"inlinee":14727,"source_line":799,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::impl$12::sample_at_coord
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1579,"record_offset":23664,"parent_record_offset":12012,"ancestor_record_offset":12012,"inlinee":8731,"source_line":1392,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1579,"record_offset":13428,"parent_record_offset":13404,"ancestor_record_offset":12012,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1579,"record_offset":13452,"parent_record_offset":13428,"ancestor_record_offset":12012,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1579,"record_offset":13476,"parent_record_offset":13452,"ancestor_record_offset":12012,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1579,"record_offset":13404,"parent_record_offset":13312,"ancestor_record_offset":12012,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1579,"record_offset":22692,"parent_record_offset":22668,"ancestor_record_offset":12012,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1579,"record_offset":13340,"parent_record_offset":13312,"ancestor_record_offset":12012,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1579,"record_offset":13312,"parent_record_offset":13288,"ancestor_record_offset":12012,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1579,"record_offset":22740,"parent_record_offset":22716,"ancestor_record_offset":12012,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1579,"record_offset":22716,"parent_record_offset":22692,"ancestor_record_offset":12012,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1579,"record_offset":13288,"parent_record_offset":13264,"ancestor_record_offset":12012,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1579,"record_offset":25548,"parent_record_offset":25524,"ancestor_record_offset":12012,"inlinee":4460,"source_line":480,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1579,"record_offset":22668,"parent_record_offset":22644,"ancestor_record_offset":12012,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1579,"record_offset":22644,"parent_record_offset":22612,"ancestor_record_offset":12012,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::from_raw_parts_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1579,"record_offset":25572,"parent_record_offset":25548,"ancestor_record_offset":12012,"inlinee":4461,"source_line":1181,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1579,"record_offset":13148,"parent_record_offset":12536,"ancestor_record_offset":12012,"inlinee":6153,"source_line":463,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$14::index
//     owner=rust_standard_library; count=16
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1579,"record_offset":14440,"parent_record_offset":14388,"ancestor_record_offset":12012,"inlinee":5465,"source_line":3803,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1579,"record_offset":22612,"parent_record_offset":22588,"ancestor_record_offset":12012,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::ArchetypeId::index
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs
//     evidence={"module":1579,"record_offset":22400,"parent_record_offset":22116,"ancestor_record_offset":12012,"inlinee":8565,"source_line":122,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::Entities::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/entity/mod.rs
//     evidence={"module":1579,"record_offset":22164,"parent_record_offset":22116,"ancestor_record_offset":12012,"inlinee":8602,"source_line":948,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::Entities::get_id_location
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/entity/mod.rs
//     evidence={"module":1579,"record_offset":22188,"parent_record_offset":22164,"ancestor_record_offset":12012,"inlinee":8603,"source_line":955,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::impl$18::get_id_location::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/entity/mod.rs
//     evidence={"module":1579,"record_offset":22304,"parent_record_offset":22272,"ancestor_record_offset":12012,"inlinee":8620,"source_line":958,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::impl$37::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/entity/mod.rs
//     evidence={"module":1579,"record_offset":22328,"parent_record_offset":22304,"ancestor_record_offset":12012,"inlinee":8622,"source_line":214,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::StorageSwitch::extract
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":1579,"record_offset":23216,"parent_record_offset":23192,"ancestor_record_offset":12012,"inlinee":9590,"source_line":2964,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::set_archetype
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":1579,"record_offset":22820,"parent_record_offset":22116,"ancestor_record_offset":12012,"inlinee":10333,"source_line":1530,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::set_table
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":1579,"record_offset":22844,"parent_record_offset":22820,"ancestor_record_offset":12012,"inlinee":10322,"source_line":1545,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$37::fetch
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":1579,"record_offset":23192,"parent_record_offset":22116,"ancestor_record_offset":12012,"inlinee":10341,"source_line":1598,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$37::fetch::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":1579,"record_offset":23240,"parent_record_offset":23216,"ancestor_record_offset":12012,"inlinee":9575,"source_line":1605,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseArray::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":1579,"record_offset":22940,"parent_record_offset":22916,"ancestor_record_offset":12012,"inlinee":8615,"source_line":56,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":1579,"record_offset":22916,"parent_record_offset":22892,"ancestor_record_offset":12012,"inlinee":8626,"source_line":405,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$16::get::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":1579,"record_offset":23076,"parent_record_offset":23052,"ancestor_record_offset":12012,"inlinee":8628,"source_line":406,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_column
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":1579,"record_offset":22892,"parent_record_offset":22868,"ancestor_record_offset":12012,"inlinee":8625,"source_line":489,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_data_slice_for
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":1579,"record_offset":22868,"parent_record_offset":22844,"ancestor_record_offset":12012,"inlinee":10323,"source_line":377,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::TableRow::index
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":1579,"record_offset":23264,"parent_record_offset":23240,"ancestor_record_offset":12012,"inlinee":8632,"source_line":114,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":1579,"record_offset":22588,"parent_record_offset":22116,"ancestor_record_offset":12012,"inlinee":10319,"source_line":752,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::as_readonly
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":1579,"record_offset":22004,"parent_record_offset":21980,"ancestor_record_offset":12012,"inlinee":11849,"source_line":545,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::copy_unsafe
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":1579,"record_offset":22052,"parent_record_offset":22028,"ancestor_record_offset":12012,"inlinee":11851,"source_line":639,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::get_inner
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":1579,"record_offset":22116,"parent_record_offset":21980,"ancestor_record_offset":12012,"inlinee":11705,"source_line":1539,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":1579,"record_offset":22076,"parent_record_offset":22052,"ancestor_record_offset":12012,"inlinee":11852,"source_line":522,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::reborrow_unsafe
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":1579,"record_offset":22028,"parent_record_offset":22004,"ancestor_record_offset":12012,"inlinee":11850,"source_line":621,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] core::bool::impl$0::then
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\bool.rs
//     evidence={"module":1579,"record_offset":22436,"parent_record_offset":22400,"ancestor_record_offset":12012,"inlinee":4276,"source_line":65,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::Ord::min
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1579,"record_offset":21652,"parent_record_offset":16060,"ancestor_record_offset":12012,"inlinee":6426,"source_line":1072,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1579,"record_offset":18488,"parent_record_offset":18464,"ancestor_record_offset":12012,"inlinee":6833,"source_line":2145,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::le
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1579,"record_offset":18512,"parent_record_offset":18488,"ancestor_record_offset":12012,"inlinee":8914,"source_line":1918,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::partial_cmp
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1579,"record_offset":21516,"parent_record_offset":21368,"ancestor_record_offset":12012,"inlinee":11574,"source_line":1957,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$76::lt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1579,"record_offset":18056,"parent_record_offset":18032,"ancestor_record_offset":12012,"inlinee":7964,"source_line":1916,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::min_by
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1579,"record_offset":21164,"parent_record_offset":21136,"ancestor_record_offset":12012,"inlinee":5404,"source_line":1605,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1579,"record_offset":15292,"parent_record_offset":14872,"ancestor_record_offset":12012,"inlinee":7191,"source_line":35,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ss
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1579,"record_offset":15200,"parent_record_offset":15144,"ancestor_record_offset":12012,"inlinee":11729,"source_line":22,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_and_ps
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1579,"record_offset":12436,"parent_record_offset":12412,"ancestor_record_offset":12012,"inlinee":27910,"source_line":253,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_cmplt_ps
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1579,"record_offset":12468,"parent_record_offset":12388,"ancestor_record_offset":12012,"inlinee":27911,"source_line":516,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1579,"record_offset":14968,"parent_record_offset":14944,"ancestor_record_offset":12012,"inlinee":7190,"source_line":87,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1579,"record_offset":15000,"parent_record_offset":14916,"ancestor_record_offset":12012,"inlinee":11716,"source_line":1045,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_sub_ps
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1579,"record_offset":15064,"parent_record_offset":14872,"ancestor_record_offset":12012,"inlinee":11730,"source_line":61,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::abs
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1579,"record_offset":13980,"parent_record_offset":13908,"ancestor_record_offset":12012,"inlinee":11731,"source_line":1569,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1579,"record_offset":13080,"parent_record_offset":13056,"ancestor_record_offset":12012,"inlinee":9255,"source_line":1503,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::is_finite
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1579,"record_offset":12308,"parent_record_offset":12280,"ancestor_record_offset":12012,"inlinee":11774,"source_line":651,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1579,"record_offset":21772,"parent_record_offset":16060,"ancestor_record_offset":12012,"inlinee":5477,"source_line":991,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::min
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1579,"record_offset":21744,"parent_record_offset":16060,"ancestor_record_offset":12012,"inlinee":5478,"source_line":1018,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1579,"record_offset":24336,"parent_record_offset":24312,"ancestor_record_offset":12012,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::copied::copy_fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\copied.rs
//     evidence={"module":1579,"record_offset":21076,"parent_record_offset":21044,"ancestor_record_offset":12012,"inlinee":10907,"source_line":36,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::copied::impl$1::fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\copied.rs
//     evidence={"module":1579,"record_offset":21012,"parent_record_offset":20980,"ancestor_record_offset":12012,"inlinee":10905,"source_line":77,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":1579,"record_offset":20980,"parent_record_offset":20948,"ancestor_record_offset":12012,"inlinee":10794,"source_line":129,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::fold::enumerate::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":1579,"record_offset":21104,"parent_record_offset":21076,"ancestor_record_offset":12012,"inlinee":5430,"source_line":139,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":1579,"record_offset":18032,"parent_record_offset":18008,"ancestor_record_offset":12012,"inlinee":5173,"source_line":771,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":1579,"record_offset":18008,"parent_record_offset":16060,"ancestor_record_offset":12012,"inlinee":5171,"source_line":856,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::min_by
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1579,"record_offset":20916,"parent_record_offset":16060,"ancestor_record_offset":12012,"inlinee":9019,"source_line":3384,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::min_by::fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1579,"record_offset":21136,"parent_record_offset":21104,"ancestor_record_offset":12012,"inlinee":5402,"source_line":3391,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::reduce
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1579,"record_offset":20948,"parent_record_offset":20916,"ancestor_record_offset":12012,"inlinee":9020,"source_line":2678,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1579,"record_offset":21624,"parent_record_offset":16060,"ancestor_record_offset":12012,"inlinee":9345,"source_line":2344,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1579,"record_offset":13372,"parent_record_offset":13340,"ancestor_record_offset":12012,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1579,"record_offset":14800,"parent_record_offset":14776,"ancestor_record_offset":12012,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1579,"record_offset":14716,"parent_record_offset":14692,"ancestor_record_offset":12012,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$385::sub_assign
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1579,"record_offset":17052,"parent_record_offset":17028,"ancestor_record_offset":12012,"inlinee":11677,"source_line":862,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1579,"record_offset":16356,"parent_record_offset":16332,"ancestor_record_offset":12012,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":1579,"record_offset":22988,"parent_record_offset":22964,"ancestor_record_offset":12012,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":1579,"record_offset":21280,"parent_record_offset":21164,"ancestor_record_offset":12012,"inlinee":5124,"source_line":309,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":1579,"record_offset":18464,"parent_record_offset":18440,"ancestor_record_offset":12012,"inlinee":6826,"source_line":868,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":1579,"record_offset":18440,"parent_record_offset":18416,"ancestor_record_offset":12012,"inlinee":9795,"source_line":522,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1579,"record_offset":13176,"parent_record_offset":12536,"ancestor_record_offset":12012,"inlinee":5449,"source_line":1541,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1579,"record_offset":23012,"parent_record_offset":22988,"ancestor_record_offset":12012,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::filter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1579,"record_offset":22272,"parent_record_offset":22188,"ancestor_record_offset":12012,"inlinee":8618,"source_line":1578,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_none
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1579,"record_offset":13532,"parent_record_offset":12536,"ancestor_record_offset":12012,"inlinee":4877,"source_line":684,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1579,"record_offset":13556,"parent_record_offset":13532,"ancestor_record_offset":12012,"inlinee":4878,"source_line":635,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some_and
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1579,"record_offset":15504,"parent_record_offset":12536,"ancestor_record_offset":12012,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1579,"record_offset":23052,"parent_record_offset":22916,"ancestor_record_offset":12012,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::ok_or
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1579,"record_offset":22372,"parent_record_offset":22116,"ancestor_record_offset":12012,"inlinee":5838,"source_line":1337,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1579,"record_offset":12812,"parent_record_offset":12536,"ancestor_record_offset":12012,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1579,"record_offset":12904,"parent_record_offset":12876,"ancestor_record_offset":12012,"inlinee":4428,"source_line":1061,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1579,"record_offset":13240,"parent_record_offset":12536,"ancestor_record_offset":12012,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1579,"record_offset":22540,"parent_record_offset":22508,"ancestor_record_offset":12012,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1579,"record_offset":15940,"parent_record_offset":15916,"ancestor_record_offset":12012,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1579,"record_offset":22216,"parent_record_offset":22188,"ancestor_record_offset":12012,"inlinee":4165,"source_line":572,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=24
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1579,"record_offset":14464,"parent_record_offset":14440,"ancestor_record_offset":12012,"inlinee":5467,"source_line":18,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1579,"record_offset":22240,"parent_record_offset":22216,"ancestor_record_offset":12012,"inlinee":4167,"source_line":218,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1579,"record_offset":23132,"parent_record_offset":23100,"ancestor_record_offset":12012,"inlinee":4183,"source_line":239,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=24
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1579,"record_offset":14488,"parent_record_offset":14464,"ancestor_record_offset":12012,"inlinee":5468,"source_line":270,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1579,"record_offset":21044,"parent_record_offset":21012,"ancestor_record_offset":12012,"inlinee":6636,"source_line":259,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1579,"record_offset":15916,"parent_record_offset":15888,"ancestor_record_offset":12012,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::stairs::stairs_state::impl$17::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/stairs/stairs_state.rs
//     evidence={"module":1579,"record_offset":15996,"parent_record_offset":15972,"ancestor_record_offset":12012,"inlinee":8701,"source_line":38,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator::impl$120::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":1579,"record_offset":25060,"parent_record_offset":24992,"ancestor_record_offset":12012,"inlinee":26915,"source_line":403,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator::impl$130::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":1579,"record_offset":25032,"parent_record_offset":24992,"ancestor_record_offset":12012,"inlinee":26913,"source_line":570,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator::impl$33::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":1579,"record_offset":16020,"parent_record_offset":15996,"ancestor_record_offset":12012,"inlinee":7427,"source_line":155,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::DecoratorCollision::dims_wall_space
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":1579,"record_offset":12932,"parent_record_offset":12876,"ancestor_record_offset":12012,"inlinee":27086,"source_line":118,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary::dims_wall_space
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":1579,"record_offset":12876,"parent_record_offset":12848,"ancestor_record_offset":12012,"inlinee":27084,"source_line":241,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo::dims_wall_space
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":1579,"record_offset":12848,"parent_record_offset":12536,"ancestor_record_offset":12012,"inlinee":27083,"source_line":285,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::public_wall_state::PublicWallState::is_premade_shape
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_wall_state.rs
//     evidence={"module":1579,"record_offset":13120,"parent_record_offset":12536,"ancestor_record_offset":12012,"inlinee":26091,"source_line":128,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::contains_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":1579,"record_offset":22484,"parent_record_offset":22460,"ancestor_record_offset":12012,"inlinee":4279,"source_line":408,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::get_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":1579,"record_offset":22508,"parent_record_offset":22484,"ancestor_record_offset":12012,"inlinee":4280,"source_line":171,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::impl$2::contains::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":1579,"record_offset":22460,"parent_record_offset":22436,"ancestor_record_offset":12012,"inlinee":4278,"source_line":395,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] glam::bool::sse2::bvec4a::BVec4A::all
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/bool/sse2/bvec4a.rs
//     evidence={"module":1579,"record_offset":12500,"parent_record_offset":12364,"ancestor_record_offset":12012,"inlinee":27913,"source_line":91,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::mul_vec3
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/quat.rs
//     evidence={"module":1579,"record_offset":14848,"parent_record_offset":12536,"ancestor_record_offset":12012,"inlinee":11719,"source_line":788,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::mul_vec3a
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/quat.rs
//     evidence={"module":1579,"record_offset":14872,"parent_record_offset":14848,"ancestor_record_offset":12012,"inlinee":11724,"source_line":865,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::cross
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec3a.rs
//     evidence={"module":1579,"record_offset":15320,"parent_record_offset":14872,"ancestor_record_offset":12012,"inlinee":11725,"source_line":274,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec4::Vec4::is_finite
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec4.rs
//     evidence={"module":1579,"record_offset":12364,"parent_record_offset":12340,"ancestor_record_offset":12012,"inlinee":27907,"source_line":513,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec4::Vec4::is_finite_mask
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec4.rs
//     evidence={"module":1579,"record_offset":12388,"parent_record_offset":12364,"ancestor_record_offset":12012,"inlinee":27908,"source_line":522,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1579,"record_offset":17000,"parent_record_offset":16168,"ancestor_record_offset":12012,"inlinee":9814,"source_line":185,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1579,"record_offset":24248,"parent_record_offset":24224,"ancestor_record_offset":12012,"inlinee":9813,"source_line":461,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_recip
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1579,"record_offset":24224,"parent_record_offset":24192,"ancestor_record_offset":12012,"inlinee":9812,"source_line":480,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_squared
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1579,"record_offset":17084,"parent_record_offset":16168,"ancestor_record_offset":12012,"inlinee":26151,"source_line":471,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::lerp
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1579,"record_offset":14664,"parent_record_offset":14636,"ancestor_record_offset":12012,"inlinee":10989,"source_line":796,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::normalize_or
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1579,"record_offset":24192,"parent_record_offset":24164,"ancestor_record_offset":12012,"inlinee":9811,"source_line":564,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1579,"record_offset":14692,"parent_record_offset":14664,"ancestor_record_offset":12012,"inlinee":8712,"source_line":1309,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1579,"record_offset":14776,"parent_record_offset":14664,"ancestor_record_offset":12012,"inlinee":8716,"source_line":1394,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1579,"record_offset":16332,"parent_record_offset":16228,"ancestor_record_offset":12012,"inlinee":8708,"source_line":1529,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$54::sub_assign
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1579,"record_offset":17028,"parent_record_offset":16168,"ancestor_record_offset":12012,"inlinee":11675,"source_line":1563,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_in_x
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/sse2.rs
//     evidence={"module":1579,"record_offset":14944,"parent_record_offset":14916,"ancestor_record_offset":12012,"inlinee":11728,"source_line":42,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_into_m128
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/sse2.rs
//     evidence={"module":1579,"record_offset":14916,"parent_record_offset":14872,"ancestor_record_offset":12012,"inlinee":11727,"source_line":66,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::m128_abs
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/sse2.rs
//     evidence={"module":1579,"record_offset":12412,"parent_record_offset":12388,"ancestor_record_offset":12012,"inlinee":27909,"source_line":123,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxU32::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nonmax","version":"0.5.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":1579,"record_offset":23296,"parent_record_offset":23264,"ancestor_record_offset":12012,"inlinee":8634,"source_line":153,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nonmax","version":"0.5.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":1579,"record_offset":23100,"parent_record_offset":23076,"ancestor_record_offset":12012,"inlinee":4193,"source_line":153,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":1579,"record_offset":24312,"parent_record_offset":24280,"ancestor_record_offset":12012,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::collision_detection::DecoratorWorldSpace::project_to_wall_space
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/collision_detection.rs
//     evidence={"module":1579,"record_offset":13024,"parent_record_offset":12536,"ancestor_record_offset":12012,"inlinee":80548,"source_line":114,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::cull_oob_decorators::should_snap_to_corner
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/cull_oob_decorators.rs
//     evidence={"module":1579,"record_offset":13908,"parent_record_offset":12536,"ancestor_record_offset":12012,"inlinee":27691,"source_line":1231,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::ui_systems::decorator_interaction_intent::BalconySnapResult::get
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/ui_systems/decorator_interaction_intent.rs
//     evidence={"module":1579,"record_offset":15472,"parent_record_offset":12536,"ancestor_record_offset":12012,"inlinee":80560,"source_line":920,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_interaction_intent::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/ui_systems/decorator_interaction_intent.rs
//     evidence={"module":1579,"record_offset":21948,"parent_record_offset":21916,"ancestor_record_offset":12012,"inlinee":80562,"source_line":135,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_interaction_intent::closure$3
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/ui_systems/decorator_interaction_intent.rs
//     evidence={"module":1579,"record_offset":15972,"parent_record_offset":15888,"ancestor_record_offset":12012,"inlinee":80552,"source_line":362,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_wall_interaction_intent::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/ui_systems/decorator_interaction_intent.rs
//     evidence={"module":1579,"record_offset":13204,"parent_record_offset":13176,"ancestor_record_offset":12012,"inlinee":80550,"source_line":553,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_wall_interaction_intent::closure$2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/ui_systems/decorator_interaction_intent.rs
//     evidence={"module":1579,"record_offset":15532,"parent_record_offset":15504,"ancestor_record_offset":12012,"inlinee":80561,"source_line":633,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::ui_systems::decorator_interaction_intent::find_closest_norm_curve_t::closure$0
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/ui_systems/decorator_interaction_intent.rs
//     evidence={"module":1579,"record_offset":16168,"parent_record_offset":16060,"ancestor_record_offset":12012,"inlinee":80555,"source_line":418,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::ui_systems::decorator_interaction_intent::find_closest_norm_curve_t::closure$1
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/ui_systems/decorator_interaction_intent.rs
//     evidence={"module":1579,"record_offset":18172,"parent_record_offset":18092,"ancestor_record_offset":12012,"inlinee":80557,"source_line":436,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::ui_systems::decorator_interaction_intent::find_closest_norm_curve_t::closure$2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/ui_systems/decorator_interaction_intent.rs
//     evidence={"module":1579,"record_offset":18092,"parent_record_offset":16060,"ancestor_record_offset":12012,"inlinee":80556,"source_line":441,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::ui_systems::decorator_interaction_intent::find_closest_norm_curve_t::closure$3
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/ui_systems/decorator_interaction_intent.rs
//     evidence={"module":1579,"record_offset":21368,"parent_record_offset":21280,"ancestor_record_offset":12012,"inlinee":80558,"source_line":455,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::get_coord_at_u
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1579,"record_offset":17216,"parent_record_offset":17172,"ancestor_record_offset":12012,"inlinee":14735,"source_line":687,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::get_pos_at_coord
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1579,"record_offset":14388,"parent_record_offset":14364,"ancestor_record_offset":12012,"inlinee":14727,"source_line":799,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::get_pos_at_u
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1579,"record_offset":14364,"parent_record_offset":12536,"ancestor_record_offset":12012,"inlinee":26304,"source_line":739,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i0
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1579,"record_offset":14412,"parent_record_offset":14388,"ancestor_record_offset":12012,"inlinee":8705,"source_line":1332,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i1
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1579,"record_offset":14524,"parent_record_offset":14388,"ancestor_record_offset":12012,"inlinee":8706,"source_line":1337,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u::CurveU::new
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":1579,"record_offset":18416,"parent_record_offset":18224,"ancestor_record_offset":12012,"inlinee":9794,"source_line":89,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u::CurveUBoundless::clamp
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":1579,"record_offset":13056,"parent_record_offset":13024,"ancestor_record_offset":12012,"inlinee":26089,"source_line":207,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::impl$12::sample_at_coord
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1579,"record_offset":16228,"parent_record_offset":16168,"ancestor_record_offset":12012,"inlinee":8731,"source_line":1392,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle3d::height
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":1579,"record_offset":12956,"parent_record_offset":12932,"ancestor_record_offset":12012,"inlinee":11330,"source_line":79,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle3d::width
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":1579,"record_offset":12984,"parent_record_offset":12932,"ancestor_record_offset":12012,"inlinee":11328,"source_line":84,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$0::lerp
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":1579,"record_offset":14636,"parent_record_offset":14388,"ancestor_record_offset":12012,"inlinee":14719,"source_line":78,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] utils::wall_space::WallSpaceCoord::new
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/wall_space.rs
//     evidence={"module":1579,"record_offset":13880,"parent_record_offset":12536,"ancestor_record_offset":12012,"inlinee":12108,"source_line":116,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] utils::wall_space::WallSpaceCoord::subtract
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/wall_space.rs
//     evidence={"module":1579,"record_offset":13952,"parent_record_offset":13908,"ancestor_record_offset":12012,"inlinee":27684,"source_line":284,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] utils::wall_space::WallSpaceCoord::with_offset
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/wall_space.rs
//     evidence={"module":1579,"record_offset":15564,"parent_record_offset":12536,"ancestor_record_offset":12012,"inlinee":27690,"source_line":213,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_interaction_intent::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 135
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x13AB2
// 首条位置: {"module":1579,"inlinee":80562}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_interaction_intent::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 266
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1CCAB
// 首条位置: {"module":3184,"inlinee":117931}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":3184,"record_offset":6160,"parent_record_offset":6140,"ancestor_record_offset":6140,"inlinee":4487,"source_line":264,"archive":"_pdb/C/modules/03184.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator::impl$33::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":3184,"record_offset":6184,"parent_record_offset":6160,"ancestor_record_offset":6140,"inlinee":7427,"source_line":155,"archive":"_pdb/C/modules/03184.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_interaction_intent::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 275
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x13AB3
// 首条位置: {"module":1579,"inlinee":80563}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_interaction_intent::closure$3
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 362
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x13AA8
// 首条位置: {"module":1579,"inlinee":80552}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::resources::stairs::stairs_state::impl$17::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/stairs/stairs_state.rs
//     evidence={"module":1579,"record_offset":15996,"parent_record_offset":15972,"ancestor_record_offset":15972,"inlinee":8701,"source_line":38,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator::impl$33::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":1579,"record_offset":16020,"parent_record_offset":15996,"ancestor_record_offset":15972,"inlinee":7427,"source_line":155,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_interaction_intent::closure$4
// 记录形式: compiled_procedure; instances=1
// 源码行: 363
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1579,"record_offset":26036,"rva":48039632,"va":5416748752}
// 编译实例 RVAs: 0x2dd06d0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-41070159","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5416748752,"raw_hex":"e8b1518dfd","size":5,"soft_fail":false,"target_va":5375678688,"va":5416748842,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-41070466","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5416748752,"raw_hex":"e87e508dfd","size":5,"soft_fail":false,"target_va":5375678704,"va":5416749165,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=1
//     target_aliases=1699 candidates; complete list: decorator_interaction_intent.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-48026642","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5416748752,"raw_hex":"e8ee2b23fd","size":5,"soft_fail":false,"target_va":5368722192,"va":5416748829,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c663b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decorator_interaction_intent.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-35038035","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5416748752,"raw_hex":"e8ad5ce9fd","size":5,"soft_fail":false,"target_va":5381710768,"va":5416748798,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-128071","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5416748752,"raw_hex":"e8b90bfeff","size":5,"soft_fail":false,"target_va":5416620794,"va":5416748860,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-126862","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5416748752,"raw_hex":"e87210feff","size":5,"soft_fail":false,"target_va":5416622240,"va":5416749097,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1579,"record_offset":27016,"parent_record_offset":26036,"ancestor_record_offset":26036,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_direct] itertools::Itertools::collect_vec
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/lib.rs
//     evidence={"module":1579,"record_offset":26236,"parent_record_offset":26036,"ancestor_record_offset":26036,"inlinee":8460,"source_line":2348,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1579,"record_offset":26520,"parent_record_offset":26496,"ancestor_record_offset":26036,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1579,"record_offset":26544,"parent_record_offset":26520,"ancestor_record_offset":26036,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1579,"record_offset":27204,"parent_record_offset":27180,"ancestor_record_offset":26036,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1579,"record_offset":27228,"parent_record_offset":27204,"ancestor_record_offset":26036,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1579,"record_offset":26568,"parent_record_offset":26544,"ancestor_record_offset":26036,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1579,"record_offset":27252,"parent_record_offset":27228,"ancestor_record_offset":26036,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1579,"record_offset":26496,"parent_record_offset":26464,"ancestor_record_offset":26036,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1579,"record_offset":27180,"parent_record_offset":27088,"ancestor_record_offset":26036,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1579,"record_offset":26412,"parent_record_offset":26388,"ancestor_record_offset":26036,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1579,"record_offset":27116,"parent_record_offset":27088,"ancestor_record_offset":26036,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1579,"record_offset":27088,"parent_record_offset":27064,"ancestor_record_offset":26036,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1579,"record_offset":26464,"parent_record_offset":26436,"ancestor_record_offset":26036,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1579,"record_offset":26436,"parent_record_offset":26412,"ancestor_record_offset":26036,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1579,"record_offset":27064,"parent_record_offset":27040,"ancestor_record_offset":26036,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1579,"record_offset":26652,"parent_record_offset":26628,"ancestor_record_offset":26036,"inlinee":5079,"source_line":3988,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1579,"record_offset":26364,"parent_record_offset":26332,"ancestor_record_offset":26036,"inlinee":5351,"source_line":523,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1579,"record_offset":26388,"parent_record_offset":26364,"ancestor_record_offset":26036,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1579,"record_offset":26284,"parent_record_offset":26260,"ancestor_record_offset":26036,"inlinee":5071,"source_line":3864,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$21::extend_trusted::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1579,"record_offset":26844,"parent_record_offset":26804,"ancestor_record_offset":26036,"inlinee":5113,"source_line":4001,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::increment_len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":1579,"record_offset":26936,"parent_record_offset":26844,"ancestor_record_offset":26036,"inlinee":5115,"source_line":18,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_extend.rs
//     evidence={"module":1579,"record_offset":26628,"parent_record_offset":26332,"ancestor_record_offset":26036,"inlinee":5078,"source_line":26,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter::impl$0::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_from_iter.rs
//     evidence={"module":1579,"record_offset":26308,"parent_record_offset":26284,"ancestor_record_offset":26036,"inlinee":6133,"source_line":32,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter_nested::impl$1::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_from_iter_nested.rs
//     evidence={"module":1579,"record_offset":26332,"parent_record_offset":26308,"ancestor_record_offset":26036,"inlinee":6135,"source_line":50,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":1579,"record_offset":26700,"parent_record_offset":26676,"ancestor_record_offset":26036,"inlinee":6631,"source_line":124,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":1579,"record_offset":26764,"parent_record_offset":26724,"ancestor_record_offset":26036,"inlinee":5396,"source_line":88,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1579,"record_offset":26260,"parent_record_offset":26236,"ancestor_record_offset":26036,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1579,"record_offset":26676,"parent_record_offset":26652,"ancestor_record_offset":26036,"inlinee":5042,"source_line":835,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1579,"record_offset":26804,"parent_record_offset":26764,"ancestor_record_offset":26036,"inlinee":5057,"source_line":842,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1579,"record_offset":27148,"parent_record_offset":27116,"ancestor_record_offset":26036,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1579,"record_offset":27040,"parent_record_offset":27016,"ancestor_record_offset":26036,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1579,"record_offset":26892,"parent_record_offset":26844,"ancestor_record_offset":26036,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1579,"record_offset":26724,"parent_record_offset":26700,"ancestor_record_offset":26036,"inlinee":6636,"source_line":259,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::ui_systems::decorator_interaction_intent::find_closest_norm_curve_t
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 408
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x13AA9
// 首条位置: {"module":1579,"inlinee":80553}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::cmp::Ord::min
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1579,"record_offset":21652,"parent_record_offset":16060,"ancestor_record_offset":16060,"inlinee":6426,"source_line":1072,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1579,"record_offset":21772,"parent_record_offset":16060,"ancestor_record_offset":16060,"inlinee":5477,"source_line":991,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::min
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1579,"record_offset":21744,"parent_record_offset":16060,"ancestor_record_offset":16060,"inlinee":5478,"source_line":1018,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_direct] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":1579,"record_offset":18008,"parent_record_offset":16060,"ancestor_record_offset":16060,"inlinee":5171,"source_line":856,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::min_by
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1579,"record_offset":20916,"parent_record_offset":16060,"ancestor_record_offset":16060,"inlinee":9019,"source_line":3384,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_direct] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1579,"record_offset":21624,"parent_record_offset":16060,"ancestor_record_offset":16060,"inlinee":9345,"source_line":2344,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_direct] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":1579,"record_offset":21800,"parent_record_offset":16060,"ancestor_record_offset":16060,"inlinee":9795,"source_line":522,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::ui_systems::decorator_interaction_intent::find_closest_norm_curve_t::closure$0
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/ui_systems/decorator_interaction_intent.rs
//     evidence={"module":1579,"record_offset":16168,"parent_record_offset":16060,"ancestor_record_offset":16060,"inlinee":80555,"source_line":418,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::ui_systems::decorator_interaction_intent::find_closest_norm_curve_t::closure$1
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/ui_systems/decorator_interaction_intent.rs
//     evidence={"module":1579,"record_offset":21680,"parent_record_offset":16060,"ancestor_record_offset":16060,"inlinee":80557,"source_line":436,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::ui_systems::decorator_interaction_intent::find_closest_norm_curve_t::closure$2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/ui_systems/decorator_interaction_intent.rs
//     evidence={"module":1579,"record_offset":18092,"parent_record_offset":16060,"ancestor_record_offset":16060,"inlinee":80556,"source_line":441,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$14::index
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1579,"record_offset":16592,"parent_record_offset":16560,"ancestor_record_offset":16060,"inlinee":5465,"source_line":3803,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1579,"record_offset":18488,"parent_record_offset":18464,"ancestor_record_offset":16060,"inlinee":6833,"source_line":2145,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::le
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1579,"record_offset":18512,"parent_record_offset":18488,"ancestor_record_offset":16060,"inlinee":8914,"source_line":1918,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::partial_cmp
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1579,"record_offset":21516,"parent_record_offset":21368,"ancestor_record_offset":16060,"inlinee":11574,"source_line":1957,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$76::lt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1579,"record_offset":18056,"parent_record_offset":18032,"ancestor_record_offset":16060,"inlinee":7964,"source_line":1916,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::min_by
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1579,"record_offset":21164,"parent_record_offset":21136,"ancestor_record_offset":16060,"inlinee":5404,"source_line":1605,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::abs
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1579,"record_offset":17140,"parent_record_offset":16168,"ancestor_record_offset":16060,"inlinee":11731,"source_line":1569,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::copied::copy_fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\copied.rs
//     evidence={"module":1579,"record_offset":21076,"parent_record_offset":21044,"ancestor_record_offset":16060,"inlinee":10907,"source_line":36,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::copied::impl$1::fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\copied.rs
//     evidence={"module":1579,"record_offset":21012,"parent_record_offset":20980,"ancestor_record_offset":16060,"inlinee":10905,"source_line":77,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":1579,"record_offset":20980,"parent_record_offset":20948,"ancestor_record_offset":16060,"inlinee":10794,"source_line":129,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::fold::enumerate::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":1579,"record_offset":21104,"parent_record_offset":21076,"ancestor_record_offset":16060,"inlinee":5430,"source_line":139,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":1579,"record_offset":18032,"parent_record_offset":18008,"ancestor_record_offset":16060,"inlinee":5173,"source_line":771,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::min_by::fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1579,"record_offset":21136,"parent_record_offset":21104,"ancestor_record_offset":16060,"inlinee":5402,"source_line":3391,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::reduce
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1579,"record_offset":20948,"parent_record_offset":20916,"ancestor_record_offset":16060,"inlinee":9020,"source_line":2678,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1579,"record_offset":16468,"parent_record_offset":16444,"ancestor_record_offset":16060,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1579,"record_offset":16412,"parent_record_offset":16388,"ancestor_record_offset":16060,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$385::sub_assign
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1579,"record_offset":17052,"parent_record_offset":17028,"ancestor_record_offset":16060,"inlinee":11677,"source_line":862,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1579,"record_offset":16356,"parent_record_offset":16332,"ancestor_record_offset":16060,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":1579,"record_offset":21280,"parent_record_offset":21164,"ancestor_record_offset":16060,"inlinee":5124,"source_line":309,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":1579,"record_offset":18464,"parent_record_offset":18440,"ancestor_record_offset":16060,"inlinee":6826,"source_line":868,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":1579,"record_offset":18440,"parent_record_offset":18416,"ancestor_record_offset":16060,"inlinee":9795,"source_line":522,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1579,"record_offset":21456,"parent_record_offset":21368,"ancestor_record_offset":16060,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=16
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1579,"record_offset":16268,"parent_record_offset":16228,"ancestor_record_offset":16060,"inlinee":5467,"source_line":18,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=16
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1579,"record_offset":16296,"parent_record_offset":16268,"ancestor_record_offset":16060,"inlinee":5468,"source_line":270,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1579,"record_offset":21044,"parent_record_offset":21012,"ancestor_record_offset":16060,"inlinee":6636,"source_line":259,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1579,"record_offset":17000,"parent_record_offset":16168,"ancestor_record_offset":16060,"inlinee":9814,"source_line":185,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_squared
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1579,"record_offset":17084,"parent_record_offset":16168,"ancestor_record_offset":16060,"inlinee":26151,"source_line":471,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::lerp
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1579,"record_offset":16712,"parent_record_offset":16688,"ancestor_record_offset":16060,"inlinee":10989,"source_line":796,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1579,"record_offset":16388,"parent_record_offset":16228,"ancestor_record_offset":16060,"inlinee":8712,"source_line":1309,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1579,"record_offset":16444,"parent_record_offset":16228,"ancestor_record_offset":16060,"inlinee":8716,"source_line":1394,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1579,"record_offset":16332,"parent_record_offset":16228,"ancestor_record_offset":16060,"inlinee":8708,"source_line":1529,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$54::sub_assign
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1579,"record_offset":17028,"parent_record_offset":16168,"ancestor_record_offset":16060,"inlinee":11675,"source_line":1563,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::ui_systems::decorator_interaction_intent::find_closest_norm_curve_t::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/ui_systems/decorator_interaction_intent.rs
//     evidence={"module":1579,"record_offset":18224,"parent_record_offset":18092,"ancestor_record_offset":16060,"inlinee":80555,"source_line":418,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::ui_systems::decorator_interaction_intent::find_closest_norm_curve_t::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/ui_systems/decorator_interaction_intent.rs
//     evidence={"module":1579,"record_offset":18172,"parent_record_offset":18092,"ancestor_record_offset":16060,"inlinee":80557,"source_line":436,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::ui_systems::decorator_interaction_intent::find_closest_norm_curve_t::closure$3
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/ui_systems/decorator_interaction_intent.rs
//     evidence={"module":1579,"record_offset":21368,"parent_record_offset":21280,"ancestor_record_offset":16060,"inlinee":80558,"source_line":455,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::get_coord_at_u
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1579,"record_offset":17216,"parent_record_offset":17172,"ancestor_record_offset":16060,"inlinee":14735,"source_line":687,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::get_pos_at_coord
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1579,"record_offset":16560,"parent_record_offset":16168,"ancestor_record_offset":16060,"inlinee":14727,"source_line":799,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i0
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1579,"record_offset":18584,"parent_record_offset":18556,"ancestor_record_offset":16060,"inlinee":8705,"source_line":1332,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i1
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1579,"record_offset":17328,"parent_record_offset":17244,"ancestor_record_offset":16060,"inlinee":8706,"source_line":1337,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u::CurveU::new
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":1579,"record_offset":18416,"parent_record_offset":18224,"ancestor_record_offset":16060,"inlinee":9794,"source_line":89,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::impl$12::sample_at_coord
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1579,"record_offset":16228,"parent_record_offset":16168,"ancestor_record_offset":16060,"inlinee":8731,"source_line":1392,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$0::lerp
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":1579,"record_offset":16688,"parent_record_offset":16560,"ancestor_record_offset":16060,"inlinee":14719,"source_line":78,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::ui_systems::decorator_interaction_intent::find_closest_norm_curve_t::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 418
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x13AAB
// 首条位置: {"module":1579,"inlinee":80555}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::f32::impl$0::abs
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1579,"record_offset":17140,"parent_record_offset":16168,"ancestor_record_offset":16168,"inlinee":11731,"source_line":1569,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1579,"record_offset":17000,"parent_record_offset":16168,"ancestor_record_offset":16168,"inlinee":9814,"source_line":185,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::length_squared
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1579,"record_offset":17084,"parent_record_offset":16168,"ancestor_record_offset":16168,"inlinee":26151,"source_line":471,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1579,"record_offset":16944,"parent_record_offset":16168,"ancestor_record_offset":16168,"inlinee":8708,"source_line":1529,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$54::sub_assign
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1579,"record_offset":17028,"parent_record_offset":16168,"ancestor_record_offset":16168,"inlinee":11675,"source_line":1563,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::get_coord_at_u
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1579,"record_offset":17216,"parent_record_offset":17172,"ancestor_record_offset":17172,"inlinee":14735,"source_line":687,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::get_pos_at_coord
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1579,"record_offset":16560,"parent_record_offset":16168,"ancestor_record_offset":16168,"inlinee":14727,"source_line":799,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u::CurveU::new
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":1579,"record_offset":18416,"parent_record_offset":18224,"ancestor_record_offset":18224,"inlinee":9794,"source_line":89,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::impl$12::sample_at_coord
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1579,"record_offset":16228,"parent_record_offset":16168,"ancestor_record_offset":16168,"inlinee":8731,"source_line":1392,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$14::index
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1579,"record_offset":16592,"parent_record_offset":16560,"ancestor_record_offset":16168,"inlinee":5465,"source_line":3803,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1579,"record_offset":18488,"parent_record_offset":18464,"ancestor_record_offset":18224,"inlinee":6833,"source_line":2145,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::le
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1579,"record_offset":18512,"parent_record_offset":18488,"ancestor_record_offset":18224,"inlinee":8914,"source_line":1918,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1579,"record_offset":16468,"parent_record_offset":16444,"ancestor_record_offset":16168,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1579,"record_offset":16412,"parent_record_offset":16388,"ancestor_record_offset":16168,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$385::sub_assign
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1579,"record_offset":17052,"parent_record_offset":17028,"ancestor_record_offset":16168,"inlinee":11677,"source_line":862,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1579,"record_offset":16356,"parent_record_offset":16332,"ancestor_record_offset":16168,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":1579,"record_offset":18464,"parent_record_offset":18440,"ancestor_record_offset":18224,"inlinee":6826,"source_line":868,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":1579,"record_offset":18440,"parent_record_offset":18416,"ancestor_record_offset":18224,"inlinee":9795,"source_line":522,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=16
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1579,"record_offset":16268,"parent_record_offset":16228,"ancestor_record_offset":16168,"inlinee":5467,"source_line":18,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=16
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1579,"record_offset":16296,"parent_record_offset":16268,"ancestor_record_offset":16168,"inlinee":5468,"source_line":270,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1579,"record_offset":17108,"parent_record_offset":17084,"ancestor_record_offset":16168,"inlinee":9814,"source_line":185,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::lerp
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1579,"record_offset":16712,"parent_record_offset":16688,"ancestor_record_offset":16168,"inlinee":10989,"source_line":796,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1579,"record_offset":16388,"parent_record_offset":16228,"ancestor_record_offset":16168,"inlinee":8712,"source_line":1309,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1579,"record_offset":16444,"parent_record_offset":16228,"ancestor_record_offset":16168,"inlinee":8716,"source_line":1394,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1579,"record_offset":16332,"parent_record_offset":16228,"ancestor_record_offset":16168,"inlinee":8708,"source_line":1529,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i0
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1579,"record_offset":18584,"parent_record_offset":18556,"ancestor_record_offset":18224,"inlinee":8705,"source_line":1332,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i1
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1579,"record_offset":17328,"parent_record_offset":17244,"ancestor_record_offset":17172,"inlinee":8706,"source_line":1337,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$0::lerp
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":1579,"record_offset":16688,"parent_record_offset":16560,"ancestor_record_offset":16168,"inlinee":14719,"source_line":78,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::ui_systems::decorator_interaction_intent::find_closest_norm_curve_t::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 436
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x13AAD
// 首条位置: {"module":1579,"inlinee":80557}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::ui_systems::decorator_interaction_intent::find_closest_norm_curve_t::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 441
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x13AAC
// 首条位置: {"module":1579,"inlinee":80556}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] system_decorator::ui_systems::decorator_interaction_intent::find_closest_norm_curve_t::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/ui_systems/decorator_interaction_intent.rs
//     evidence={"module":1579,"record_offset":18224,"parent_record_offset":18092,"ancestor_record_offset":18092,"inlinee":80555,"source_line":418,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::ui_systems::decorator_interaction_intent::find_closest_norm_curve_t::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/ui_systems/decorator_interaction_intent.rs
//     evidence={"module":1579,"record_offset":18172,"parent_record_offset":18092,"ancestor_record_offset":18092,"inlinee":80557,"source_line":436,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$14::index
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1579,"record_offset":18780,"parent_record_offset":18756,"ancestor_record_offset":18092,"inlinee":5465,"source_line":3803,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1579,"record_offset":18488,"parent_record_offset":18464,"ancestor_record_offset":18092,"inlinee":6833,"source_line":2145,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::le
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1579,"record_offset":18512,"parent_record_offset":18488,"ancestor_record_offset":18092,"inlinee":8914,"source_line":1918,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::abs
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1579,"record_offset":19520,"parent_record_offset":18224,"ancestor_record_offset":18092,"inlinee":11731,"source_line":1569,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1579,"record_offset":19112,"parent_record_offset":19088,"ancestor_record_offset":18092,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1579,"record_offset":19028,"parent_record_offset":19004,"ancestor_record_offset":18092,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$385::sub_assign
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1579,"record_offset":19360,"parent_record_offset":19312,"ancestor_record_offset":18092,"inlinee":11677,"source_line":862,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1579,"record_offset":19204,"parent_record_offset":19156,"ancestor_record_offset":18092,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":1579,"record_offset":18464,"parent_record_offset":18440,"ancestor_record_offset":18092,"inlinee":6826,"source_line":868,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":1579,"record_offset":18440,"parent_record_offset":18416,"ancestor_record_offset":18092,"inlinee":9795,"source_line":522,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1579,"record_offset":18612,"parent_record_offset":18556,"ancestor_record_offset":18092,"inlinee":5467,"source_line":18,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1579,"record_offset":18636,"parent_record_offset":18612,"ancestor_record_offset":18092,"inlinee":5468,"source_line":270,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1579,"record_offset":19260,"parent_record_offset":18224,"ancestor_record_offset":18092,"inlinee":9814,"source_line":185,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_squared
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1579,"record_offset":19416,"parent_record_offset":18224,"ancestor_record_offset":18092,"inlinee":26151,"source_line":471,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::lerp
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1579,"record_offset":18976,"parent_record_offset":18948,"ancestor_record_offset":18092,"inlinee":10989,"source_line":796,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1579,"record_offset":19004,"parent_record_offset":18976,"ancestor_record_offset":18092,"inlinee":8712,"source_line":1309,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1579,"record_offset":19088,"parent_record_offset":18976,"ancestor_record_offset":18092,"inlinee":8716,"source_line":1394,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1579,"record_offset":19156,"parent_record_offset":18224,"ancestor_record_offset":18092,"inlinee":8708,"source_line":1529,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$54::sub_assign
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1579,"record_offset":19312,"parent_record_offset":18224,"ancestor_record_offset":18092,"inlinee":11675,"source_line":1563,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::get_pos_at_coord
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1579,"record_offset":18756,"parent_record_offset":18224,"ancestor_record_offset":18092,"inlinee":14727,"source_line":799,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i0
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1579,"record_offset":18584,"parent_record_offset":18556,"ancestor_record_offset":18092,"inlinee":8705,"source_line":1332,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i1
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1579,"record_offset":18668,"parent_record_offset":18556,"ancestor_record_offset":18092,"inlinee":8706,"source_line":1337,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u::CurveU::new
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":1579,"record_offset":18416,"parent_record_offset":18224,"ancestor_record_offset":18092,"inlinee":9794,"source_line":89,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::impl$12::sample_at_coord
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1579,"record_offset":18556,"parent_record_offset":18224,"ancestor_record_offset":18092,"inlinee":8731,"source_line":1392,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$0::lerp
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":1579,"record_offset":18948,"parent_record_offset":18756,"ancestor_record_offset":18092,"inlinee":14719,"source_line":78,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::ui_systems::decorator_interaction_intent::find_closest_norm_curve_t::closure$3
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 455
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x13AAE
// 首条位置: {"module":1579,"inlinee":80558}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::cmp::impls::impl$53::partial_cmp
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1579,"record_offset":21516,"parent_record_offset":21368,"ancestor_record_offset":21368,"inlinee":11574,"source_line":1957,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1579,"record_offset":21456,"parent_record_offset":21368,"ancestor_record_offset":21368,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_wall_interaction_intent
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 477
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x13AA2
// 首条位置: {"module":1579,"inlinee":80546}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1579,"record_offset":13148,"parent_record_offset":12536,"ancestor_record_offset":12536,"inlinee":6153,"source_line":463,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::and_then
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1579,"record_offset":13176,"parent_record_offset":12536,"ancestor_record_offset":12536,"inlinee":5449,"source_line":1541,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_none
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1579,"record_offset":13532,"parent_record_offset":12536,"ancestor_record_offset":12536,"inlinee":4877,"source_line":684,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_some_and
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1579,"record_offset":15504,"parent_record_offset":12536,"ancestor_record_offset":12536,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1579,"record_offset":12812,"parent_record_offset":12536,"ancestor_record_offset":12536,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1579,"record_offset":13240,"parent_record_offset":12536,"ancestor_record_offset":12536,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo::dims_wall_space
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":1579,"record_offset":12848,"parent_record_offset":12536,"ancestor_record_offset":12536,"inlinee":27083,"source_line":285,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::public_wall_state::PublicWallState::is_premade_shape
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_wall_state.rs
//     evidence={"module":1579,"record_offset":13120,"parent_record_offset":12536,"ancestor_record_offset":12536,"inlinee":26091,"source_line":128,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::quat::Quat::mul_vec3
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/quat.rs
//     evidence={"module":1579,"record_offset":14848,"parent_record_offset":12536,"ancestor_record_offset":12536,"inlinee":11719,"source_line":788,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::collision_detection::DecoratorWorldSpace::project_to_wall_space
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/collision_detection.rs
//     evidence={"module":1579,"record_offset":13024,"parent_record_offset":12536,"ancestor_record_offset":12536,"inlinee":80548,"source_line":114,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::cull_oob_decorators::should_snap_to_corner
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/cull_oob_decorators.rs
//     evidence={"module":1579,"record_offset":13908,"parent_record_offset":12536,"ancestor_record_offset":12536,"inlinee":27691,"source_line":1231,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::ui_systems::decorator_interaction_intent::BalconySnapResult::get
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/ui_systems/decorator_interaction_intent.rs
//     evidence={"module":1579,"record_offset":15472,"parent_record_offset":12536,"ancestor_record_offset":12536,"inlinee":80560,"source_line":920,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::get_pos_at_u
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1579,"record_offset":14364,"parent_record_offset":12536,"ancestor_record_offset":12536,"inlinee":26304,"source_line":739,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u::CurveUBoundless::clamp
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":1579,"record_offset":14304,"parent_record_offset":12536,"ancestor_record_offset":12536,"inlinee":26089,"source_line":207,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_direct] utils::wall_space::WallSpaceCoord::new
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/wall_space.rs
//     evidence={"module":1579,"record_offset":13880,"parent_record_offset":12536,"ancestor_record_offset":12536,"inlinee":12108,"source_line":116,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_direct] utils::wall_space::WallSpaceCoord::with_offset
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/wall_space.rs
//     evidence={"module":1579,"record_offset":15564,"parent_record_offset":12536,"ancestor_record_offset":12536,"inlinee":27690,"source_line":213,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1579,"record_offset":13428,"parent_record_offset":13404,"ancestor_record_offset":12536,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1579,"record_offset":13452,"parent_record_offset":13428,"ancestor_record_offset":12536,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1579,"record_offset":13476,"parent_record_offset":13452,"ancestor_record_offset":12536,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1579,"record_offset":13404,"parent_record_offset":13312,"ancestor_record_offset":12536,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1579,"record_offset":13340,"parent_record_offset":13312,"ancestor_record_offset":12536,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1579,"record_offset":13312,"parent_record_offset":13288,"ancestor_record_offset":12536,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1579,"record_offset":13288,"parent_record_offset":13264,"ancestor_record_offset":12536,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$14::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1579,"record_offset":14440,"parent_record_offset":14388,"ancestor_record_offset":12536,"inlinee":5465,"source_line":3803,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1579,"record_offset":15292,"parent_record_offset":14872,"ancestor_record_offset":12536,"inlinee":7191,"source_line":35,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ss
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1579,"record_offset":15200,"parent_record_offset":15144,"ancestor_record_offset":12536,"inlinee":11729,"source_line":22,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1579,"record_offset":14968,"parent_record_offset":14944,"ancestor_record_offset":12536,"inlinee":7190,"source_line":87,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1579,"record_offset":15000,"parent_record_offset":14916,"ancestor_record_offset":12536,"inlinee":11716,"source_line":1045,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_sub_ps
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1579,"record_offset":15064,"parent_record_offset":14872,"ancestor_record_offset":12536,"inlinee":11730,"source_line":61,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::abs
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1579,"record_offset":13980,"parent_record_offset":13908,"ancestor_record_offset":12536,"inlinee":11731,"source_line":1569,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1579,"record_offset":13080,"parent_record_offset":13056,"ancestor_record_offset":12536,"inlinee":9255,"source_line":1503,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1579,"record_offset":13372,"parent_record_offset":13340,"ancestor_record_offset":12536,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1579,"record_offset":14800,"parent_record_offset":14776,"ancestor_record_offset":12536,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1579,"record_offset":14716,"parent_record_offset":14692,"ancestor_record_offset":12536,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1579,"record_offset":13556,"parent_record_offset":13532,"ancestor_record_offset":12536,"inlinee":4878,"source_line":635,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1579,"record_offset":12904,"parent_record_offset":12876,"ancestor_record_offset":12536,"inlinee":4428,"source_line":1061,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1579,"record_offset":13264,"parent_record_offset":13240,"ancestor_record_offset":12536,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1579,"record_offset":14464,"parent_record_offset":14440,"ancestor_record_offset":12536,"inlinee":5467,"source_line":18,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1579,"record_offset":14488,"parent_record_offset":14464,"ancestor_record_offset":12536,"inlinee":5468,"source_line":270,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::DecoratorCollision::dims_wall_space
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":1579,"record_offset":12932,"parent_record_offset":12876,"ancestor_record_offset":12536,"inlinee":27086,"source_line":118,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary::dims_wall_space
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":1579,"record_offset":12876,"parent_record_offset":12848,"ancestor_record_offset":12536,"inlinee":27084,"source_line":241,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::mul_vec3a
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/quat.rs
//     evidence={"module":1579,"record_offset":14872,"parent_record_offset":14848,"ancestor_record_offset":12536,"inlinee":11724,"source_line":865,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::cross
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec3a.rs
//     evidence={"module":1579,"record_offset":15320,"parent_record_offset":14872,"ancestor_record_offset":12536,"inlinee":11725,"source_line":274,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1579,"record_offset":14664,"parent_record_offset":14636,"ancestor_record_offset":12536,"inlinee":10989,"source_line":796,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1579,"record_offset":14692,"parent_record_offset":14664,"ancestor_record_offset":12536,"inlinee":8712,"source_line":1309,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1579,"record_offset":14776,"parent_record_offset":14664,"ancestor_record_offset":12536,"inlinee":8716,"source_line":1394,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_in_x
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/sse2.rs
//     evidence={"module":1579,"record_offset":14944,"parent_record_offset":14916,"ancestor_record_offset":12536,"inlinee":11728,"source_line":42,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_into_m128
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/sse2.rs
//     evidence={"module":1579,"record_offset":14916,"parent_record_offset":14872,"ancestor_record_offset":12536,"inlinee":11727,"source_line":66,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_wall_interaction_intent::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/ui_systems/decorator_interaction_intent.rs
//     evidence={"module":1579,"record_offset":13204,"parent_record_offset":13176,"ancestor_record_offset":12536,"inlinee":80550,"source_line":553,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_wall_interaction_intent::closure$2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/ui_systems/decorator_interaction_intent.rs
//     evidence={"module":1579,"record_offset":15532,"parent_record_offset":15504,"ancestor_record_offset":12536,"inlinee":80561,"source_line":633,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::get_pos_at_coord
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1579,"record_offset":14388,"parent_record_offset":14364,"ancestor_record_offset":12536,"inlinee":14727,"source_line":799,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1579,"record_offset":14412,"parent_record_offset":14388,"ancestor_record_offset":12536,"inlinee":8705,"source_line":1332,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1579,"record_offset":14524,"parent_record_offset":14388,"ancestor_record_offset":12536,"inlinee":8706,"source_line":1337,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u::CurveUBoundless::clamp
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":1579,"record_offset":13056,"parent_record_offset":13024,"ancestor_record_offset":12536,"inlinee":26089,"source_line":207,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle3d::height
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":1579,"record_offset":12956,"parent_record_offset":12932,"ancestor_record_offset":12536,"inlinee":11330,"source_line":79,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle3d::width
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":1579,"record_offset":12984,"parent_record_offset":12932,"ancestor_record_offset":12536,"inlinee":11328,"source_line":84,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$0::lerp
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":1579,"record_offset":14636,"parent_record_offset":14388,"ancestor_record_offset":12536,"inlinee":14719,"source_line":78,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   [pdb_inline_transitive] utils::wall_space::WallSpaceCoord::subtract
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/wall_space.rs
//     evidence={"module":1579,"record_offset":13952,"parent_record_offset":13908,"ancestor_record_offset":12536,"inlinee":27684,"source_line":284,"archive":"_pdb/C/modules/01579.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_wall_interaction_intent::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 546
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1CCB9
// 首条位置: {"module":3185,"inlinee":117945}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":3185,"record_offset":9104,"parent_record_offset":9084,"ancestor_record_offset":9084,"inlinee":4487,"source_line":264,"archive":"_pdb/C/modules/03185.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator::impl$33::eq
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":3185,"record_offset":9128,"parent_record_offset":9104,"ancestor_record_offset":9084,"inlinee":7427,"source_line":155,"archive":"_pdb/C/modules/03185.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_wall_interaction_intent::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 553
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x13AA6
// 首条位置: {"module":1579,"inlinee":80550}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_wall_interaction_intent::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 633
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x13AB1
// 首条位置: {"module":1579,"inlinee":80561}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::ui_systems::decorator_interaction_intent::BalconySnapResult::get
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 920
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x13AB0
// 首条位置: {"module":1579,"inlinee":80560}
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
// 完整结构及原始类型记录: decorator_interaction_intent.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] system_decorator::ui_systems::decorator_interaction_intent::BalconySnapResult
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 2; 过程引用数: 0
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: decorator_interaction_intent.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: decorator_interaction_intent.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: system_decorator::ui_systems::decorator_interaction_intent::BalconySnapResult::get
// 信息 ID: 0bb8f40064645920238a82ae
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_interaction_intent; module=1579, inline_record=15472; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_interaction_intent
// 信息 ID: fc306f057599d8e07fb35ed3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::ui_systems::ui_move_decorator::ui_move_decorator @ 0x862c92 -> 0x14d7090 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::ui_place_decorator::ui_place_decorator @ 0x867f8d -> 0x14d7090 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_interaction_intent::closure$0
// 信息 ID: 468868529d777ca99d67fee9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_interaction_intent", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_interaction_intent; module=1579, inline_record=21948; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_interaction_intent::closure$1
// 信息 ID: 30b0d8dceb5ec0d7ded294a9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_interaction_intent", "captures": "unknown"}
// [闭包环境类型引用] system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_interaction_intent::closure_env$1 <- system_decorator::merge_proposal_roof_decorators::intersect_roof_decorators<system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_interaction_intent::closure_env$1>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_interaction_intent::closure_env$1 <- core::ops::function::impls::impl$3::call_mut<tuple$<tuple$<ref$<country_core::resources::walls::decorator::DecoratorId>,ref$<country_core::resources::walls::decorator::DecoratorInfo> > >,system_decorator::merge_proposal_roof_decorators::intersect_roof_decorators::closure_env$0<system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_interaction_intent::closure_env$1> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 2 条。
// [PDB 内联宿主] core::ops::function::impls::impl$3::call_mut<tuple$<tuple$<ref$<country_core::resources::walls::decorator::DecoratorId>,ref$<country_core::resources::walls::decorator::DecoratorInfo> > >,system_decorator::merge_proposal_roof_decorators::intersect_roof_decorators::closure_env$0<system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_interaction_intent::closure_env$1> >; module=3184, inline_record=6140; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_interaction_intent::closure$2
// 信息 ID: a72516c128dce31085c6cfce
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_interaction_intent", "captures": "unknown"}
// [闭包环境类型引用] system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_interaction_intent::closure_env$2 <- core::option::Option::and_then<ref$<country_core::resources::walls::decorator_storage::Decorators>,system_decorator::merge_proposal_roof_decorators::RoofDecoratorMergeProposal,system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_interaction_intent::closure_env$2>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] core::option::Option::and_then<ref$<country_core::resources::walls::decorator_storage::Decorators>,system_decorator::merge_proposal_roof_decorators::RoofDecoratorMergeProposal,system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_interaction_intent::closure_env$2>; module=1579, inline_record=27688; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_interaction_intent::closure$3
// 信息 ID: f5b73dea907afd11a0f878f3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 3, "lexical_owner": "system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_interaction_intent", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_interaction_intent; module=1579, inline_record=15972; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_interaction_intent::closure$4
// 信息 ID: 43120ccdf5bc2ecc16cfb539
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 4, "lexical_owner": "system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_interaction_intent", "captures": "unknown"}
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_interaction_intent @ 0x14d919a -> 0x2dd06d0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_wall_interaction_intent
// 信息 ID: fea0a84c7ebc033ca3439fde
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_interaction_intent; module=1579, inline_record=12536; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_wall_interaction_intent::closure$0
// 信息 ID: c9b4c834f4ec440044a1bea8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_wall_interaction_intent", "captures": "unknown"}
// [闭包环境类型引用] system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_wall_interaction_intent::closure_env$0 <- system_decorator::collision_detection::intersect_wall_decorators<system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_wall_interaction_intent::closure_env$0>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_wall_interaction_intent::closure_env$0 <- core::ops::function::impls::impl$3::call_mut<tuple$<tuple$<ref$<country_core::resources::walls::decorator::DecoratorId>,ref$<country_core::resources::walls::decorator::DecoratorInfo> > >,system_decorator::collision_detection::intersect_wall_decorators_impl::closure_env$0<utils::comparative_space::ZeroBoundedComparativeSpace,system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_wall_interaction_intent::closure_env$0> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_wall_interaction_intent::closure_env$0 <- core::ops::function::impls::impl$3::call_mut<tuple$<tuple$<ref$<country_core::resources::walls::decorator::DecoratorId>,ref$<country_core::resources::walls::decorator::DecoratorInfo> > >,system_decorator::collision_detection::intersect_wall_decorators_impl::closure_env$0<utils::comparative_space::CircularComparativeSpace,system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_wall_interaction_intent::closure_env$0> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 3 条。
// [PDB 内联宿主] core::ops::function::impls::impl$3::call_mut<tuple$<tuple$<ref$<country_core::resources::walls::decorator::DecoratorId>,ref$<country_core::resources::walls::decorator::DecoratorInfo> > >,system_decorator::collision_detection::intersect_wall_decorators_impl::closure_env$0<utils::comparative_space::ZeroBoundedComparativeSpace,system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_wall_interaction_intent::closure_env$0> >; module=3185, inline_record=9084; 不是独立调用边。
// [PDB 内联宿主] core::ops::function::impls::impl$3::call_mut<tuple$<tuple$<ref$<country_core::resources::walls::decorator::DecoratorId>,ref$<country_core::resources::walls::decorator::DecoratorInfo> > >,system_decorator::collision_detection::intersect_wall_decorators_impl::closure_env$0<utils::comparative_space::CircularComparativeSpace,system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_wall_interaction_intent::closure_env$0> >; module=3185, inline_record=11732; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_wall_interaction_intent::closure$1
// 信息 ID: 71d3565702a72d08061c6858
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_wall_interaction_intent", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_interaction_intent; module=1579, inline_record=13204; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_wall_interaction_intent::closure$2
// 信息 ID: 13966d129a316c8f4376c1c8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_wall_interaction_intent", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_interaction_intent; module=1579, inline_record=15532; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::ui_systems::decorator_interaction_intent::find_closest_norm_curve_t
// 信息 ID: c3035d5bad55faf1c385144e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_interaction_intent; module=1579, inline_record=16060; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::ui_systems::decorator_interaction_intent::find_closest_norm_curve_t::closure$0
// 信息 ID: fab3bad5cdefb21a648be36e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_decorator::ui_systems::decorator_interaction_intent::find_closest_norm_curve_t", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_interaction_intent; module=1579, inline_record=16168; 不是独立调用边。
// [PDB 内联宿主] system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_interaction_intent; module=1579, inline_record=17172; 不是独立调用边。
// [PDB 内联宿主] system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_interaction_intent; module=1579, inline_record=18224; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::ui_systems::decorator_interaction_intent::find_closest_norm_curve_t::closure$1
// 信息 ID: ba4e1dc8d236a5d9ad8d68fd
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "system_decorator::ui_systems::decorator_interaction_intent::find_closest_norm_curve_t", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_interaction_intent; module=1579, inline_record=18172; 不是独立调用边。
// [PDB 内联宿主] system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_interaction_intent; module=1579, inline_record=21680; 不是独立调用边。
// [PDB 内联宿主] system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_interaction_intent; module=1579, inline_record=21712; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::ui_systems::decorator_interaction_intent::find_closest_norm_curve_t::closure$2
// 信息 ID: 0ac9beefe0567f9ae8339858
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "system_decorator::ui_systems::decorator_interaction_intent::find_closest_norm_curve_t", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_interaction_intent; module=1579, inline_record=18092; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::ui_systems::decorator_interaction_intent::find_closest_norm_curve_t::closure$3
// 信息 ID: 45877274b7c299653b0d90ec
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 3, "lexical_owner": "system_decorator::ui_systems::decorator_interaction_intent::find_closest_norm_curve_t", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_interaction_intent; module=1579, inline_record=21368; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::ui_systems::decorator_interaction_intent::snap_balcony_door
// 信息 ID: df7cebef51c3128b11fbd4e0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_interaction_intent @ 0x14d8d6c -> 0x14d62c0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
