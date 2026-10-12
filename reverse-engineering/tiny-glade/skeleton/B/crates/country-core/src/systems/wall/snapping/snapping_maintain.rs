// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/country-core/src/systems/wall/snapping/snapping_maintain.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: 26225cc9bcacff0957b658bc62c4e8090259ed14a1d2cd676f755ed3398b7a4d
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: snapping_maintain.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 551
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
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.3"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.5"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.15.2"}]
// use nonmax; // namespace candidate
//   package identities: [{"name":"nonmax","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.5.5"}]
// use std; // namespace candidate
//   package identities: []
// use utils; // namespace candidate
//   package identities: [{"name":"utils","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::snapping::snapping_maintain::check_for_disable_snapping
// 记录形式: compiled_procedure; instances=1
// 源码行: 19
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":551,"record_offset":35324,"rva":10921312,"va":5379630432}
// 编译实例 RVAs: 0xa6a560
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-5275532","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379630432,"raw_hex":"e87480afff","size":5,"soft_fail":false,"target_va":5374354992,"va":5379630519,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":551,"record_offset":35820,"parent_record_offset":35324,"ancestor_record_offset":35324,"inlinee":4457,"source_line":434,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::writer::EventWriter::write
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/writer.rs
//     evidence={"module":551,"record_offset":35496,"parent_record_offset":35324,"ancestor_record_offset":35324,"inlinee":7657,"source_line":74,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":551,"record_offset":35688,"parent_record_offset":35656,"ancestor_record_offset":35324,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":551,"record_offset":35736,"parent_record_offset":35712,"ancestor_record_offset":35324,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":551,"record_offset":35712,"parent_record_offset":35688,"ancestor_record_offset":35324,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":551,"record_offset":35656,"parent_record_offset":35616,"ancestor_record_offset":35324,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":551,"record_offset":35616,"parent_record_offset":35580,"ancestor_record_offset":35324,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":551,"record_offset":35552,"parent_record_offset":35520,"ancestor_record_offset":35324,"inlinee":4459,"source_line":397,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":551,"record_offset":35520,"parent_record_offset":35496,"ancestor_record_offset":35324,"inlinee":4457,"source_line":434,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::collections::Events::send_with_caller
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/collections.rs
//     evidence={"module":551,"record_offset":35580,"parent_record_offset":35520,"ancestor_record_offset":35324,"inlinee":7659,"source_line":128,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":551,"record_offset":35776,"parent_record_offset":35616,"ancestor_record_offset":35324,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::snapping::snapping_maintain::snapping_signifiers
// 记录形式: compiled_procedure; instances=1
// 源码行: 40
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":551,"record_offset":36208,"rva":10921552,"va":5379630672}
// 编译实例 RVAs: 0xa6a650
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1405b0d70
//     owner=unresolved_namespace; count=1
//     target_aliases=7 candidates; complete list: snapping_maintain.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4955533","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379630672,"raw_hex":"e87362b4ff","size":5,"soft_fail":false,"target_va":5374676336,"va":5379631864,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c9cde0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: snapping_maintain.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2302777","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379630672,"raw_hex":"e839232300","size":5,"soft_fail":false,"target_va":5381934560,"va":5379631778,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t31909695","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379630672,"raw_hex":"e83fe7e601","size":5,"soft_fail":false,"target_va":5411542064,"va":5379632364,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t31911190","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379630672,"raw_hex":"e816ede601","size":5,"soft_fail":false,"target_va":5411542880,"va":5379631685,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::wall::shapes::shape_placement_signifiers::preview_snapping_line
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-1617832","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379630672,"raw_hex":"e85850e7ff","size":5,"soft_fail":false,"target_va":5378014304,"va":5379632131,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::utils::debug_viz::DebugLine::show
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t631525","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379630672,"raw_hex":"e8e5a20900","size":5,"soft_fail":false,"target_va":5380263792,"va":5379632262,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::utils::debug_viz::DebugSphere::show
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t620636","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379630672,"raw_hex":"e85c780900","size":5,"soft_fail":false,"target_va":5380252960,"va":5379632319,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$58::deref_mut
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":551,"record_offset":39640,"parent_record_offset":36208,"ancestor_record_offset":36208,"inlinee":9177,"source_line":434,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::query::Query::single_mut
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":551,"record_offset":36392,"parent_record_offset":36208,"ancestor_record_offset":36208,"inlinee":31549,"source_line":2032,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":551,"record_offset":39612,"parent_record_offset":36208,"ancestor_record_offset":36208,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::public_walls::PublicWalls::get
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_walls.rs
//     evidence={"module":551,"record_offset":43120,"parent_record_offset":36208,"ancestor_record_offset":36208,"inlinee":26470,"source_line":39,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_direct] country_core::utils::debug_viz::DebugLine::line_width
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/debug_viz.rs
//     evidence={"module":551,"record_offset":44228,"parent_record_offset":36208,"ancestor_record_offset":36208,"inlinee":30066,"source_line":407,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_direct] country_core::utils::debug_viz::DebugSphere::radius
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/debug_viz.rs
//     evidence={"module":551,"record_offset":44256,"parent_record_offset":36208,"ancestor_record_offset":36208,"inlinee":29721,"source_line":100,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":551,"record_offset":36696,"parent_record_offset":36672,"ancestor_record_offset":36208,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":551,"record_offset":36744,"parent_record_offset":36720,"ancestor_record_offset":36208,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":551,"record_offset":36720,"parent_record_offset":36696,"ancestor_record_offset":36208,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":551,"record_offset":36672,"parent_record_offset":36648,"ancestor_record_offset":36208,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":551,"record_offset":36648,"parent_record_offset":36624,"ancestor_record_offset":36208,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":551,"record_offset":37164,"parent_record_offset":37140,"ancestor_record_offset":36208,"inlinee":4613,"source_line":2705,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":551,"record_offset":37188,"parent_record_offset":37164,"ancestor_record_offset":36208,"inlinee":4352,"source_line":2680,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":551,"record_offset":36624,"parent_record_offset":36600,"ancestor_record_offset":36208,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::entities
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":551,"record_offset":38584,"parent_record_offset":36828,"ancestor_record_offset":36208,"inlinee":11682,"source_line":466,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::is_empty
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":551,"record_offset":37960,"parent_record_offset":36828,"ancestor_record_offset":36208,"inlinee":11679,"source_line":615,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::table_id
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":551,"record_offset":38044,"parent_record_offset":36828,"ancestor_record_offset":36208,"inlinee":5495,"source_line":460,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::ArchetypeEntity::table_row
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":551,"record_offset":38780,"parent_record_offset":36828,"ancestor_record_offset":36208,"inlinee":17012,"source_line":325,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetypes::get
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":551,"record_offset":37876,"parent_record_offset":36828,"ancestor_record_offset":36208,"inlinee":11678,"source_line":895,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$57::set_changed
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":551,"record_offset":39664,"parent_record_offset":39640,"ancestor_record_offset":36208,"inlinee":9179,"source_line":397,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::StorageSwitch::extract
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":551,"record_offset":38856,"parent_record_offset":38832,"ancestor_record_offset":36208,"inlinee":4876,"source_line":2472,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::set_archetype
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":551,"record_offset":38164,"parent_record_offset":38136,"ancestor_record_offset":36208,"inlinee":13655,"source_line":1524,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::set_table
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":551,"record_offset":37248,"parent_record_offset":37224,"ancestor_record_offset":36208,"inlinee":13656,"source_line":1539,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$37::fetch
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":551,"record_offset":38832,"parent_record_offset":38808,"ancestor_record_offset":36208,"inlinee":13686,"source_line":1596,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$37::fetch::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":551,"record_offset":38880,"parent_record_offset":38856,"ancestor_record_offset":36208,"inlinee":9533,"source_line":1602,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$61::fetch
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":551,"record_offset":38808,"parent_record_offset":36828,"ancestor_record_offset":36208,"inlinee":17019,"source_line":2083,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIter::new
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs
//     evidence={"module":551,"record_offset":36576,"parent_record_offset":36552,"ancestor_record_offset":36208,"inlinee":13709,"source_line":39,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::init
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs
//     evidence={"module":551,"record_offset":36600,"parent_record_offset":36576,"ancestor_record_offset":36208,"inlinee":9292,"source_line":2359,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::next
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs
//     evidence={"module":551,"record_offset":36828,"parent_record_offset":36804,"ancestor_record_offset":36208,"inlinee":11674,"source_line":2458,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::impl$1::next
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs
//     evidence={"module":551,"record_offset":36804,"parent_record_offset":36504,"ancestor_record_offset":36208,"inlinee":11673,"source_line":884,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$2::set_archetype
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/world_query.rs
//     evidence={"module":551,"record_offset":38136,"parent_record_offset":36828,"ancestor_record_offset":36208,"inlinee":13725,"source_line":182,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$2::set_table
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/world_query.rs
//     evidence={"module":551,"record_offset":37224,"parent_record_offset":36828,"ancestor_record_offset":36208,"inlinee":17013,"source_line":195,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_array::BlobArray::get_sub_slice
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_array.rs
//     evidence={"module":551,"record_offset":37628,"parent_record_offset":37604,"ancestor_record_offset":36208,"inlinee":4872,"source_line":143,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseArray::get
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":551,"record_offset":37336,"parent_record_offset":37308,"ancestor_record_offset":36208,"inlinee":4864,"source_line":58,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":551,"record_offset":37308,"parent_record_offset":37280,"ancestor_record_offset":36208,"inlinee":4862,"source_line":395,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$16::get::closure$0
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":551,"record_offset":37472,"parent_record_offset":37448,"ancestor_record_offset":36208,"inlinee":4866,"source_line":396,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_column
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":551,"record_offset":37280,"parent_record_offset":37248,"ancestor_record_offset":36208,"inlinee":4860,"source_line":475,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::is_empty
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":551,"record_offset":37140,"parent_record_offset":36828,"ancestor_record_offset":36208,"inlinee":11675,"source_line":615,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":551,"record_offset":37056,"parent_record_offset":36828,"ancestor_record_offset":36208,"inlinee":4854,"source_line":721,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::column::ThinColumn::get_changed_ticks_slice
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/column.rs
//     evidence={"module":551,"record_offset":37660,"parent_record_offset":37248,"ancestor_record_offset":36208,"inlinee":13683,"source_line":319,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::column::ThinColumn::get_data_slice
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/column.rs
//     evidence={"module":551,"record_offset":37604,"parent_record_offset":37248,"ancestor_record_offset":36208,"inlinee":4870,"source_line":303,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::thin_array_ptr::ThinArrayPtr::as_slice
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/thin_array_ptr.rs
//     evidence={"module":551,"record_offset":37684,"parent_record_offset":37660,"ancestor_record_offset":36208,"inlinee":13684,"source_line":300,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::copy_unsafe
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":551,"record_offset":36468,"parent_record_offset":36444,"ancestor_record_offset":36208,"inlinee":13707,"source_line":636,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::reborrow
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":551,"record_offset":36420,"parent_record_offset":36392,"ancestor_record_offset":36208,"inlinee":17021,"source_line":601,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::reborrow_unsafe
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":551,"record_offset":36444,"parent_record_offset":36420,"ancestor_record_offset":36208,"inlinee":13706,"source_line":618,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::single_inner
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":551,"record_offset":36504,"parent_record_offset":36392,"ancestor_record_offset":36208,"inlinee":31550,"source_line":2070,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::impl$4::into_iter
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":551,"record_offset":36552,"parent_record_offset":36504,"ancestor_record_offset":36208,"inlinee":25723,"source_line":2574,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ptr::ThinSlicePtr::get
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ptr","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ptr-0.16.0/src/lib.rs
//     evidence={"module":551,"record_offset":38908,"parent_record_offset":38880,"ancestor_record_offset":36208,"inlinee":4880,"source_line":526,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":551,"record_offset":43952,"parent_record_offset":43928,"ancestor_record_offset":36208,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":551,"record_offset":43460,"parent_record_offset":43436,"ancestor_record_offset":36208,"inlinee":4484,"source_line":1227,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":551,"record_offset":43544,"parent_record_offset":43520,"ancestor_record_offset":36208,"inlinee":4486,"source_line":1433,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":551,"record_offset":43484,"parent_record_offset":43460,"ancestor_record_offset":36208,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":551,"record_offset":44020,"parent_record_offset":43324,"ancestor_record_offset":36208,"inlinee":4502,"source_line":1497,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":551,"record_offset":43692,"parent_record_offset":43664,"ancestor_record_offset":36208,"inlinee":4503,"source_line":449,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":551,"record_offset":43664,"parent_record_offset":43636,"ancestor_record_offset":36208,"inlinee":4493,"source_line":601,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":551,"record_offset":37384,"parent_record_offset":37360,"ancestor_record_offset":36208,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":551,"record_offset":37360,"parent_record_offset":37336,"ancestor_record_offset":36208,"inlinee":4518,"source_line":1447,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":551,"record_offset":37408,"parent_record_offset":37384,"ancestor_record_offset":36208,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":551,"record_offset":37448,"parent_record_offset":37308,"ancestor_record_offset":36208,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":551,"record_offset":38932,"parent_record_offset":38908,"ancestor_record_offset":36208,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":551,"record_offset":43812,"parent_record_offset":43788,"ancestor_record_offset":36208,"inlinee":4723,"source_line":1129,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":551,"record_offset":37016,"parent_record_offset":36992,"ancestor_record_offset":36208,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":551,"record_offset":36940,"parent_record_offset":36900,"ancestor_record_offset":36208,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":551,"record_offset":37080,"parent_record_offset":37056,"ancestor_record_offset":36208,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_noubcheck
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":551,"record_offset":37552,"parent_record_offset":37528,"ancestor_record_offset":36208,"inlinee":4164,"source_line":87,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":551,"record_offset":37104,"parent_record_offset":37080,"ancestor_record_offset":36208,"inlinee":4145,"source_line":226,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":551,"record_offset":37528,"parent_record_offset":37496,"ancestor_record_offset":36208,"inlinee":4162,"source_line":242,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":551,"record_offset":36992,"parent_record_offset":36968,"ancestor_record_offset":36208,"inlinee":4510,"source_line":99,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":551,"record_offset":36900,"parent_record_offset":36828,"ancestor_record_offset":36208,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$186::next_unchecked
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":551,"record_offset":36968,"parent_record_offset":36900,"ancestor_record_offset":36208,"inlinee":4509,"source_line":440,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::impl$9::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":551,"record_offset":43976,"parent_record_offset":43952,"ancestor_record_offset":36208,"inlinee":4919,"source_line":69,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":551,"record_offset":43604,"parent_record_offset":43576,"ancestor_record_offset":36208,"inlinee":4490,"source_line":49,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":551,"record_offset":43636,"parent_record_offset":43604,"ancestor_record_offset":36208,"inlinee":4491,"source_line":74,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":551,"record_offset":43576,"parent_record_offset":43324,"ancestor_record_offset":36208,"inlinee":4488,"source_line":112,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":551,"record_offset":43436,"parent_record_offset":43324,"ancestor_record_offset":36208,"inlinee":4482,"source_line":51,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":551,"record_offset":44048,"parent_record_offset":43324,"ancestor_record_offset":36208,"inlinee":4504,"source_line":94,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":551,"record_offset":43520,"parent_record_offset":43324,"ancestor_record_offset":36208,"inlinee":4485,"source_line":76,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\tag.rs
//     evidence={"module":551,"record_offset":43408,"parent_record_offset":43324,"ancestor_record_offset":36208,"inlinee":4517,"source_line":36,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\lib.rs
//     evidence={"module":551,"record_offset":43928,"parent_record_offset":43904,"ancestor_record_offset":36208,"inlinee":4499,"source_line":164,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":551,"record_offset":43188,"parent_record_offset":43160,"ancestor_record_offset":36208,"inlinee":4719,"source_line":1304,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":551,"record_offset":43216,"parent_record_offset":43188,"ancestor_record_offset":36208,"inlinee":4475,"source_line":1347,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":551,"record_offset":43904,"parent_record_offset":43736,"ancestor_record_offset":36208,"inlinee":4497,"source_line":224,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":551,"record_offset":43248,"parent_record_offset":43216,"ancestor_record_offset":36208,"inlinee":4514,"source_line":253,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":551,"record_offset":43844,"parent_record_offset":43760,"ancestor_record_offset":36208,"inlinee":4722,"source_line":384,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":551,"record_offset":43788,"parent_record_offset":43760,"ancestor_record_offset":36208,"inlinee":5616,"source_line":261,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":551,"record_offset":44140,"parent_record_offset":43324,"ancestor_record_offset":36208,"inlinee":4506,"source_line":81,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":551,"record_offset":43760,"parent_record_offset":43736,"ancestor_record_offset":36208,"inlinee":5615,"source_line":734,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":551,"record_offset":43300,"parent_record_offset":43276,"ancestor_record_offset":36208,"inlinee":4478,"source_line":1156,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":551,"record_offset":43276,"parent_record_offset":43216,"ancestor_record_offset":36208,"inlinee":4477,"source_line":1179,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":551,"record_offset":43324,"parent_record_offset":43300,"ancestor_record_offset":36208,"inlinee":4480,"source_line":1862,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":551,"record_offset":43380,"parent_record_offset":43324,"ancestor_record_offset":36208,"inlinee":4515,"source_line":2301,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":551,"record_offset":43736,"parent_record_offset":43324,"ancestor_record_offset":36208,"inlinee":4495,"source_line":1166,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"nonmax","version":"0.5.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":551,"record_offset":37496,"parent_record_offset":37472,"ancestor_record_offset":36208,"inlinee":4171,"source_line":153,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":551,"record_offset":43160,"parent_record_offset":43120,"ancestor_record_offset":36208,"inlinee":4718,"source_line":894,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::is_valid
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":551,"record_offset":44196,"parent_record_offset":43120,"ancestor_record_offset":36208,"inlinee":26471,"source_line":165,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::snapping::snapping_maintain::snapping_maintain
// 记录形式: compiled_procedure; instances=1
// 源码行: 93
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":551,"record_offset":44288,"rva":10923264,"va":5379632384}
// 编译实例 RVAs: 0xa6ad00
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":551,"record_offset":44504,"parent_record_offset":44288,"ancestor_record_offset":44288,"inlinee":4457,"source_line":434,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::wall::snapping::impl$3::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/snapping/mod.rs
//     evidence={"module":551,"record_offset":44452,"parent_record_offset":44288,"ancestor_record_offset":44288,"inlinee":31552,"source_line":30,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":551,"record_offset":44528,"parent_record_offset":44504,"ancestor_record_offset":44288,"inlinee":4459,"source_line":397,"archive":"_pdb/B/modules/00551.jsonl.gz"}
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
// 完整结构及原始类型记录: snapping_maintain.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: snapping_maintain.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: snapping_maintain.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 3; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::wall::snapping::SnappingMode>,bevy_ecs::change_detection::Res<country_core::systems::mod_key::Modifiers>,bevy_ecs::event::writer::EventWriter<country_core::resources::onboarding::NotifyHintSystemSnapping>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::systems::wall::snapping::SnappingMode>", "bevy_ecs::change_detection::Res<country_core::systems::mod_key::Modifiers>", "bevy_ecs::event::writer::EventWriter<country_core::resources::onboarding::NotifyHintSystemSnapping>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::wall::snapping::SnappingMode>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::mod_key::Modifiers>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::resources::onboarding::NotifyHintSystemSnapping>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::systems::wall::snapping::SnappingMode>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::components::SnappingToSegmentPreview>,ref_mut$<utils::components::transform::Transform> >,bevy_ecs::query::filter::Without<country_core::components::SnappingToEndPreview> >,bevy_ecs::system::query::Query<tuple$<ref$<country_core::components::SnappingToEndPreview>,ref_mut$<utils::components::transform::Transform> >,bevy_ecs::query::filter::Without<country_core::components::SnappingToSegmentPreview> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::systems::wall::snapping::SnappingMode>", "bevy_ecs::system::query::Query<tuple$<ref$<country_core::components::SnappingToSegmentPreview>,ref_mut$<utils::components::transform::Transform> >,bevy_ecs::query::filter::Without<country_core::components::SnappingToEndPreview> >", "bevy_ecs::system::query::Query<tuple$<ref$<country_core::components::SnappingToEndPreview>,ref_mut$<utils::components::transform::Transform> >,bevy_ecs::query::filter::Without<country_core::components::SnappingToSegmentPreview> >", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::wall::snapping::SnappingMode>
// [候选系统接口] query_without_filter: bevy_ecs::query::filter::Without<country_core::components::SnappingToEndPreview>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<ref$<country_core::components::SnappingToSegmentPreview>,ref_mut$<utils::components::transform::Transform> >,bevy_ecs::query::filter::Without<country_core::components::SnappingToEndPreview> >
// [候选系统接口] query_without_filter: bevy_ecs::query::filter::Without<country_core::components::SnappingToSegmentPreview>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<ref$<country_core::components::SnappingToEndPreview>,ref_mut$<utils::components::transform::Transform> >,bevy_ecs::query::filter::Without<country_core::components::SnappingToSegmentPreview> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::wall::snapping::SnappingMode>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::systems::wall::snapping::SnappingMode>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::wall::snapping::SnappingMode>

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::snapping::snapping_maintain::check_for_disable_snapping
// 信息 ID: 524541a76808d901f420ad86
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 9b849602d0b9c4175f827fa5: void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::wall::snapping::SnappingMode>,bevy_ecs::change_detection::Res<country_core::systems::mod_key::Modifiers>,bevy_ecs::event::writer::EventWriter<country_core::resources::onboarding::NotifyHintSystemSnapping>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::schedule_system::impl$1::run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::wall::snapping::SnappingMode>,bevy_ecs::change_detection::Res<country_core::systems::mod_key::Modifiers>,bevy_ecs::event::writer::EventWriter<country_core::resources::onboarding::NotifyHintSystemSnapping>),void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::wall::snapping::SnappingMode>,bevy_ecs::change_detection::Res<country_core::systems::mod_key::Modifiers>,bevy_ecs::event::writer::EventWriter<country_core::resources::onboarding::NotifyHintSystemSnapping>)> > @ 0xeada9 -> 0xa6a560 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::wall::snapping::SnappingMode>,bevy_ecs::change_detection::Res<country_core::systems::mod_key::Modifiers>,bevy_ecs::event::writer::EventWriter<country_core::resources::onboarding::NotifyHintSystemSnapping>),void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::wall::snapping::SnappingMode>,bevy_ecs::change_detection::Res<country_core::systems::mod_key::Modifiers>,bevy_ecs::event::writer::EventWriter<country_core::resources::onboarding::NotifyHintSystemSnapping>)> > > @ 0x453c0b -> 0xa6a560 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::snapping::snapping_maintain::snapping_maintain
// 信息 ID: 3ed93ac4005499c896a82476
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 0dcc397b87b2a358b39e9cc0: void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::wall::snapping::SnappingMode>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::schedule_system::impl$1::run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::wall::snapping::SnappingMode>),void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::wall::snapping::SnappingMode>)> > @ 0xeab50 -> 0xa6ad00 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::wall::snapping::SnappingMode>),void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::wall::snapping::SnappingMode>)> > > @ 0x46261e -> 0xa6ad00 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::snapping::snapping_maintain::snapping_signifiers
// 信息 ID: f3e88db5f780c11998285c46
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] c6a58549ba308228e181fe6a: void (*)(bevy_ecs::change_detection::Res<country_core::systems::wall::snapping::SnappingMode>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::components::SnappingToSegmentPreview>,ref_mut$<utils::components::transform::Transform> >,bevy_ecs::query::filter::Without<country_core::components::SnappingToEndPreview> >,bevy_ecs::system::query::Query<tuple$<ref$<country_core::components::SnappingToEndPreview>,ref_mut$<utils::components::transform::Transform> >,bevy_ecs::query::filter::Without<country_core::components::SnappingToSegmentPreview> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::schedule_system::impl$1::run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::systems::wall::snapping::SnappingMode>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::components::SnappingToSegmentPreview>,ref_mut$<utils::components::transform::Transform> >,bevy_ecs::query::filter::Without<country_core::components::SnappingToEndPreview> >,bevy_ecs::system::query::Query<tuple$<ref$<country_core::components::SnappingToEndPreview>,ref_mut$<utils::components::transform::Transform> >,bevy_ecs::query::filter::Without<country_core::components::SnappingToSegmentPreview> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>),void (*)(bevy_ecs::change_detection::Res<country_core::systems::wall::snapping::SnappingMode>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::components::SnappingToSegmentPreview>,ref_mut$<utils::components::transform::Transform> >,bevy_ecs::query::filter::Without<country_core::components::SnappingToEndPreview> >,bevy_ecs::system::query::Query<tuple$<ref$<country_core::components::SnappingToEndPreview>,ref_mut$<utils::components::transform::Transform> >,bevy_ecs::query::filter::Without<country_core::components::SnappingToSegmentPreview> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)> > @ 0xea9a2 -> 0xa6a650 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::systems::wall::snapping::SnappingMode>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::components::SnappingToSegmentPreview>,ref_mut$<utils::components::transform::Transform> >,bevy_ecs::query::filter::Without<country_core::components::SnappingToEndPreview> >,bevy_ecs::system::query::Query<tuple$<ref$<country_core::components::SnappingToEndPreview>,ref_mut$<utils::components::transform::Transform> >,bevy_ecs::query::filter::Without<country_core::components::SnappingToSegmentPreview> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>),void (*)(bevy_ecs::change_detection::Res<country_core::systems::wall::snapping::SnappingMode>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::components::SnappingToSegmentPreview>,ref_mut$<utils::components::transform::Transform> >,bevy_ecs::query::filter::Without<country_core::components::SnappingToEndPreview> >,bevy_ecs::system::query::Query<tuple$<ref$<country_core::components::SnappingToEndPreview>,ref_mut$<utils::components::transform::Transform> >,bevy_ecs::query::filter::Without<country_core::components::SnappingToSegmentPreview> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)> > > @ 0x46b79a -> 0xa6a650 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
