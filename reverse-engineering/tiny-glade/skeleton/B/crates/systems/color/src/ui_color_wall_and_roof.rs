// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/systems/color/src/ui_color_wall_and_roof.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: d893e55f967a810b351231ab82497afe48b2a8b8d24bb788882c32c131764529
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: ui_color_wall_and_roof.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 1061, 2567, 3775
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use bevy_ecs; // namespace candidate
//   package identities: [{"name":"bevy_ecs","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.0"}]
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"bitflags","source":"PDB_registry_path_only","version":"1.3.2"},{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.29.3"}]
// use nonmax; // namespace candidate
//   package identities: [{"name":"nonmax","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.5.5"}]
// use std; // namespace candidate
//   package identities: []
// use system_color; // namespace candidate
//   package identities: [{"name":"system-color","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_color::ui_color_wall_and_roof::ui_color_wall_and_roof
// 记录形式: compiled_procedure; instances=1
// 源码行: 23
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":2567,"record_offset":380,"rva":29993840,"va":5398702960}
// 编译实例 RVAs: 0x1c9ab70
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: ui_color_wall_and_roof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-23623871","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398702960,"raw_hex":"e8418797fe","size":5,"soft_fail":false,"target_va":5375081504,"va":5398705370,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140895470
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: ui_color_wall_and_roof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-20995866","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398702960,"raw_hex":"e8e6a0bffe","size":5,"soft_fail":false,"target_va":5377709168,"va":5398705029,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1422bc980
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ui_color_wall_and_roof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t6429958","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398702960,"raw_hex":"e8061d6200","size":5,"soft_fail":false,"target_va":5405133184,"va":5398703221,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1422bcc20
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ui_color_wall_and_roof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t6429763","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398702960,"raw_hex":"e8431c6200","size":5,"soft_fail":false,"target_va":5405133856,"va":5398704088,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1422bd120
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ui_color_wall_and_roof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t6431013","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398702960,"raw_hex":"e825216200","size":5,"soft_fail":false,"target_va":5405135136,"va":5398704118,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1422bd3c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ui_color_wall_and_roof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t6432554","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398702960,"raw_hex":"e82a276200","size":5,"soft_fail":false,"target_va":5405135808,"va":5398703249,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: ui_color_wall_and_roof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13143243","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398702960,"raw_hex":"e8cb8cc800","size":5,"soft_fail":false,"target_va":5411847168,"va":5398703920,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-15863586","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398702960,"raw_hex":"e8def00dff","size":5,"soft_fail":false,"target_va":5382840208,"va":5398703789,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t-24348341","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398702960,"raw_hex":"e84b798cfe","size":5,"soft_fail":false,"target_va":5374354992,"va":5398703328,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::collision::world::RaycastWorld::raycast
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-20767930","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398702960,"raw_hex":"e8461bc3fe","size":5,"soft_fail":false,"target_va":5377935472,"va":5398703397,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::resources::ui_state::UiWidgetContext::clicked
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-20367040","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398702960,"raw_hex":"e84039c9fe","size":5,"soft_fail":false,"target_va":5378336240,"va":5398703275,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::resources::ui_state::UiWidgetContext::is_active
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-20367771","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398702960,"raw_hex":"e86536c9fe","size":5,"soft_fail":false,"target_va":5378336384,"va":5398704150,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::resources::widget_params::UiSounds::empty
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-19864695","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398702960,"raw_hex":"e889e3d0fe","size":5,"soft_fail":false,"target_va":5378838448,"va":5398703138,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::resources::widget_params::UiSounds::on_click
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-19865477","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398702960,"raw_hex":"e87be0d0fe","size":5,"soft_fail":false,"target_va":5378838560,"va":5398704032,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::resources::widget_params::UiSounds::on_hover_start
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-19865517","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398702960,"raw_hex":"e853e0d0fe","size":5,"soft_fail":false,"target_va":5378838480,"va":5398703992,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::resources::widget_params::WidgetParams::new
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-19865092","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398702960,"raw_hex":"e8fce1d0fe","size":5,"soft_fail":false,"target_va":5378838032,"va":5398703119,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::resources::widget_params::WidgetParams::occlude_by_world
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-19864969","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398702960,"raw_hex":"e877e2d0fe","size":5,"soft_fail":false,"target_va":5378838240,"va":5398703204,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::resources::widget_params::WidgetParams::sounds
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-19865029","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398702960,"raw_hex":"e83be2d0fe","size":5,"soft_fail":false,"target_va":5378838144,"va":5398703168,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::resources::widget_params::WidgetVolumeShape::clearing_canvas
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-19864293","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398702960,"raw_hex":"e81be5d0fe","size":5,"soft_fail":false,"target_va":5378838800,"va":5398703088,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":2567,"record_offset":3940,"parent_record_offset":380,"ancestor_record_offset":380,"inlinee":7051,"source_line":644,"archive":"_pdb/B/modules/02567.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::writer::EventWriter::write
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/writer.rs
//     evidence={"module":2567,"record_offset":552,"parent_record_offset":380,"ancestor_record_offset":380,"inlinee":7657,"source_line":74,"archive":"_pdb/B/modules/02567.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::query::Query::iter
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":2567,"record_offset":880,"parent_record_offset":380,"ancestor_record_offset":380,"inlinee":13704,"source_line":670,"archive":"_pdb/B/modules/02567.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::last
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":2567,"record_offset":1128,"parent_record_offset":380,"ancestor_record_offset":380,"inlinee":11662,"source_line":249,"archive":"_pdb/B/modules/02567.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":2567,"record_offset":4336,"parent_record_offset":380,"ancestor_record_offset":380,"inlinee":4512,"source_line":607,"archive":"_pdb/B/modules/02567.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":2567,"record_offset":5352,"parent_record_offset":380,"ancestor_record_offset":380,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/02567.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui::resources::widget_params::WidgetParams::with_flags
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/resources/widget_params.rs
//     evidence={"module":2567,"record_offset":4248,"parent_record_offset":380,"ancestor_record_offset":380,"inlinee":24901,"source_line":117,"archive":"_pdb/B/modules/02567.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::distance
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":2567,"record_offset":4024,"parent_record_offset":380,"ancestor_record_offset":380,"inlinee":13631,"source_line":477,"archive":"_pdb/B/modules/02567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":2567,"record_offset":5528,"parent_record_offset":5504,"ancestor_record_offset":380,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/02567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":2567,"record_offset":5504,"parent_record_offset":5448,"ancestor_record_offset":380,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/02567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":2567,"record_offset":3988,"parent_record_offset":3964,"ancestor_record_offset":380,"inlinee":7053,"source_line":654,"archive":"_pdb/B/modules/02567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":2567,"record_offset":748,"parent_record_offset":716,"ancestor_record_offset":380,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/02567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":2567,"record_offset":5476,"parent_record_offset":5448,"ancestor_record_offset":380,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/02567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":2567,"record_offset":5448,"parent_record_offset":5424,"ancestor_record_offset":380,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/02567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":2567,"record_offset":796,"parent_record_offset":772,"ancestor_record_offset":380,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/02567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":2567,"record_offset":772,"parent_record_offset":748,"ancestor_record_offset":380,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/02567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":2567,"record_offset":5424,"parent_record_offset":5400,"ancestor_record_offset":380,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/02567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":2567,"record_offset":716,"parent_record_offset":676,"ancestor_record_offset":380,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/02567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":2567,"record_offset":3744,"parent_record_offset":3720,"ancestor_record_offset":380,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/02567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":2567,"record_offset":1080,"parent_record_offset":1056,"ancestor_record_offset":380,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/02567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":2567,"record_offset":2144,"parent_record_offset":2120,"ancestor_record_offset":380,"inlinee":4352,"source_line":2680,"archive":"_pdb/B/modules/02567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":2567,"record_offset":676,"parent_record_offset":640,"ancestor_record_offset":380,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/02567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":2567,"record_offset":1056,"parent_record_offset":1032,"ancestor_record_offset":380,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/02567.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::entities
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":2567,"record_offset":3672,"parent_record_offset":3140,"ancestor_record_offset":380,"inlinee":11682,"source_line":466,"archive":"_pdb/B/modules/02567.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::len
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":2567,"record_offset":2408,"parent_record_offset":1372,"ancestor_record_offset":380,"inlinee":55771,"source_line":609,"archive":"_pdb/B/modules/02567.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::table_id
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":2567,"record_offset":2288,"parent_record_offset":1372,"ancestor_record_offset":380,"inlinee":5495,"source_line":460,"archive":"_pdb/B/modules/02567.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetypes::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":2567,"record_offset":2176,"parent_record_offset":1372,"ancestor_record_offset":380,"inlinee":11678,"source_line":895,"archive":"_pdb/B/modules/02567.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":2567,"record_offset":612,"parent_record_offset":576,"ancestor_record_offset":380,"inlinee":4459,"source_line":397,"archive":"_pdb/B/modules/02567.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":2567,"record_offset":576,"parent_record_offset":552,"ancestor_record_offset":380,"inlinee":4457,"source_line":434,"archive":"_pdb/B/modules/02567.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::collections::Events::send_with_caller
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/collections.rs
//     evidence={"module":2567,"record_offset":640,"parent_record_offset":576,"ancestor_record_offset":380,"inlinee":7659,"source_line":128,"archive":"_pdb/B/modules/02567.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$26::set_archetype
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":2567,"record_offset":2500,"parent_record_offset":2464,"ancestor_record_offset":380,"inlinee":4856,"source_line":1160,"archive":"_pdb/B/modules/02567.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$26::set_table
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":2567,"record_offset":1536,"parent_record_offset":1444,"ancestor_record_offset":380,"inlinee":4857,"source_line":1175,"archive":"_pdb/B/modules/02567.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIter::fold_over_archetype_range
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs
//     evidence={"module":2567,"record_offset":3140,"parent_record_offset":1372,"ancestor_record_offset":380,"inlinee":73843,"source_line":252,"archive":"_pdb/B/modules/02567.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIter::fold_over_dense_archetype_range
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs
//     evidence={"module":2567,"record_offset":2464,"parent_record_offset":1372,"ancestor_record_offset":380,"inlinee":73842,"source_line":321,"archive":"_pdb/B/modules/02567.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIter::fold_over_storage_range
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs
//     evidence={"module":2567,"record_offset":1372,"parent_record_offset":1164,"ancestor_record_offset":380,"inlinee":61776,"source_line":134,"archive":"_pdb/B/modules/02567.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIter::fold_over_table_range
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs
//     evidence={"module":2567,"record_offset":1444,"parent_record_offset":1372,"ancestor_record_offset":380,"inlinee":61777,"source_line":197,"archive":"_pdb/B/modules/02567.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIter::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs
//     evidence={"module":2567,"record_offset":1008,"parent_record_offset":908,"ancestor_record_offset":380,"inlinee":13709,"source_line":39,"archive":"_pdb/B/modules/02567.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::init
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs
//     evidence={"module":2567,"record_offset":1032,"parent_record_offset":1008,"ancestor_record_offset":380,"inlinee":9292,"source_line":2359,"archive":"_pdb/B/modules/02567.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::impl$1::fold
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs
//     evidence={"module":2567,"record_offset":1164,"parent_record_offset":1128,"ancestor_record_offset":380,"inlinee":61775,"source_line":902,"archive":"_pdb/B/modules/02567.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_array::BlobArray::get_sub_slice
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_array.rs
//     evidence={"module":2567,"record_offset":1956,"parent_record_offset":1932,"ancestor_record_offset":380,"inlinee":4872,"source_line":143,"archive":"_pdb/B/modules/02567.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseArray::get
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":2567,"record_offset":1636,"parent_record_offset":1612,"ancestor_record_offset":380,"inlinee":4864,"source_line":58,"archive":"_pdb/B/modules/02567.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":2567,"record_offset":1612,"parent_record_offset":1588,"ancestor_record_offset":380,"inlinee":4862,"source_line":395,"archive":"_pdb/B/modules/02567.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$16::get::closure$0
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":2567,"record_offset":1816,"parent_record_offset":1792,"ancestor_record_offset":380,"inlinee":4866,"source_line":396,"archive":"_pdb/B/modules/02567.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::entity_count
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":2567,"record_offset":2120,"parent_record_offset":1372,"ancestor_record_offset":380,"inlinee":15359,"source_line":588,"archive":"_pdb/B/modules/02567.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_column
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":2567,"record_offset":1588,"parent_record_offset":1560,"ancestor_record_offset":380,"inlinee":4860,"source_line":475,"archive":"_pdb/B/modules/02567.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_data_slice_for
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":2567,"record_offset":1560,"parent_record_offset":1536,"ancestor_record_offset":380,"inlinee":4859,"source_line":363,"archive":"_pdb/B/modules/02567.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":2567,"record_offset":2008,"parent_record_offset":1372,"ancestor_record_offset":380,"inlinee":4854,"source_line":721,"archive":"_pdb/B/modules/02567.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::column::ThinColumn::get_data_slice
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/column.rs
//     evidence={"module":2567,"record_offset":1932,"parent_record_offset":1908,"ancestor_record_offset":380,"inlinee":4870,"source_line":303,"archive":"_pdb/B/modules/02567.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::impl$4::get_data_slice_for::closure$0
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":2567,"record_offset":1908,"parent_record_offset":1884,"ancestor_record_offset":380,"inlinee":4868,"source_line":368,"archive":"_pdb/B/modules/02567.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::as_readonly
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":2567,"record_offset":908,"parent_record_offset":880,"ancestor_record_offset":380,"inlinee":13705,"source_line":542,"archive":"_pdb/B/modules/02567.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::copy_unsafe
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":2567,"record_offset":972,"parent_record_offset":944,"ancestor_record_offset":380,"inlinee":13707,"source_line":636,"archive":"_pdb/B/modules/02567.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::reborrow_unsafe
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":2567,"record_offset":944,"parent_record_offset":908,"ancestor_record_offset":380,"inlinee":13706,"source_line":618,"archive":"_pdb/B/modules/02567.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::copied::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\copied.rs
//     evidence={"module":2567,"record_offset":1212,"parent_record_offset":1164,"ancestor_record_offset":380,"inlinee":5527,"source_line":45,"archive":"_pdb/B/modules/02567.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$43::forward_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":2567,"record_offset":3064,"parent_record_offset":3024,"ancestor_record_offset":380,"inlinee":4764,"source_line":203,"archive":"_pdb/B/modules/02567.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":2567,"record_offset":1504,"parent_record_offset":1480,"ancestor_record_offset":380,"inlinee":4688,"source_line":763,"archive":"_pdb/B/modules/02567.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":2567,"record_offset":1480,"parent_record_offset":1444,"ancestor_record_offset":380,"inlinee":4686,"source_line":848,"archive":"_pdb/B/modules/02567.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":2567,"record_offset":3092,"parent_record_offset":3064,"ancestor_record_offset":380,"inlinee":4562,"source_line":560,"archive":"_pdb/B/modules/02567.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":2567,"record_offset":4092,"parent_record_offset":4056,"ancestor_record_offset":380,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/02567.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":2567,"record_offset":1728,"parent_record_offset":1664,"ancestor_record_offset":380,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/02567.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":2567,"record_offset":1752,"parent_record_offset":1728,"ancestor_record_offset":380,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/02567.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::copied
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":2567,"record_offset":1336,"parent_record_offset":1212,"ancestor_record_offset":380,"inlinee":4665,"source_line":1892,"archive":"_pdb/B/modules/02567.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":2567,"record_offset":1792,"parent_record_offset":1612,"ancestor_record_offset":380,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/02567.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":2567,"record_offset":3964,"parent_record_offset":3940,"ancestor_record_offset":380,"inlinee":6345,"source_line":1218,"archive":"_pdb/B/modules/02567.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":2567,"record_offset":5376,"parent_record_offset":5352,"ancestor_record_offset":380,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/02567.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":2567,"record_offset":1296,"parent_record_offset":1244,"ancestor_record_offset":380,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/02567.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":2567,"record_offset":836,"parent_record_offset":676,"ancestor_record_offset":380,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/02567.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":2567,"record_offset":1664,"parent_record_offset":1636,"ancestor_record_offset":380,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/02567.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_noubcheck
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":2567,"record_offset":2080,"parent_record_offset":2056,"ancestor_record_offset":380,"inlinee":4164,"source_line":87,"archive":"_pdb/B/modules/02567.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":2567,"record_offset":1700,"parent_record_offset":1664,"ancestor_record_offset":380,"inlinee":4145,"source_line":226,"archive":"_pdb/B/modules/02567.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":2567,"record_offset":1244,"parent_record_offset":1212,"ancestor_record_offset":380,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/02567.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::ui::resources::widget_params::WidgetFlags::insert
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitflags","version":"1.3.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-1.3.2/src/lib.rs
//     evidence={"module":2567,"record_offset":4300,"parent_record_offset":4276,"ancestor_record_offset":380,"inlinee":24904,"source_line":609,"archive":"_pdb/B/modules/02567.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":2567,"record_offset":4176,"parent_record_offset":4056,"ancestor_record_offset":380,"inlinee":4895,"source_line":203,"archive":"_pdb/B/modules/02567.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":2567,"record_offset":4056,"parent_record_offset":4024,"ancestor_record_offset":380,"inlinee":4743,"source_line":1473,"archive":"_pdb/B/modules/02567.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"nonmax","version":"0.5.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":2567,"record_offset":1840,"parent_record_offset":1816,"ancestor_record_offset":380,"inlinee":4171,"source_line":153,"archive":"_pdb/B/modules/02567.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":2567,"record_offset":4208,"parent_record_offset":4176,"ancestor_record_offset":380,"inlinee":4750,"source_line":369,"archive":"_pdb/B/modules/02567.jsonl.gz"}
//   [pdb_inline_transitive] system_color::ui_color_wall_and_roof::ui_color_wall_and_roof::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-color","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/color/src/ui_color_wall_and_roof.rs
//     evidence={"module":2567,"record_offset":4276,"parent_record_offset":4248,"ancestor_record_offset":380,"inlinee":97817,"source_line":126,"archive":"_pdb/B/modules/02567.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_color::ui_color_wall_and_roof::ui_color_wall_and_roof::closure$0
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=3
// 源码行: 53
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xE46D
// 首条位置: {"module":1061,"inlinee":58477}
// 编译实例 RVAs: 0xed2580
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: ui_color_wall_and_roof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9169321","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384250752,"raw_hex":"e8571674ff","size":5,"soft_fail":false,"target_va":5375081488,"va":5384250804,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: ui_color_wall_and_roof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9169447","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384250752,"raw_hex":"e8d91574ff","size":5,"soft_fail":false,"target_va":5375081504,"va":5384250946,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t27290083","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384250752,"raw_hex":"e8e369a001","size":5,"soft_fail":false,"target_va":5411540992,"va":5384250904,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_color::ui_color_wall_and_roof::color_cursor_icon
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"system-color","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t14454570","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384250752,"raw_hex":"e82a8fdc00","size":5,"soft_fail":false,"target_va":5398705424,"va":5384250849,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1061,"record_offset":41840,"parent_record_offset":41808,"ancestor_record_offset":41808,"inlinee":4281,"source_line":347,"archive":"_pdb/B/modules/01061.jsonl.gz"}
//   [pdb_inline_direct] alloc::slice::impl$0::into_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1061,"record_offset":41984,"parent_record_offset":41808,"ancestor_record_offset":41808,"inlinee":7065,"source_line":526,"archive":"_pdb/B/modules/01061.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1061,"record_offset":42068,"parent_record_offset":41808,"ancestor_record_offset":41808,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01061.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1061,"record_offset":41892,"parent_record_offset":41868,"ancestor_record_offset":41808,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/01061.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1061,"record_offset":41916,"parent_record_offset":41892,"ancestor_record_offset":41808,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/01061.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1061,"record_offset":42140,"parent_record_offset":42116,"ancestor_record_offset":41808,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01061.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1061,"record_offset":41868,"parent_record_offset":41840,"ancestor_record_offset":41808,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/01061.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1061,"record_offset":42116,"parent_record_offset":42092,"ancestor_record_offset":41808,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01061.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":1061,"record_offset":42092,"parent_record_offset":42068,"ancestor_record_offset":41808,"inlinee":4364,"source_line":1676,"archive":"_pdb/B/modules/01061.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::into_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1061,"record_offset":42008,"parent_record_offset":41984,"ancestor_record_offset":41808,"inlinee":7066,"source_line":99,"archive":"_pdb/B/modules/01061.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::from_raw_parts_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1061,"record_offset":42032,"parent_record_offset":42008,"ancestor_record_offset":41808,"inlinee":7067,"source_line":934,"archive":"_pdb/B/modules/01061.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1061,"record_offset":41940,"parent_record_offset":41916,"ancestor_record_offset":41808,"inlinee":4282,"source_line":1740,"archive":"_pdb/B/modules/01061.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_color::ui_color_wall_and_roof::ui_color_wall_and_roof::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 126
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x17E19
// 首条位置: {"module":2567,"inlinee":97817}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::systems::ui::resources::widget_params::WidgetFlags::insert
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitflags","version":"1.3.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-1.3.2/src/lib.rs
//     evidence={"module":2567,"record_offset":4300,"parent_record_offset":4276,"ancestor_record_offset":4276,"inlinee":24904,"source_line":609,"archive":"_pdb/B/modules/02567.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_color::ui_color_wall_and_roof::ui_color_wall_and_roof::closure$2
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=3
// 源码行: 137
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xE46E
// 首条位置: {"module":1061,"inlinee":58478}
// 编译实例 RVAs: 0xed2660
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: ui_color_wall_and_roof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9169545","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384250976,"raw_hex":"e8771574ff","size":5,"soft_fail":false,"target_va":5375081488,"va":5384251028,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: ui_color_wall_and_roof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9169671","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384250976,"raw_hex":"e8f91474ff","size":5,"soft_fail":false,"target_va":5375081504,"va":5384251170,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t27289859","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384250976,"raw_hex":"e80369a001","size":5,"soft_fail":false,"target_va":5411540992,"va":5384251128,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_color::ui_color_wall_and_roof::color_cursor_icon
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"system-color","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t14454346","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384250976,"raw_hex":"e84a8edc00","size":5,"soft_fail":false,"target_va":5398705424,"va":5384251073,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1061,"record_offset":42644,"parent_record_offset":42612,"ancestor_record_offset":42612,"inlinee":4281,"source_line":347,"archive":"_pdb/B/modules/01061.jsonl.gz"}
//   [pdb_inline_direct] alloc::slice::impl$0::into_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1061,"record_offset":42788,"parent_record_offset":42612,"ancestor_record_offset":42612,"inlinee":7065,"source_line":526,"archive":"_pdb/B/modules/01061.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1061,"record_offset":42872,"parent_record_offset":42612,"ancestor_record_offset":42612,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01061.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1061,"record_offset":42696,"parent_record_offset":42672,"ancestor_record_offset":42612,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/01061.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1061,"record_offset":42720,"parent_record_offset":42696,"ancestor_record_offset":42612,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/01061.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1061,"record_offset":42944,"parent_record_offset":42920,"ancestor_record_offset":42612,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01061.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1061,"record_offset":42672,"parent_record_offset":42644,"ancestor_record_offset":42612,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/01061.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1061,"record_offset":42920,"parent_record_offset":42896,"ancestor_record_offset":42612,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01061.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":1061,"record_offset":42896,"parent_record_offset":42872,"ancestor_record_offset":42612,"inlinee":4364,"source_line":1676,"archive":"_pdb/B/modules/01061.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::into_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1061,"record_offset":42812,"parent_record_offset":42788,"ancestor_record_offset":42612,"inlinee":7066,"source_line":99,"archive":"_pdb/B/modules/01061.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::from_raw_parts_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1061,"record_offset":42836,"parent_record_offset":42812,"ancestor_record_offset":42612,"inlinee":7067,"source_line":934,"archive":"_pdb/B/modules/01061.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1061,"record_offset":42744,"parent_record_offset":42720,"ancestor_record_offset":42612,"inlinee":4282,"source_line":1740,"archive":"_pdb/B/modules/01061.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_color::ui_color_wall_and_roof::color_cursor_icon
// 记录形式: compiled_procedure; instances=1
// 源码行: 187
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":2567,"record_offset":5612,"rva":29996304,"va":5398705424}
// 编译实例 RVAs: 0x1c9b510
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: ui_color_wall_and_roof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-23624192","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398705424,"raw_hex":"e8008697fe","size":5,"soft_fail":false,"target_va":5375081488,"va":5398705675,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVecInner$LT$A$GT$::reserve::do_reserve_and_handle
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t13062361","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398705424,"raw_hex":"e8d950c700","size":5,"soft_fail":false,"target_va":5411768304,"va":5398705938,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t12834959","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398705424,"raw_hex":"e88fd8c300","size":5,"soft_fail":false,"target_va":5411540963,"va":5398705999,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::system::commands::EntityCommands::insert
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t6435888","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398705424,"raw_hex":"e830346200","size":5,"soft_fail":false,"target_va":5405141680,"va":5398705787,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::animation_duration
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-20961529","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398705424,"raw_hex":"e80727c0fe","size":5,"soft_fail":false,"target_va":5377744016,"va":5398705540,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::build
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-20961361","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398705424,"raw_hex":"e8af27c0fe","size":5,"soft_fail":false,"target_va":5377744208,"va":5398705564,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-20962243","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398705424,"raw_hex":"e83d24c0fe","size":5,"soft_fail":false,"target_va":5377743280,"va":5398705518,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::system::commands::EntityCommands::insert
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs
//     evidence={"module":2567,"record_offset":5756,"parent_record_offset":5612,"ancestor_record_offset":5612,"inlinee":9427,"source_line":1347,"archive":"_pdb/B/modules/02567.jsonl.gz"}
//   [pdb_inline_direct] country_core::components::Ui2dImage::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/components/mod.rs
//     evidence={"module":2567,"record_offset":6060,"parent_record_offset":5612,"ancestor_record_offset":5612,"inlinee":24881,"source_line":229,"archive":"_pdb/B/modules/02567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":2567,"record_offset":6468,"parent_record_offset":6444,"ancestor_record_offset":5612,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/02567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":2567,"record_offset":6492,"parent_record_offset":6468,"ancestor_record_offset":5612,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/02567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":2567,"record_offset":6444,"parent_record_offset":6416,"ancestor_record_offset":5612,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/02567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":2567,"record_offset":5908,"parent_record_offset":5880,"ancestor_record_offset":5612,"inlinee":4962,"source_line":326,"archive":"_pdb/B/modules/02567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":2567,"record_offset":6356,"parent_record_offset":6328,"ancestor_record_offset":5612,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/02567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":2567,"record_offset":5992,"parent_record_offset":5968,"ancestor_record_offset":5612,"inlinee":4737,"source_line":512,"archive":"_pdb/B/modules/02567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":2567,"record_offset":5968,"parent_record_offset":5936,"ancestor_record_offset":5612,"inlinee":4964,"source_line":622,"archive":"_pdb/B/modules/02567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":2567,"record_offset":5936,"parent_record_offset":5908,"ancestor_record_offset":5612,"inlinee":4963,"source_line":541,"archive":"_pdb/B/modules/02567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":2567,"record_offset":6416,"parent_record_offset":6384,"ancestor_record_offset":5612,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/02567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":2567,"record_offset":6384,"parent_record_offset":6356,"ancestor_record_offset":5612,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/02567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":2567,"record_offset":6292,"parent_record_offset":6264,"ancestor_record_offset":5612,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/02567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":2567,"record_offset":6264,"parent_record_offset":6236,"ancestor_record_offset":5612,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/02567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":2567,"record_offset":6208,"parent_record_offset":6180,"ancestor_record_offset":5612,"inlinee":4247,"source_line":476,"archive":"_pdb/B/modules/02567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":2567,"record_offset":6236,"parent_record_offset":6208,"ancestor_record_offset":5612,"inlinee":4248,"source_line":500,"archive":"_pdb/B/modules/02567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":2567,"record_offset":6180,"parent_record_offset":6152,"ancestor_record_offset":5612,"inlinee":4245,"source_line":887,"archive":"_pdb/B/modules/02567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\str.rs
//     evidence={"module":2567,"record_offset":6152,"parent_record_offset":6124,"ancestor_record_offset":5612,"inlinee":4243,"source_line":210,"archive":"_pdb/B/modules/02567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$47::from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":2567,"record_offset":6124,"parent_record_offset":6096,"ancestor_record_offset":5612,"inlinee":5924,"source_line":2956,"archive":"_pdb/B/modules/02567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":2567,"record_offset":5880,"parent_record_offset":5844,"ancestor_record_offset":5612,"inlinee":4961,"source_line":1271,"archive":"_pdb/B/modules/02567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":2567,"record_offset":6328,"parent_record_offset":6292,"ancestor_record_offset":5612,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/02567.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::queue
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs
//     evidence={"module":2567,"record_offset":5812,"parent_record_offset":5784,"ancestor_record_offset":5612,"inlinee":4959,"source_line":553,"archive":"_pdb/B/modules/02567.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::queue_internal
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs
//     evidence={"module":2567,"record_offset":5844,"parent_record_offset":5812,"ancestor_record_offset":5612,"inlinee":4960,"source_line":612,"archive":"_pdb/B/modules/02567.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::EntityCommands::queue
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs
//     evidence={"module":2567,"record_offset":5784,"parent_record_offset":5756,"ancestor_record_offset":5612,"inlinee":9420,"source_line":1798,"archive":"_pdb/B/modules/02567.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":2567,"record_offset":6096,"parent_record_offset":6060,"ancestor_record_offset":5612,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/02567.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":2567,"record_offset":6568,"parent_record_offset":6544,"ancestor_record_offset":5612,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/02567.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":2567,"record_offset":6544,"parent_record_offset":6292,"ancestor_record_offset":5612,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/02567.jsonl.gz"}
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
// 完整结构及原始类型记录: ui_color_wall_and_roof.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: ui_color_wall_and_roof.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: ui_color_wall_and_roof.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 18; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::event::writer::EventWriter<system_color::show_color_menu::ShowColorMenuCmd>,bevy_ecs::event::writer::EventWriter<system_color::show_color_menu::HideColorMenuCmd>,bevy_ecs::system::query::Query<ref$<system_color::radial_menu::ColorRadialMenu>,tuple$<> >,bevy_ecs::event::writer::EventWriter<country_core::systems::wall::structure_outline::OutlineWallCmd>,bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >", "bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>", "bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>", "bevy_ecs::event::writer::EventWriter<system_color::show_color_menu::ShowColorMenuCmd>", "bevy_ecs::event::writer::EventWriter<system_color::show_color_menu::HideColorMenuCmd>", "bevy_ecs::system::query::Query<ref$<system_color::radial_menu::ColorRadialMenu>,tuple$<> >", "bevy_ecs::event::writer::EventWriter<country_core::systems::wall::structure_outline::OutlineWallCmd>", "bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>", "bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<system_color::show_color_menu::ShowColorMenuCmd>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<system_color::show_color_menu::HideColorMenuCmd>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<ref$<system_color::radial_menu::ColorRadialMenu>,tuple$<> >
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::systems::wall::structure_outline::OutlineWallCmd>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::writer::EventWriter<system_color::show_color_menu::ShowColorMenuCmd>,bevy_ecs::event::writer::EventWriter<system_color::show_color_menu::HideColorMenuCmd>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>,bevy_ecs::event::writer::EventWriter<country_core::systems::wall::structure_outline::OutlineStairsCmd>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyRequests>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_state::StairsState>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >", "bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>", "bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>", "bevy_ecs::event::writer::EventWriter<system_color::show_color_menu::ShowColorMenuCmd>", "bevy_ecs::event::writer::EventWriter<system_color::show_color_menu::HideColorMenuCmd>", "bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>", "bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>", "country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>", "bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>", "bevy_ecs::event::writer::EventWriter<country_core::systems::wall::structure_outline::OutlineStairsCmd>", "bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyRequests>", "bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_state::StairsState>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<system_color::show_color_menu::ShowColorMenuCmd>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<system_color::show_color_menu::HideColorMenuCmd>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::systems::wall::structure_outline::OutlineStairsCmd>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyRequests>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_state::StairsState>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::walls::wooden_gates::Gates>,bevy_ecs::system::query::Query<ref$<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh> >,bevy_ecs::query::filter::With<country_core::components::WoodenGate> >,bevy_ecs::system::query::Query<ref$<country_core::components::WoodenGate>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::event::writer::EventWriter<system_color::show_color_menu::ShowColorMenuCmd>,bevy_ecs::event::writer::EventWriter<system_color::show_color_menu::HideColorMenuCmd>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >", "bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>", "bevy_ecs::change_detection::Res<country_core::resources::walls::wooden_gates::Gates>", "bevy_ecs::system::query::Query<ref$<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh> >,bevy_ecs::query::filter::With<country_core::components::WoodenGate> >", "bevy_ecs::system::query::Query<ref$<country_core::components::WoodenGate>,tuple$<> >", "bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>", "bevy_ecs::event::writer::EventWriter<system_color::show_color_menu::ShowColorMenuCmd>", "bevy_ecs::event::writer::EventWriter<system_color::show_color_menu::HideColorMenuCmd>", "bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::wooden_gates::Gates>
// [候选系统接口] query_with_filter: bevy_ecs::query::filter::With<country_core::components::WoodenGate>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<ref$<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh> >,bevy_ecs::query::filter::With<country_core::components::WoodenGate> >
// [候选系统接口] component_query: bevy_ecs::system::query::Query<ref$<country_core::components::WoodenGate>,tuple$<> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<system_color::show_color_menu::ShowColorMenuCmd>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<system_color::show_color_menu::HideColorMenuCmd>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::event::reader::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::system::query::Query<ref$<system_color::radial_menu::ColorRadialMenu>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::event::writer::EventWriter<system_color::show_color_menu::HideColorMenuCmd>)
// 候选参数类型: ["bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>", "bevy_ecs::event::reader::EventReader<country_core::resources::gui::EscapeMenuJustOpened>", "bevy_ecs::system::query::Query<ref$<system_color::radial_menu::ColorRadialMenu>,tuple$<> >", "bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>", "bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>", "bevy_ecs::event::writer::EventWriter<system_color::show_color_menu::HideColorMenuCmd>"]
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::resources::gui::EscapeMenuJustOpened>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<ref$<system_color::radial_menu::ColorRadialMenu>,tuple$<> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<system_color::show_color_menu::HideColorMenuCmd>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::event::reader::EventReader<system_color::show_color_menu::HideColorMenuCmd>,bevy_ecs::event::writer::EventWriter<enum2$<country_core::systems::ui::resources::widget_params::PlayUiSound> >,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<system_color::radial_menu::ColorRadialMenu> >,tuple$<> >,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::color_icon::CustomizationIcon> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::system::commands::Commands,bevy_ecs::event::writer::EventWriter<system_color::set_roof_customization::SetFlatRoofStyleCmd>,tuple$<bevy_ecs::event::writer::EventWriter<country_core::resources::color_id::SetWallColorIdCmd>,bevy_ecs::event::writer::EventWriter<system_color::set_roof_customization::SetRoofColorIdCmd>,bevy_ecs::event::writer::EventWriter<system_color::set_decorator_color_id::SetDecoratorColorIdCmd>,bevy_ecs::event::writer::EventWriter<system_color::set_decorator_color_id::SetLadderColorIdCmd>,bevy_ecs::event::writer::EventWriter<system_color::set_decorator_color_id::SetDecoratorBrightnessCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::color_id::SetGateColorIdCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::color_id::SetHalftimberColorIdCmd>,bevy_ecs::event::writer::EventWriter<country_core::systems::wall::set_wall_style::SetWallStyleCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::walls::decorator::SetFlagConnectionCmd>,bevy_ecs::event::writer::EventWriter<system_color::set_window_light::SetWindowLightCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::SetFlatRoofRimStyleCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::walls::decorator::SetChimneySmokeCmd>,bevy_ecs::event::writer::EventWriter<system_color::set_decorator_color_id::SetMagicColorIdCmd> >,bevy_ecs::event::writer::EventWriter<country_core::systems::wall::set_wall_style::SetPlasterPeelAmountCmd>)
// 候选参数类型: ["bevy_ecs::event::reader::EventReader<system_color::show_color_menu::HideColorMenuCmd>", "bevy_ecs::event::writer::EventWriter<enum2$<country_core::systems::ui::resources::widget_params::PlayUiSound> >", "bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<system_color::radial_menu::ColorRadialMenu> >,tuple$<> >", "bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::color_icon::CustomizationIcon> >,tuple$<> >", "bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>", "bevy_ecs::system::commands::Commands", "bevy_ecs::event::writer::EventWriter<system_color::set_roof_customization::SetFlatRoofStyleCmd>", "tuple$<bevy_ecs::event::writer::EventWriter<country_core::resources::color_id::SetWallColorIdCmd>,bevy_ecs::event::writer::EventWriter<system_color::set_roof_customization::SetRoofColorIdCmd>,bevy_ecs::event::writer::EventWriter<system_color::set_decorator_color_id::SetDecoratorColorIdCmd>,bevy_ecs::event::writer::EventWriter<system_color::set_decorator_color_id::SetLadderColorIdCmd>,bevy_ecs::event::writer::EventWriter<system_color::set_decorator_color_id::SetDecoratorBrightnessCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::color_id::SetGateColorIdCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::color_id::SetHalftimberColorIdCmd>,bevy_ecs::event::writer::EventWriter<country_core::systems::wall::set_wall_style::SetWallStyleCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::walls::decorator::SetFlagConnectionCmd>,bevy_ecs::event::writer::EventWriter<system_color::set_window_light::SetWindowLightCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::SetFlatRoofRimStyleCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::walls::decorator::SetChimneySmokeCmd>,bevy_ecs::event::writer::EventWriter<system_color::set_decorator_color_id::SetMagicColorIdCmd> >", "bevy_ecs::event::writer::EventWriter<country_core::systems::wall::set_wall_style::SetPlasterPeelAmountCmd>"]
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<system_color::show_color_menu::HideColorMenuCmd>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<enum2$<country_core::systems::ui::resources::widget_params::PlayUiSound> >
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<system_color::radial_menu::ColorRadialMenu> >,tuple$<> >
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::color_icon::CustomizationIcon> >,tuple$<> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<system_color::set_roof_customization::SetFlatRoofStyleCmd>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::resources::color_id::SetWallColorIdCmd>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<system_color::set_roof_customization::SetRoofColorIdCmd>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<system_color::set_decorator_color_id::SetDecoratorColorIdCmd>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<system_color::set_decorator_color_id::SetLadderColorIdCmd>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<system_color::set_decorator_color_id::SetDecoratorBrightnessCmd>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::resources::color_id::SetGateColorIdCmd>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::resources::color_id::SetHalftimberColorIdCmd>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::systems::wall::set_wall_style::SetWallStyleCmd>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::resources::walls::decorator::SetFlagConnectionCmd>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<system_color::set_window_light::SetWindowLightCmd>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::SetFlatRoofRimStyleCmd>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::resources::walls::decorator::SetChimneySmokeCmd>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<system_color::set_decorator_color_id::SetMagicColorIdCmd>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::systems::wall::set_wall_style::SetPlasterPeelAmountCmd>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::color_icon::CustomizationIcon>,ref$<country_core::resources::color_icon::CustomizationIconMarkedForDeletion> >,tuple$<> >,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::color_icon::CustomizationIcon>,ref$<country_core::resources::color_icon::CustomizationIconMarkedForDeletion> >,tuple$<> >", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::color_icon::CustomizationIcon>,ref$<country_core::resources::color_icon::CustomizationIconMarkedForDeletion> >,tuple$<> >
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::event::reader::EventReader<system_color::show_color_menu::ShowColorMenuCmd>,bevy_ecs::change_detection::Res<country_core::resources::color_id::WallColorIds>,bevy_ecs::change_detection::Res<country_core::resources::color_id::GateColorIds>,bevy_ecs::change_detection::Res<country_core::resources::color_id::HalftimberColorIds>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::widget_params::UiNavState>,country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::resources::stairs::visual::StairsVisualState>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_state::StairsState>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::event::reader::EventReader<system_color::show_color_menu::ShowColorMenuCmd>", "bevy_ecs::change_detection::Res<country_core::resources::color_id::WallColorIds>", "bevy_ecs::change_detection::Res<country_core::resources::color_id::GateColorIds>", "bevy_ecs::change_detection::Res<country_core::resources::color_id::HalftimberColorIds>", "bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >", "bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >", "bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>", "bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>", "bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::widget_params::UiNavState>", "country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources", "bevy_ecs::change_detection::Res<country_core::resources::stairs::visual::StairsVisualState>", "bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_state::StairsState>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<system_color::show_color_menu::ShowColorMenuCmd>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::color_id::WallColorIds>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::color_id::GateColorIds>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::color_id::HalftimberColorIds>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::widget_params::UiNavState>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::stairs::visual::StairsVisualState>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_state::StairsState>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::color_icon::CustomizationIcon>,enum2$<core::option::Option<ref$<country_core::resources::color_icon::ColorIcon> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::WallStyleIcon> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::FlagConnectionIcon> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::FreehandHeightTypeToggleIcon> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::FlatRoofRimTypeToggle> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::ColorMenuBrightnessIcon> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::ChimneySmokeToggle> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::FlatRoofStyleIcon> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::PlasterPeelIcon> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::StairRailingIcon> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::MagicColorIcon> > >,ref$<country_core::components::Ui2dExtent>,ref_mut$<country_core::components::Ui2dImage> >,tuple$<> >,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<system_color::radial_menu::ColorRadialMenu> >,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::resources::glade_color_customization::GladeColorCustomization>,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::widget_params::UiNavState>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<tuple$<usize,usize> > > >,tuple$<bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::resources::color_id::WallColorIds>,bevy_ecs::change_detection::Res<country_core::resources::color_id::HalftimberColorIds>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState> >,tuple$<bevy_ecs::event::writer::EventWriter<country_core::resources::color_id::SetWallColorIdCmd>,bevy_ecs::event::writer::EventWriter<system_color::set_roof_customization::SetRoofColorIdCmd>,bevy_ecs::event::writer::EventWriter<system_color::set_decorator_color_id::SetDecoratorColorIdCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::color_id::SetGateColorIdCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::color_id::SetHalftimberColorIdCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::walls::decorator::SetFlagConnectionCmd>,bevy_ecs::event::writer::EventWriter<system_color::set_decorator_color_id::SetDecoratorBrightnessCmd>,bevy_ecs::event::writer::EventWriter<country_core::systems::wall::set_wall_style::SetFreehandHeightTypeCmd>,bevy_ecs::event::writer::EventWriter<system_color::set_window_light::SetWindowLightCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::SetFlatRoofRimStyleCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::glade_border::NotInDemoHint>,bevy_ecs::event::writer::EventWriter<country_core::systems::wall::set_wall_style::SetWallStyleCmd>,bevy_ecs::event::writer::EventWriter<system_color::show_color_menu::HideColorMenuCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::events::LightLanternCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::events::LightWindowCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::history::FinishInputRecordingCmd> >,bevy_ecs::event::writer::EventWriter<country_core::resources::walls::decorator::SetChimneySmokeCmd>,bevy_ecs::event::writer::EventWriter<enum2$<country_core::systems::ui::resources::widget_params::PlayUiSound> >,bevy_ecs::event::writer::EventWriter<system_color::set_roof_customization::SetFlatRoofStyleCmd>,bevy_ecs::event::writer::EventWriter<country_core::systems::wall::set_wall_style::SetPlasterPeelAmountCmd>,bevy_ecs::event::writer::EventWriter<system_color::set_decorator_color_id::SetLadderColorIdCmd>,bevy_ecs::event::writer::EventWriter<system_color::set_decorator_color_id::SetStairRailingCmd>,bevy_ecs::event::writer::EventWriter<system_color::set_decorator_color_id::SetMagicColorIdCmd>)
// 候选参数类型: ["bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::color_icon::CustomizationIcon>,enum2$<core::option::Option<ref$<country_core::resources::color_icon::ColorIcon> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::WallStyleIcon> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::FlagConnectionIcon> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::FreehandHeightTypeToggleIcon> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::FlatRoofRimTypeToggle> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::ColorMenuBrightnessIcon> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::ChimneySmokeToggle> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::FlatRoofStyleIcon> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::PlasterPeelIcon> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::StairRailingIcon> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::MagicColorIcon> > >,ref$<country_core::components::Ui2dExtent>,ref_mut$<country_core::components::Ui2dImage> >,tuple$<> >", "bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<system_color::radial_menu::ColorRadialMenu> >,tuple$<> >", "bevy_ecs::change_detection::ResMut<country_core::resources::glade_color_customization::GladeColorCustomization>", "bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::widget_params::UiNavState>", "bevy_ecs::system::system_param::Local<enum2$<core::option::Option<tuple$<usize,usize> > > >", "tuple$<bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::resources::color_id::WallColorIds>,bevy_ecs::change_detection::Res<country_core::resources::color_id::HalftimberColorIds>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates> >", "tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState> >", "tuple$<bevy_ecs::event::writer::EventWriter<country_core::resources::color_id::SetWallColorIdCmd>,bevy_ecs::event::writer::EventWriter<system_color::set_roof_customization::SetRoofColorIdCmd>,bevy_ecs::event::writer::EventWriter<system_color::set_decorator_color_id::SetDecoratorColorIdCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::color_id::SetGateColorIdCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::color_id::SetHalftimberColorIdCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::walls::decorator::SetFlagConnectionCmd>,bevy_ecs::event::writer::EventWriter<system_color::set_decorator_color_id::SetDecoratorBrightnessCmd>,bevy_ecs::event::writer::EventWriter<country_core::systems::wall::set_wall_style::SetFreehandHeightTypeCmd>,bevy_ecs::event::writer::EventWriter<system_color::set_window_light::SetWindowLightCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::SetFlatRoofRimStyleCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::glade_border::NotInDemoHint>,bevy_ecs::event::writer::EventWriter<country_core::systems::wall::set_wall_style::SetWallStyleCmd>,bevy_ecs::event::writer::EventWriter<system_color::show_color_menu::HideColorMenuCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::events::LightLanternCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::events::LightWindowCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::history::FinishInputRecordingCmd> >", "bevy_ecs::event::writer::EventWriter<country_core::resources::walls::decorator::SetChimneySmokeCmd>", "bevy_ecs::event::writer::EventWriter<enum2$<country_core::systems::ui::resources::widget_params::PlayUiSound> >", "bevy_ecs::event::writer::EventWriter<system_color::set_roof_customization::SetFlatRoofStyleCmd>", "bevy_ecs::event::writer::EventWriter<country_core::systems::wall::set_wall_style::SetPlasterPeelAmountCmd>", "bevy_ecs::event::writer::EventWriter<system_color::set_decorator_color_id::SetLadderColorIdCmd>", "bevy_ecs::event::writer::EventWriter<system_color::set_decorator_color_id::SetStairRailingCmd>", "bevy_ecs::event::writer::EventWriter<system_color::set_decorator_color_id::SetMagicColorIdCmd>"]
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::color_icon::CustomizationIcon>,enum2$<core::option::Option<ref$<country_core::resources::color_icon::ColorIcon> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::WallStyleIcon> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::FlagConnectionIcon> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::FreehandHeightTypeToggleIcon> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::FlatRoofRimTypeToggle> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::ColorMenuBrightnessIcon> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::ChimneySmokeToggle> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::FlatRoofStyleIcon> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::PlasterPeelIcon> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::StairRailingIcon> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::MagicColorIcon> > >,ref$<country_core::components::Ui2dExtent>,ref_mut$<country_core::components::Ui2dImage> >,tuple$<> >
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<system_color::radial_menu::ColorRadialMenu> >,tuple$<> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::glade_color_customization::GladeColorCustomization>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::widget_params::UiNavState>
// [候选系统接口] system_local: bevy_ecs::system::system_param::Local<enum2$<core::option::Option<tuple$<usize,usize> > > >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::color_id::WallColorIds>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::color_id::HalftimberColorIds>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::history::History>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::resources::color_id::SetWallColorIdCmd>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<system_color::set_roof_customization::SetRoofColorIdCmd>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<system_color::set_decorator_color_id::SetDecoratorColorIdCmd>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::resources::color_id::SetGateColorIdCmd>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::resources::color_id::SetHalftimberColorIdCmd>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::resources::walls::decorator::SetFlagConnectionCmd>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<system_color::set_decorator_color_id::SetDecoratorBrightnessCmd>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::systems::wall::set_wall_style::SetFreehandHeightTypeCmd>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<system_color::set_window_light::SetWindowLightCmd>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::SetFlatRoofRimStyleCmd>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::resources::glade_border::NotInDemoHint>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::systems::wall::set_wall_style::SetWallStyleCmd>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<system_color::show_color_menu::HideColorMenuCmd>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::resources::events::LightLanternCmd>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::resources::events::LightWindowCmd>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::resources::history::FinishInputRecordingCmd>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::resources::walls::decorator::SetChimneySmokeCmd>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<enum2$<country_core::systems::ui::resources::widget_params::PlayUiSound> >
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<system_color::set_roof_customization::SetFlatRoofStyleCmd>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::systems::wall::set_wall_style::SetPlasterPeelAmountCmd>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<system_color::set_decorator_color_id::SetLadderColorIdCmd>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<system_color::set_decorator_color_id::SetStairRailingCmd>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<system_color::set_decorator_color_id::SetMagicColorIdCmd>
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: system_color::ui_color_wall_and_roof::color_cursor_icon
// 信息 ID: 08921092a64e34c53b38c311
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 5 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] core::ops::function::FnOnce::call_once<system_color::ui_color_wall_and_roof::ui_color_wall_and_roof::closure_env$0,tuple$<ref$<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,ref$<country_core::asset_libraries::shader_library::AssetShaderLibrary>,ref_mut$<bevy_ecs::system::commands::Commands> > > @ 0xed15c8 -> 0x1c9b510 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::FnOnce::call_once<system_color::ui_color_wall_and_roof::ui_color_wall_and_roof::closure_env$2,tuple$<ref$<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,ref$<country_core::asset_libraries::shader_library::AssetShaderLibrary>,ref_mut$<bevy_ecs::system::commands::Commands> > > | core::ops::function::FnOnce::call_once<system_color::ui_color_gates::ui_color_gates::closure_env$1,tuple$<ref$<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,ref$<country_core::asset_libraries::shader_library::AssetShaderLibrary>,ref_mut$<bevy_ecs::system::commands::Commands> > > @ 0xed16c8 -> 0x1c9b510 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_color::ui_color_wall_and_roof::ui_color_wall_and_roof::closure$0 @ 0xed25e1 -> 0x1c9b510 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_color::ui_color_wall_and_roof::ui_color_wall_and_roof::closure$2 | system_color::ui_color_gates::ui_color_gates::closure$1 @ 0xed26c1 -> 0x1c9b510 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::FnOnce::call_once<system_color::ui_color_decorator::ui_color_windows_and_flags::closure_env$3,tuple$<ref$<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,ref$<country_core::asset_libraries::shader_library::AssetShaderLibrary>,ref_mut$<bevy_ecs::system::commands::Commands> > > | system_color::ui_color_decorator::ui_color_windows_and_flags::closure$3 @ 0x1c9d183 -> 0x1c9b510 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_color::ui_color_wall_and_roof::ui_color_wall_and_roof
// 信息 ID: ccb58269b036cdb5d051ffe7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] fd37735c476a2cf7ed143ed9: void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::event::writer::EventWriter<system_color::show_color_menu::ShowColorMenuCmd>,bevy_ecs::event::writer::EventWriter<system_color::show_color_menu::HideColorMenuCmd>,bevy_ecs::system::query::Query<ref$<system_color::radial_menu::ColorRadialMenu>,tuple$<> >,bevy_ecs::event::writer::EventWriter<country_core::systems::wall::structure_outline::OutlineWallCmd>,bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::schedule_system::impl$1::run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::event::writer::EventWriter<system_color::show_color_menu::ShowColorMenuCmd>,bevy_ecs::event::writer::EventWriter<system_color::show_color_menu::HideColorMenuCmd>,bevy_ecs::system::query::Query<ref$<system_color::radial_menu::ColorRadialMenu>,tuple$<> >,bevy_ecs::event::writer::EventWriter<country_core::systems::wall::structure_outline::OutlineWallCmd>,bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::event::writer::EventWriter<system_color::show_color_menu::ShowColorMenuCmd>,bevy_ecs::event::writer::EventWriter<system_color::show_color_menu::HideColorMenuCmd>,bevy_ecs::system::query::Query<ref$<system_color::radial_menu::ColorRadialMenu>,tuple$<> >,bevy_ecs::event::writer::EventWriter<country_core::systems::wall::structure_outline::OutlineWallCmd>,bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>)> > @ 0xed0e0d -> 0x1c9ab70 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::event::writer::EventWriter<system_color::show_color_menu::ShowColorMenuCmd>,bevy_ecs::event::writer::EventWriter<system_color::show_color_menu::HideColorMenuCmd>,bevy_ecs::system::query::Query<ref$<system_color::radial_menu::ColorRadialMenu>,tuple$<> >,bevy_ecs::event::writer::EventWriter<country_core::systems::wall::structure_outline::OutlineWallCmd>,bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::event::writer::EventWriter<system_color::show_color_menu::ShowColorMenuCmd>,bevy_ecs::event::writer::EventWriter<system_color::show_color_menu::HideColorMenuCmd>,bevy_ecs::system::query::Query<ref$<system_color::radial_menu::ColorRadialMenu>,tuple$<> >,bevy_ecs::event::writer::EventWriter<country_core::systems::wall::structure_outline::OutlineWallCmd>,bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>)> > > @ 0x1c8a735 -> 0x1c9ab70 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_color::ui_color_wall_and_roof::ui_color_wall_and_roof::closure$0
// 信息 ID: dfbea966210c3d1ba2942513
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_color::ui_color_wall_and_roof::ui_color_wall_and_roof", "captures": "unknown"}
// [闭包环境类型引用] system_color::ui_color_wall_and_roof::ui_color_wall_and_roof::closure_env$0 <- core::ops::function::FnOnce::call_once<system_color::ui_color_wall_and_roof::ui_color_wall_and_roof::closure_env$0,tuple$<ref$<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,ref$<country_core::asset_libraries::shader_library::AssetShaderLibrary>,ref_mut$<bevy_ecs::system::commands::Commands> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_color::ui_color_wall_and_roof::ui_color_wall_and_roof::closure_env$0 <- country_core::systems::ui::resources::widget_params::WidgetParams::signifier<system_color::ui_color_wall_and_roof::ui_color_wall_and_roof::closure_env$0>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 2 条。
// [PDB 内联宿主] core::ops::function::FnOnce::call_once<system_color::ui_color_wall_and_roof::ui_color_wall_and_roof::closure_env$0,tuple$<ref$<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,ref$<country_core::asset_libraries::shader_library::AssetShaderLibrary>,ref_mut$<bevy_ecs::system::commands::Commands> > >; module=1061, inline_record=41808; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_color::ui_color_wall_and_roof::ui_color_wall_and_roof::closure$1
// 信息 ID: c7fb9d30a9a4d29fc0662751
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "system_color::ui_color_wall_and_roof::ui_color_wall_and_roof", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_color::ui_color_wall_and_roof::ui_color_wall_and_roof; module=2567, inline_record=4276; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_color::ui_color_wall_and_roof::ui_color_wall_and_roof::closure$2
// 信息 ID: 802aab49b6e17921b58e3aaf
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "system_color::ui_color_wall_and_roof::ui_color_wall_and_roof", "captures": "unknown"}
// [闭包环境类型引用] system_color::ui_color_wall_and_roof::ui_color_wall_and_roof::closure_env$2 <- core::ops::function::FnOnce::call_once<system_color::ui_color_wall_and_roof::ui_color_wall_and_roof::closure_env$2,tuple$<ref$<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,ref$<country_core::asset_libraries::shader_library::AssetShaderLibrary>,ref_mut$<bevy_ecs::system::commands::Commands> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_color::ui_color_wall_and_roof::ui_color_wall_and_roof::closure_env$2 <- country_core::systems::ui::resources::widget_params::WidgetParams::signifier<system_color::ui_color_wall_and_roof::ui_color_wall_and_roof::closure_env$2>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 2 条。
// [PDB 内联宿主] core::ops::function::FnOnce::call_once<system_color::ui_color_wall_and_roof::ui_color_wall_and_roof::closure_env$2,tuple$<ref$<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,ref$<country_core::asset_libraries::shader_library::AssetShaderLibrary>,ref_mut$<bevy_ecs::system::commands::Commands> > >; module=1061, inline_record=42612; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
