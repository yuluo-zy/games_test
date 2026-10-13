// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/country-core/src/systems/render_loop/draw_passes/draw_shaders/stairs_draw.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: 6f43a28bb8491e195e107801ea16ab174980f5b6c9a6354b3ee9f80abe61f391
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: stairs_draw.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 482
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use bevy_ecs; // namespace candidate
//   package identities: [{"name":"bevy_ecs","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.0"}]
// use bevy_platform; // namespace candidate
//   package identities: [{"name":"bevy_platform","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.0"}]
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use equivalent; // namespace candidate
//   package identities: [{"name":"equivalent","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.1"}]
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.3"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.5"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.15.2"}]
// use std; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::draw_passes::draw_shaders::stairs_draw::stairs_draw
// 记录形式: compiled_procedure; instances=1
// 源码行: 15
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":482,"record_offset":39936,"rva":8824704,"va":5377533824}
// 编译实例 RVAs: 0x86a780
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$bevy_ecs..entity..Entity$u20$as$u20$bevy_ecs..world..entity_fetch..WorldEntityFetch$GT$::fetch_ref
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4722891","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377533824,"raw_hex":"e8cb104800","size":5,"soft_fail":false,"target_va":5382256784,"va":5377533888,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408935c0
//     owner=unresolved_namespace; count=5
//     target_aliases=2 candidates; complete list: stairs_draw.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t166258","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377533824,"raw_hex":"e872890200","size":5,"soft_fail":false,"target_va":5377701312,"va":5377535049,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140908610
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: stairs_draw.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t646042","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377533824,"raw_hex":"e89adb0900","size":5,"soft_fail":false,"target_va":5378180624,"va":5377534577,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140908c50
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: stairs_draw.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t647432","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377533824,"raw_hex":"e808e10900","size":5,"soft_fail":false,"target_va":5378182224,"va":5377534787,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14096b570
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: stairs_draw.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1051315","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377533824,"raw_hex":"e8b30a1000","size":5,"soft_fail":false,"target_va":5378585968,"va":5377534648,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14096b850
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: stairs_draw.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1051980","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377533824,"raw_hex":"e84c0d1000","size":5,"soft_fail":false,"target_va":5378586704,"va":5377534719,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14162cb90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: stairs_draw.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t14426455","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377533824,"raw_hex":"e85721dc00","size":5,"soft_fail":false,"target_va":5391960976,"va":5377534516,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=5
//     target_aliases=1 candidates; complete list: stairs_draw.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t34312676","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377533824,"raw_hex":"e8e4910b02","size":5,"soft_fail":false,"target_va":5411847168,"va":5377534487,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::storage::table::Table::get_component
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t4809068","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377533824,"raw_hex":"e86c614900","size":5,"soft_fail":false,"target_va":5382343504,"va":5377534431,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t34006812","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377533824,"raw_hex":"e81ce70602","size":5,"soft_fail":false,"target_va":5411541776,"va":5377534959,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::draw_builder::DrawBuilder::multi_draw_indexed_indirect
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t642580","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377533824,"raw_hex":"e814ce0900","size":5,"soft_fail":false,"target_va":5378177424,"va":5377534839,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::sync::rwlock::futex::RwLock::read_contended
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t34004623","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377533824,"raw_hex":"e88fde0602","size":5,"soft_fail":false,"target_va":5411539504,"va":5377534876,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::sync::rwlock::futex::RwLock::wake_writer_or_readers
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t34005376","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377533824,"raw_hex":"e880e10602","size":5,"soft_fail":false,"target_va":5411540288,"va":5377534907,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::world::World::get_entity
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/mod.rs
//     evidence={"module":482,"record_offset":40128,"parent_record_offset":39936,"ancestor_record_offset":39936,"inlinee":26777,"source_line":910,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":482,"record_offset":40156,"parent_record_offset":39936,"ancestor_record_offset":39936,"inlinee":5236,"source_line":771,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap_or_default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":482,"record_offset":43260,"parent_record_offset":39936,"ancestor_record_offset":39936,"inlinee":14043,"source_line":1140,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::draw_builder::DrawBuilder::draw_indexed_indirect
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_builder.rs
//     evidence={"module":482,"record_offset":43520,"parent_record_offset":39936,"ancestor_record_offset":39936,"inlinee":26793,"source_line":384,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::draw_builder::impl$2::read_texture
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_builder.rs
//     evidence={"module":482,"record_offset":43288,"parent_record_offset":39936,"ancestor_record_offset":39936,"inlinee":8283,"source_line":524,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::draw_builder::impl$2::uniform
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_builder.rs
//     evidence={"module":482,"record_offset":43332,"parent_record_offset":39936,"ancestor_record_offset":39936,"inlinee":26792,"source_line":502,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::Components::get_id
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/component.rs
//     evidence={"module":482,"record_offset":40320,"parent_record_offset":40276,"ancestor_record_offset":39936,"inlinee":26784,"source_line":2427,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::impl$19::get_id::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/component.rs
//     evidence={"module":482,"record_offset":41308,"parent_record_offset":41272,"ancestor_record_offset":39936,"inlinee":26786,"source_line":2428,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::impl$19::get_id::closure$0::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/component.rs
//     evidence={"module":482,"record_offset":42644,"parent_record_offset":42620,"ancestor_record_offset":39936,"inlinee":26788,"source_line":2434,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":482,"record_offset":43088,"parent_record_offset":43064,"ancestor_record_offset":39936,"inlinee":4854,"source_line":721,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::entity_ref::EntityRef::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/entity_ref.rs
//     evidence={"module":482,"record_offset":40236,"parent_record_offset":40204,"ancestor_record_offset":39936,"inlinee":26781,"source_line":133,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeEntityCell::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":482,"record_offset":40276,"parent_record_offset":40236,"ancestor_record_offset":39936,"inlinee":26783,"source_line":785,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::fetch_table
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":482,"record_offset":43064,"parent_record_offset":43036,"ancestor_record_offset":39936,"inlinee":26791,"source_line":1161,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::get_component
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":482,"record_offset":43036,"parent_record_offset":40276,"ancestor_record_offset":39936,"inlinee":26790,"source_line":1189,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] bevy_platform::collections::hash_map::HashMap::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_platform","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_platform-0.16.0/src/collections/hash_map.rs
//     evidence={"module":482,"record_offset":40360,"parent_record_offset":40320,"ancestor_record_offset":39936,"inlinee":5614,"source_line":933,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$7::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\any.rs
//     evidence={"module":482,"record_offset":41152,"parent_record_offset":41128,"ancestor_record_offset":39936,"inlinee":5068,"source_line":719,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$25::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":482,"record_offset":41200,"parent_record_offset":41176,"ancestor_record_offset":39936,"inlinee":5070,"source_line":1740,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":482,"record_offset":40600,"parent_record_offset":40576,"ancestor_record_offset":39936,"inlinee":4484,"source_line":1227,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":482,"record_offset":40684,"parent_record_offset":40660,"ancestor_record_offset":39936,"inlinee":4486,"source_line":1433,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":482,"record_offset":40624,"parent_record_offset":40600,"ancestor_record_offset":39936,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":482,"record_offset":40836,"parent_record_offset":40808,"ancestor_record_offset":39936,"inlinee":4503,"source_line":449,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":482,"record_offset":40808,"parent_record_offset":40780,"ancestor_record_offset":39936,"inlinee":4493,"source_line":601,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":482,"record_offset":43372,"parent_record_offset":43332,"ancestor_record_offset":39936,"inlinee":4130,"source_line":728,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":482,"record_offset":42620,"parent_record_offset":41308,"ancestor_record_offset":39936,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":482,"record_offset":41272,"parent_record_offset":40320,"ancestor_record_offset":39936,"inlinee":4571,"source_line":1546,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":482,"record_offset":43216,"parent_record_offset":40204,"ancestor_record_offset":39936,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$39::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":482,"record_offset":43008,"parent_record_offset":40276,"ancestor_record_offset":39936,"inlinee":4927,"source_line":2486,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":482,"record_offset":42676,"parent_record_offset":41308,"ancestor_record_offset":39936,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":482,"record_offset":41068,"parent_record_offset":41044,"ancestor_record_offset":39936,"inlinee":4723,"source_line":1129,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":482,"record_offset":43112,"parent_record_offset":43088,"ancestor_record_offset":39936,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_noubcheck
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":482,"record_offset":43160,"parent_record_offset":43136,"ancestor_record_offset":39936,"inlinee":4164,"source_line":87,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":482,"record_offset":43136,"parent_record_offset":43112,"ancestor_record_offset":39936,"inlinee":4145,"source_line":226,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":482,"record_offset":41648,"parent_record_offset":41624,"ancestor_record_offset":39936,"inlinee":5623,"source_line":623,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":482,"record_offset":41516,"parent_record_offset":41396,"ancestor_record_offset":39936,"inlinee":7020,"source_line":2920,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::fetch_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":482,"record_offset":42768,"parent_record_offset":42732,"ancestor_record_offset":39936,"inlinee":7030,"source_line":2986,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":482,"record_offset":41432,"parent_record_offset":41396,"ancestor_record_offset":39936,"inlinee":4181,"source_line":2698,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":482,"record_offset":41540,"parent_record_offset":41516,"ancestor_record_offset":39936,"inlinee":5825,"source_line":3828,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":482,"record_offset":41456,"parent_record_offset":41432,"ancestor_record_offset":39936,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":482,"record_offset":42792,"parent_record_offset":42768,"ancestor_record_offset":39936,"inlinee":4778,"source_line":3777,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::tuple::impl$10::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\tuple.rs
//     evidence={"module":482,"record_offset":41176,"parent_record_offset":41152,"ancestor_record_offset":39936,"inlinee":4662,"source_line":30,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::render_loop::draw_passes::draw_shaders::stairs_draw::stairs_draw::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/render_loop/draw_passes/draw_shaders/stairs_draw.rs
//     evidence={"module":482,"record_offset":40204,"parent_record_offset":40156,"ancestor_record_offset":39936,"inlinee":26779,"source_line":23,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] equivalent::impl$0::equivalent
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"equivalent","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/equivalent-1.0.1/src/lib.rs
//     evidence={"module":482,"record_offset":41128,"parent_record_offset":41104,"ancestor_record_offset":39936,"inlinee":4660,"source_line":88,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":482,"record_offset":40744,"parent_record_offset":40716,"ancestor_record_offset":39936,"inlinee":4490,"source_line":49,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":482,"record_offset":40780,"parent_record_offset":40744,"ancestor_record_offset":39936,"inlinee":4491,"source_line":74,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":482,"record_offset":40716,"parent_record_offset":40496,"ancestor_record_offset":39936,"inlinee":4488,"source_line":112,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":482,"record_offset":40576,"parent_record_offset":40496,"ancestor_record_offset":39936,"inlinee":4482,"source_line":51,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":482,"record_offset":40880,"parent_record_offset":40496,"ancestor_record_offset":39936,"inlinee":4504,"source_line":94,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":482,"record_offset":40660,"parent_record_offset":40496,"ancestor_record_offset":39936,"inlinee":4485,"source_line":76,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/map.rs
//     evidence={"module":482,"record_offset":40396,"parent_record_offset":40360,"ancestor_record_offset":39936,"inlinee":4719,"source_line":1304,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":482,"record_offset":40420,"parent_record_offset":40396,"ancestor_record_offset":39936,"inlinee":4475,"source_line":1347,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":482,"record_offset":41104,"parent_record_offset":40996,"ancestor_record_offset":39936,"inlinee":4497,"source_line":224,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":482,"record_offset":41044,"parent_record_offset":41020,"ancestor_record_offset":39936,"inlinee":5616,"source_line":261,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":482,"record_offset":40964,"parent_record_offset":40496,"ancestor_record_offset":39936,"inlinee":4506,"source_line":81,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":482,"record_offset":41020,"parent_record_offset":40996,"ancestor_record_offset":39936,"inlinee":5615,"source_line":734,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":482,"record_offset":40472,"parent_record_offset":40448,"ancestor_record_offset":39936,"inlinee":4478,"source_line":1156,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":482,"record_offset":40448,"parent_record_offset":40420,"ancestor_record_offset":39936,"inlinee":4477,"source_line":1179,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":482,"record_offset":40496,"parent_record_offset":40472,"ancestor_record_offset":39936,"inlinee":4480,"source_line":1862,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":482,"record_offset":40548,"parent_record_offset":40496,"ancestor_record_offset":39936,"inlinee":4515,"source_line":2301,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":482,"record_offset":40996,"parent_record_offset":40496,"ancestor_record_offset":39936,"inlinee":4495,"source_line":1166,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::borrow
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison.rs
//     evidence={"module":482,"record_offset":41600,"parent_record_offset":41576,"ancestor_record_offset":39936,"inlinee":5620,"source_line":116,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison.rs
//     evidence={"module":482,"record_offset":41624,"parent_record_offset":41600,"ancestor_record_offset":39936,"inlinee":5621,"source_line":144,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::rwlock::RwLock::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison\rwlock.rs
//     evidence={"module":482,"record_offset":41360,"parent_record_offset":41308,"ancestor_record_offset":39936,"inlinee":8101,"source_line":367,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::rwlock::RwLockReadGuard::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison\rwlock.rs
//     evidence={"module":482,"record_offset":41576,"parent_record_offset":41360,"ancestor_record_offset":39936,"inlinee":7022,"source_line":682,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::rwlock::impl$31::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison\rwlock.rs
//     evidence={"module":482,"record_offset":42704,"parent_record_offset":42676,"ancestor_record_offset":39936,"inlinee":7028,"source_line":816,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::rwlock::futex::RwLock::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\sync\rwlock\futex.rs
//     evidence={"module":482,"record_offset":41396,"parent_record_offset":41360,"ancestor_record_offset":39936,"inlinee":7017,"source_line":94,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::rwlock::futex::RwLock::read_unlock
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\sync\rwlock\futex.rs
//     evidence={"module":482,"record_offset":42732,"parent_record_offset":42704,"ancestor_record_offset":39936,"inlinee":7029,"source_line":110,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::rwlock::futex::is_read_lockable
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\sync\rwlock\futex.rs
//     evidence={"module":482,"record_offset":41488,"parent_record_offset":41396,"ancestor_record_offset":39936,"inlinee":7019,"source_line":47,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::draw_passes::draw_shaders::stairs_draw::stairs_draw::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 23
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x689B
// 首条位置: {"module":482,"inlinee":26779}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] bevy_ecs::world::entity_ref::EntityRef::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/entity_ref.rs
//     evidence={"module":482,"record_offset":40236,"parent_record_offset":40204,"ancestor_record_offset":40204,"inlinee":26781,"source_line":133,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":482,"record_offset":43216,"parent_record_offset":40204,"ancestor_record_offset":40204,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::Components::get_id
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/component.rs
//     evidence={"module":482,"record_offset":40320,"parent_record_offset":40276,"ancestor_record_offset":40204,"inlinee":26784,"source_line":2427,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::impl$19::get_id::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/component.rs
//     evidence={"module":482,"record_offset":41308,"parent_record_offset":41272,"ancestor_record_offset":40204,"inlinee":26786,"source_line":2428,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::impl$19::get_id::closure$0::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/component.rs
//     evidence={"module":482,"record_offset":42644,"parent_record_offset":42620,"ancestor_record_offset":40204,"inlinee":26788,"source_line":2434,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":482,"record_offset":43088,"parent_record_offset":43064,"ancestor_record_offset":40204,"inlinee":4854,"source_line":721,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeEntityCell::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":482,"record_offset":40276,"parent_record_offset":40236,"ancestor_record_offset":40204,"inlinee":26783,"source_line":785,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::fetch_table
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":482,"record_offset":43064,"parent_record_offset":43036,"ancestor_record_offset":40204,"inlinee":26791,"source_line":1161,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::get_component
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":482,"record_offset":43036,"parent_record_offset":40276,"ancestor_record_offset":40204,"inlinee":26790,"source_line":1189,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] bevy_platform::collections::hash_map::HashMap::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_platform","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_platform-0.16.0/src/collections/hash_map.rs
//     evidence={"module":482,"record_offset":40360,"parent_record_offset":40320,"ancestor_record_offset":40204,"inlinee":5614,"source_line":933,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$7::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\any.rs
//     evidence={"module":482,"record_offset":41152,"parent_record_offset":41128,"ancestor_record_offset":40204,"inlinee":5068,"source_line":719,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$25::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":482,"record_offset":41200,"parent_record_offset":41176,"ancestor_record_offset":40204,"inlinee":5070,"source_line":1740,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":482,"record_offset":40600,"parent_record_offset":40576,"ancestor_record_offset":40204,"inlinee":4484,"source_line":1227,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":482,"record_offset":40684,"parent_record_offset":40660,"ancestor_record_offset":40204,"inlinee":4486,"source_line":1433,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":482,"record_offset":40624,"parent_record_offset":40600,"ancestor_record_offset":40204,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":482,"record_offset":40836,"parent_record_offset":40808,"ancestor_record_offset":40204,"inlinee":4503,"source_line":449,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":482,"record_offset":40808,"parent_record_offset":40780,"ancestor_record_offset":40204,"inlinee":4493,"source_line":601,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":482,"record_offset":42620,"parent_record_offset":41308,"ancestor_record_offset":40204,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":482,"record_offset":41272,"parent_record_offset":40320,"ancestor_record_offset":40204,"inlinee":4571,"source_line":1546,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$39::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":482,"record_offset":43008,"parent_record_offset":40276,"ancestor_record_offset":40204,"inlinee":4927,"source_line":2486,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":482,"record_offset":42676,"parent_record_offset":41308,"ancestor_record_offset":40204,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":482,"record_offset":41068,"parent_record_offset":41044,"ancestor_record_offset":40204,"inlinee":4723,"source_line":1129,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":482,"record_offset":43112,"parent_record_offset":43088,"ancestor_record_offset":40204,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_noubcheck
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":482,"record_offset":43160,"parent_record_offset":43136,"ancestor_record_offset":40204,"inlinee":4164,"source_line":87,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":482,"record_offset":43136,"parent_record_offset":43112,"ancestor_record_offset":40204,"inlinee":4145,"source_line":226,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":482,"record_offset":41648,"parent_record_offset":41624,"ancestor_record_offset":40204,"inlinee":5623,"source_line":623,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":482,"record_offset":41516,"parent_record_offset":41396,"ancestor_record_offset":40204,"inlinee":7020,"source_line":2920,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::fetch_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":482,"record_offset":42768,"parent_record_offset":42732,"ancestor_record_offset":40204,"inlinee":7030,"source_line":2986,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":482,"record_offset":41432,"parent_record_offset":41396,"ancestor_record_offset":40204,"inlinee":4181,"source_line":2698,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":482,"record_offset":41540,"parent_record_offset":41516,"ancestor_record_offset":40204,"inlinee":5825,"source_line":3828,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":482,"record_offset":41456,"parent_record_offset":41432,"ancestor_record_offset":40204,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":482,"record_offset":42792,"parent_record_offset":42768,"ancestor_record_offset":40204,"inlinee":4778,"source_line":3777,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::tuple::impl$10::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\tuple.rs
//     evidence={"module":482,"record_offset":41176,"parent_record_offset":41152,"ancestor_record_offset":40204,"inlinee":4662,"source_line":30,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] equivalent::impl$0::equivalent
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"equivalent","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/equivalent-1.0.1/src/lib.rs
//     evidence={"module":482,"record_offset":41128,"parent_record_offset":41104,"ancestor_record_offset":40204,"inlinee":4660,"source_line":88,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":482,"record_offset":40744,"parent_record_offset":40716,"ancestor_record_offset":40204,"inlinee":4490,"source_line":49,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":482,"record_offset":40780,"parent_record_offset":40744,"ancestor_record_offset":40204,"inlinee":4491,"source_line":74,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":482,"record_offset":40716,"parent_record_offset":40496,"ancestor_record_offset":40204,"inlinee":4488,"source_line":112,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":482,"record_offset":40576,"parent_record_offset":40496,"ancestor_record_offset":40204,"inlinee":4482,"source_line":51,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":482,"record_offset":40880,"parent_record_offset":40496,"ancestor_record_offset":40204,"inlinee":4504,"source_line":94,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":482,"record_offset":40660,"parent_record_offset":40496,"ancestor_record_offset":40204,"inlinee":4485,"source_line":76,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/map.rs
//     evidence={"module":482,"record_offset":40396,"parent_record_offset":40360,"ancestor_record_offset":40204,"inlinee":4719,"source_line":1304,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":482,"record_offset":40420,"parent_record_offset":40396,"ancestor_record_offset":40204,"inlinee":4475,"source_line":1347,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":482,"record_offset":41104,"parent_record_offset":40996,"ancestor_record_offset":40204,"inlinee":4497,"source_line":224,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":482,"record_offset":41044,"parent_record_offset":41020,"ancestor_record_offset":40204,"inlinee":5616,"source_line":261,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":482,"record_offset":40964,"parent_record_offset":40496,"ancestor_record_offset":40204,"inlinee":4506,"source_line":81,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":482,"record_offset":41020,"parent_record_offset":40996,"ancestor_record_offset":40204,"inlinee":5615,"source_line":734,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":482,"record_offset":40472,"parent_record_offset":40448,"ancestor_record_offset":40204,"inlinee":4478,"source_line":1156,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":482,"record_offset":40448,"parent_record_offset":40420,"ancestor_record_offset":40204,"inlinee":4477,"source_line":1179,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":482,"record_offset":40496,"parent_record_offset":40472,"ancestor_record_offset":40204,"inlinee":4480,"source_line":1862,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":482,"record_offset":40548,"parent_record_offset":40496,"ancestor_record_offset":40204,"inlinee":4515,"source_line":2301,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":482,"record_offset":40996,"parent_record_offset":40496,"ancestor_record_offset":40204,"inlinee":4495,"source_line":1166,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::borrow
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison.rs
//     evidence={"module":482,"record_offset":41600,"parent_record_offset":41576,"ancestor_record_offset":40204,"inlinee":5620,"source_line":116,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison.rs
//     evidence={"module":482,"record_offset":41624,"parent_record_offset":41600,"ancestor_record_offset":40204,"inlinee":5621,"source_line":144,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::rwlock::RwLock::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison\rwlock.rs
//     evidence={"module":482,"record_offset":41360,"parent_record_offset":41308,"ancestor_record_offset":40204,"inlinee":8101,"source_line":367,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::rwlock::RwLockReadGuard::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison\rwlock.rs
//     evidence={"module":482,"record_offset":41576,"parent_record_offset":41360,"ancestor_record_offset":40204,"inlinee":7022,"source_line":682,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::rwlock::impl$31::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison\rwlock.rs
//     evidence={"module":482,"record_offset":42704,"parent_record_offset":42676,"ancestor_record_offset":40204,"inlinee":7028,"source_line":816,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::rwlock::futex::RwLock::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\sync\rwlock\futex.rs
//     evidence={"module":482,"record_offset":41396,"parent_record_offset":41360,"ancestor_record_offset":40204,"inlinee":7017,"source_line":94,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::rwlock::futex::RwLock::read_unlock
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\sync\rwlock\futex.rs
//     evidence={"module":482,"record_offset":42732,"parent_record_offset":42704,"ancestor_record_offset":40204,"inlinee":7029,"source_line":110,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::rwlock::futex::is_read_lockable
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\sync\rwlock\futex.rs
//     evidence={"module":482,"record_offset":41488,"parent_record_offset":41396,"ancestor_record_offset":40204,"inlinee":7019,"source_line":47,"archive":"_pdb/B/modules/00482.jsonl.gz"}
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
// 完整结构及原始类型记录: stairs_draw.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: stairs_draw.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === RENDER PASSES V1 BEGIN ===
// 渲染 Pass: crates/country-core/src/systems/render_loop/draw_passes/draw_shaders/stairs_draw.rs
// 范围: PDB/API 事实、Shader 声明、映射候选、未知分别标注。
// 完整记录: stairs_draw.rs.render-pass.json.gz
// 输入/输出: 具体参数、运行时资源句柄及完整返回资源 = unknown；可确认的 API 操作及 Shader 声明见下方。
// 资源依赖: 跨 Pass 实际 producer/consumer、资源版本、队列/屏障参数 = unknown。
// 调用顺序: 只记录同一物理父函数内唯一 PDB 目标名称的 callsite 地址排列；分支、循环和 GPU 执行先后 = unknown。
// 同地址别名目标候选 0 条：全部保留在附档，但是否调用本 Pass = unknown，不列为已确认 Pass 顺序边。
// ------------------------------------------------------------------
// [PDB 函数上下文] country_core::systems::render_loop::draw_passes::draw_shaders::stairs_draw::stairs_draw
// [资源接口动作；资源实例 unknown] {"role":"texture_read_api","symbol":"country_core::render::pass_builder::draw_builder::impl$2::read_texture","relationship":"pdb_inline_direct","evidence":{"module":482,"record_offset":43288,"parent_record_offset":39936,"ancestor_record_offset":39936,"inlinee":8283,"source_line":524,"archive":"_pdb/B/modules/00482.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// [资源接口动作；资源实例 unknown] {"role":"uniform_input_api","symbol":"country_core::render::pass_builder::draw_builder::impl$2::uniform","relationship":"pdb_inline_direct","evidence":{"module":482,"record_offset":43332,"parent_record_offset":39936,"ancestor_record_offset":39936,"inlinee":26792,"source_line":502,"archive":"_pdb/B/modules/00482.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// Shader 二进制文件映射 = unknown（没有足够名称/类型关联；不按效果名称强行绑定）。
// 父函数直接调用位置 = unknown/当前扫描未找到；PDB 内联及间接调用不能据此排除。
// 全部直接引用方/目标别名、原指令字节及位置见同名 render-pass.json.gz；没有裁定同地址唯一符号。
// Shader 方程、启用条件、实际执行和完整渲染器实现 = unknown。
// === RENDER PASSES V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: stairs_draw.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::draw_passes::draw_shaders::stairs_draw::stairs_draw
// 信息 ID: 0018c9b8f2c9aa45547e9761
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::draw_passes::draw_shaders::stairs_draw::stairs_draw::closure$0
// 信息 ID: 5d048bcad63014d44eef8e51
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::render_loop::draw_passes::draw_shaders::stairs_draw::stairs_draw", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::draw_shaders::stairs_draw::stairs_draw; module=482, inline_record=40204; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
