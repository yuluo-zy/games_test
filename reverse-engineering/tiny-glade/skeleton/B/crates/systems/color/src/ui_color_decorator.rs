// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/systems/color/src/ui_color_decorator.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: 359f3bf16b52e02938f79b14852214f100d50088b6c340d37fb359261890977b
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: ui_color_decorator.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 2568, 3767
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
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.3"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.5"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.15.2"}]
// use petgraph; // namespace candidate
//   package identities: [{"name":"petgraph","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.8.1"}]
// use smallvec; // namespace candidate
//   package identities: [{"name":"smallvec","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.13.2"}]
// use std; // namespace candidate
//   package identities: []
// use system_color; // namespace candidate
//   package identities: [{"name":"system-color","source":"local_or_path","version":"0.1.0"}]
// use utils; // namespace candidate
//   package identities: [{"name":"utils","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_color::ui_color_decorator::ui_color_windows_and_flags
// 记录形式: compiled_procedure; instances=1
// 源码行: 26
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3767,"record_offset":1088,"rva":36376064,"va":5405085184}
// 编译实例 RVAs: 0x22b0e00
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: ui_color_decorator.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-30006250","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405085184,"raw_hex":"e8162436fe","size":5,"soft_fail":false,"target_va":5375081504,"va":5405087749,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140895470
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ui_color_decorator.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-27378389","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405085184,"raw_hex":"e82b3d5efe","size":5,"soft_fail":false,"target_va":5377709168,"va":5405087552,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140901260
//     owner=unresolved_namespace; count=2
//     target_aliases=5 candidates; complete list: ui_color_decorator.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-26935145","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405085184,"raw_hex":"e8970065fe","size":5,"soft_fail":false,"target_va":5378151008,"va":5405086148,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140959e70
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: ui_color_decorator.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-26570989","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405085184,"raw_hex":"e8138f6afe","size":5,"soft_fail":false,"target_va":5378514544,"va":5405085528,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b26a70
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ui_color_decorator.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-24684680","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405085184,"raw_hex":"e8785787fe","size":5,"soft_fail":false,"target_va":5380401776,"va":5405086451,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1422bcec0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ui_color_decorator.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t47591","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405085184,"raw_hex":"e8e7b90000","size":5,"soft_fail":false,"target_va":5405134528,"va":5405086932,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1422bd120
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ui_color_decorator.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t48173","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405085184,"raw_hex":"e82dbc0000","size":5,"soft_fail":false,"target_va":5405135136,"va":5405086958,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1425f4610
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: ui_color_decorator.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3421832","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405085184,"raw_hex":"e888363400","size":5,"soft_fail":false,"target_va":5408507408,"va":5405085571,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1425f4950
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ui_color_decorator.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3422399","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405085184,"raw_hex":"e8bf383400","size":5,"soft_fail":false,"target_va":5408508240,"va":5405085836,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: ui_color_decorator.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t6760415","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405085184,"raw_hex":"e8df276700","size":5,"soft_fail":false,"target_va":5411847168,"va":5405086748,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-22246409","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405085184,"raw_hex":"e8f78bacfe","size":5,"soft_fail":false,"target_va":5382840208,"va":5405086612,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::event::writer::EventWriter$LT$E$GT$::write
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-132205","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405085184,"raw_hex":"e893fbfdff","size":5,"soft_fail":false,"target_va":5404954880,"va":5405087080,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t6454588","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405085184,"raw_hex":"e83c7d6200","size":5,"soft_fail":false,"target_va":5411542064,"va":5405087471,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::stairs::stairs_state::StairsState::graph
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-26945484","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405085184,"raw_hex":"e834d864fe","size":5,"soft_fail":false,"target_va":5378140336,"va":5405085815,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms::transform
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-25783171","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405085184,"raw_hex":"e87d9476fe","size":5,"soft_fail":false,"target_va":5379303968,"va":5405087134,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator_outline::DecoratorOutline::display
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-24632109","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405085184,"raw_hex":"e8d32488fe","size":5,"soft_fail":false,"target_va":5380455168,"va":5405087272,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator_storage::DecoratorStorage::decorator_info
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-25774692","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405085184,"raw_hex":"e89cb576fe","size":5,"soft_fail":false,"target_va":5379311088,"va":5405085775,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator_storage::DecoratorStorage::lookup_decorator_info
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-25773554","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405085184,"raw_hex":"e80eba76fe","size":5,"soft_fail":false,"target_va":5379311920,"va":5405085469,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator_storage::DecoratorStorage::lookup_storage
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-25774946","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405085184,"raw_hex":"e89eb476fe","size":5,"soft_fail":false,"target_va":5379310560,"va":5405085501,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator_storage::DecoratorStorageAddr::as_wall
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-25777060","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405085184,"raw_hex":"e85cac76fe","size":5,"soft_fail":false,"target_va":5379310144,"va":5405087199,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary::get_outline
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-24686388","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405085184,"raw_hex":"e8cc5087fe","size":5,"soft_fail":false,"target_va":5380400784,"va":5405087167,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo::subtype
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-24684964","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405085184,"raw_hex":"e85c5687fe","size":5,"soft_fail":false,"target_va":5380401520,"va":5405086479,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_walls::PublicWalls::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-25294598","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405085184,"raw_hex":"e8fa087efe","size":5,"soft_fail":false,"target_va":5379792624,"va":5405087217,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::collision::world::RaycastWorld::legacy_raycast_w_params
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-27150108","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405085184,"raw_hex":"e8e4b861fe","size":5,"soft_fail":false,"target_va":5377935216,"va":5405085319,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::resources::ui_state::UiWidgetContext::clicked
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-26751052","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405085184,"raw_hex":"e8b4cf67fe","size":5,"soft_fail":false,"target_va":5378336240,"va":5405087287,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::resources::ui_state::UiWidgetContext::is_active
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-26750614","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405085184,"raw_hex":"e86ad167fe","size":5,"soft_fail":false,"target_va":5378336384,"va":5405086993,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::resources::widget_params::UiSounds::empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-26248324","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405085184,"raw_hex":"e87c7b6ffe","size":5,"soft_fail":false,"target_va":5378838448,"va":5405086767,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::resources::widget_params::UiSounds::on_click
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-26248304","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405085184,"raw_hex":"e8907b6ffe","size":5,"soft_fail":false,"target_va":5378838560,"va":5405086859,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::resources::widget_params::UiSounds::on_hover_start
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-26248337","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405085184,"raw_hex":"e86f7b6ffe","size":5,"soft_fail":false,"target_va":5378838480,"va":5405086812,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::resources::widget_params::WidgetParams::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-26248692","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405085184,"raw_hex":"e80c7a6ffe","size":5,"soft_fail":false,"target_va":5378838032,"va":5405086719,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::resources::widget_params::WidgetParams::sounds
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-26248747","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405085184,"raw_hex":"e8d5796ffe","size":5,"soft_fail":false,"target_va":5378838144,"va":5405086886,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::components::transform::Transform::compute_affine
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-28364799","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405085184,"raw_hex":"e801304ffe","size":5,"soft_fail":false,"target_va":5376722352,"va":5405087146,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":3767,"record_offset":4016,"parent_record_offset":1088,"ancestor_record_offset":1088,"inlinee":7051,"source_line":644,"archive":"_pdb/B/modules/03767.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":3767,"record_offset":1276,"parent_record_offset":1088,"ancestor_record_offset":1088,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/03767.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::map
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3767,"record_offset":3688,"parent_record_offset":1088,"ancestor_record_offset":1088,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/03767.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":3767,"record_offset":4500,"parent_record_offset":1088,"ancestor_record_offset":1088,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/03767.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::last
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":3767,"record_offset":3656,"parent_record_offset":1088,"ancestor_record_offset":1088,"inlinee":4716,"source_line":279,"archive":"_pdb/B/modules/03767.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":3767,"record_offset":3960,"parent_record_offset":1088,"ancestor_record_offset":1088,"inlinee":24514,"source_line":273,"archive":"_pdb/B/modules/03767.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui::resources::widget_params::WidgetParams::with_flags
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/resources/widget_params.rs
//     evidence={"module":3767,"record_offset":4324,"parent_record_offset":1088,"ancestor_record_offset":1088,"inlinee":24901,"source_line":117,"archive":"_pdb/B/modules/03767.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::distance
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":3767,"record_offset":4100,"parent_record_offset":1088,"ancestor_record_offset":1088,"inlinee":13631,"source_line":477,"archive":"_pdb/B/modules/03767.jsonl.gz"}
//   [pdb_inline_direct] petgraph::graphmap::GraphMap::neighbors
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"petgraph","version":"0.8.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/petgraph-0.8.1/src/graphmap.rs
//     evidence={"module":3767,"record_offset":2320,"parent_record_offset":1088,"ancestor_record_offset":1088,"inlinee":4972,"source_line":454,"archive":"_pdb/B/modules/03767.jsonl.gz"}
//   [pdb_inline_direct] petgraph::graphmap::impl$9::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"petgraph","version":"0.8.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/petgraph-0.8.1/src/graphmap.rs
//     evidence={"module":3767,"record_offset":2408,"parent_record_offset":1088,"ancestor_record_offset":1088,"inlinee":10600,"source_line":719,"archive":"_pdb/B/modules/03767.jsonl.gz"}
//   [pdb_inline_direct] smallvec::impl$17::deref
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"smallvec","version":"1.13.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.13.2/src/lib.rs
//     evidence={"module":3767,"record_offset":3572,"parent_record_offset":1088,"ancestor_record_offset":1088,"inlinee":4601,"source_line":1846,"archive":"_pdb/B/modules/03767.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":3767,"record_offset":1304,"parent_record_offset":1088,"ancestor_record_offset":1088,"inlinee":4718,"source_line":894,"archive":"_pdb/B/modules/03767.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":3767,"record_offset":4676,"parent_record_offset":4652,"ancestor_record_offset":1088,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/03767.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":3767,"record_offset":4652,"parent_record_offset":4596,"ancestor_record_offset":1088,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/03767.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":3767,"record_offset":4064,"parent_record_offset":4040,"ancestor_record_offset":1088,"inlinee":7053,"source_line":654,"archive":"_pdb/B/modules/03767.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3767,"record_offset":4624,"parent_record_offset":4596,"ancestor_record_offset":1088,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/03767.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3767,"record_offset":4596,"parent_record_offset":4572,"ancestor_record_offset":1088,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/03767.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3767,"record_offset":4572,"parent_record_offset":4548,"ancestor_record_offset":1088,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/03767.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3767,"record_offset":2372,"parent_record_offset":2348,"ancestor_record_offset":1088,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/03767.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3767,"record_offset":2348,"parent_record_offset":2320,"ancestor_record_offset":1088,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/03767.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":3767,"record_offset":2076,"parent_record_offset":2052,"ancestor_record_offset":1088,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/03767.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":3767,"record_offset":3984,"parent_record_offset":3960,"ancestor_record_offset":1088,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/03767.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":3767,"record_offset":1644,"parent_record_offset":1620,"ancestor_record_offset":1088,"inlinee":4484,"source_line":1227,"archive":"_pdb/B/modules/03767.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":3767,"record_offset":1728,"parent_record_offset":1704,"ancestor_record_offset":1088,"inlinee":4486,"source_line":1433,"archive":"_pdb/B/modules/03767.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":3767,"record_offset":1668,"parent_record_offset":1644,"ancestor_record_offset":1088,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/03767.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":3767,"record_offset":2144,"parent_record_offset":1508,"ancestor_record_offset":1088,"inlinee":4502,"source_line":1497,"archive":"_pdb/B/modules/03767.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":3767,"record_offset":1876,"parent_record_offset":1848,"ancestor_record_offset":1088,"inlinee":4503,"source_line":449,"archive":"_pdb/B/modules/03767.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":3767,"record_offset":1848,"parent_record_offset":1820,"ancestor_record_offset":1088,"inlinee":4493,"source_line":601,"archive":"_pdb/B/modules/03767.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":3767,"record_offset":4168,"parent_record_offset":4132,"ancestor_record_offset":1088,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/03767.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3767,"record_offset":2512,"parent_record_offset":2408,"ancestor_record_offset":1088,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/03767.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3767,"record_offset":4040,"parent_record_offset":4016,"ancestor_record_offset":1088,"inlinee":6345,"source_line":1218,"archive":"_pdb/B/modules/03767.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":3767,"record_offset":4524,"parent_record_offset":4500,"ancestor_record_offset":1088,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/03767.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3767,"record_offset":1992,"parent_record_offset":1968,"ancestor_record_offset":1088,"inlinee":4723,"source_line":1129,"archive":"_pdb/B/modules/03767.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":3767,"record_offset":2476,"parent_record_offset":2440,"ancestor_record_offset":1088,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/03767.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":3767,"record_offset":2440,"parent_record_offset":2408,"ancestor_record_offset":1088,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/03767.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::stairs::stairs_assembly_params::impl$21::eq
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/stairs/stairs_assembly_params.rs
//     evidence={"module":3767,"record_offset":2100,"parent_record_offset":2076,"ancestor_record_offset":1088,"inlinee":4725,"source_line":120,"archive":"_pdb/B/modules/03767.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::public_wall_state::PublicWallState::is_premade_shape
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_wall_state.rs
//     evidence={"module":3767,"record_offset":4464,"parent_record_offset":4440,"ancestor_record_offset":1088,"inlinee":24411,"source_line":113,"archive":"_pdb/B/modules/03767.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::ui::resources::widget_params::WidgetFlags::insert
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitflags","version":"1.3.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-1.3.2/src/lib.rs
//     evidence={"module":3767,"record_offset":4376,"parent_record_offset":4352,"ancestor_record_offset":1088,"inlinee":24904,"source_line":609,"archive":"_pdb/B/modules/03767.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":3767,"record_offset":4252,"parent_record_offset":4132,"ancestor_record_offset":1088,"inlinee":4895,"source_line":203,"archive":"_pdb/B/modules/03767.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":3767,"record_offset":4132,"parent_record_offset":4100,"ancestor_record_offset":1088,"inlinee":4743,"source_line":1473,"archive":"_pdb/B/modules/03767.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":3767,"record_offset":1788,"parent_record_offset":1760,"ancestor_record_offset":1088,"inlinee":4490,"source_line":49,"archive":"_pdb/B/modules/03767.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":3767,"record_offset":1820,"parent_record_offset":1788,"ancestor_record_offset":1088,"inlinee":4491,"source_line":74,"archive":"_pdb/B/modules/03767.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":3767,"record_offset":1760,"parent_record_offset":1508,"ancestor_record_offset":1088,"inlinee":4488,"source_line":112,"archive":"_pdb/B/modules/03767.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":3767,"record_offset":1620,"parent_record_offset":1508,"ancestor_record_offset":1088,"inlinee":4482,"source_line":51,"archive":"_pdb/B/modules/03767.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":3767,"record_offset":2172,"parent_record_offset":1508,"ancestor_record_offset":1088,"inlinee":4504,"source_line":94,"archive":"_pdb/B/modules/03767.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":3767,"record_offset":1704,"parent_record_offset":1508,"ancestor_record_offset":1088,"inlinee":4485,"source_line":76,"archive":"_pdb/B/modules/03767.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\tag.rs
//     evidence={"module":3767,"record_offset":1592,"parent_record_offset":1508,"ancestor_record_offset":1088,"inlinee":4517,"source_line":36,"archive":"_pdb/B/modules/03767.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\lib.rs
//     evidence={"module":3767,"record_offset":2052,"parent_record_offset":2028,"ancestor_record_offset":1088,"inlinee":4499,"source_line":164,"archive":"_pdb/B/modules/03767.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":3767,"record_offset":1340,"parent_record_offset":1304,"ancestor_record_offset":1088,"inlinee":4719,"source_line":1304,"archive":"_pdb/B/modules/03767.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":3767,"record_offset":1376,"parent_record_offset":1340,"ancestor_record_offset":1088,"inlinee":4475,"source_line":1347,"archive":"_pdb/B/modules/03767.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":3767,"record_offset":2028,"parent_record_offset":1920,"ancestor_record_offset":1088,"inlinee":4497,"source_line":224,"archive":"_pdb/B/modules/03767.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":3767,"record_offset":1416,"parent_record_offset":1376,"ancestor_record_offset":1088,"inlinee":4514,"source_line":253,"archive":"_pdb/B/modules/03767.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":3767,"record_offset":1968,"parent_record_offset":1944,"ancestor_record_offset":1088,"inlinee":4722,"source_line":384,"archive":"_pdb/B/modules/03767.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":3767,"record_offset":1944,"parent_record_offset":1920,"ancestor_record_offset":1088,"inlinee":4721,"source_line":522,"archive":"_pdb/B/modules/03767.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":3767,"record_offset":2264,"parent_record_offset":1508,"ancestor_record_offset":1088,"inlinee":4506,"source_line":81,"archive":"_pdb/B/modules/03767.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":3767,"record_offset":1476,"parent_record_offset":1444,"ancestor_record_offset":1088,"inlinee":4478,"source_line":1156,"archive":"_pdb/B/modules/03767.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":3767,"record_offset":1444,"parent_record_offset":1376,"ancestor_record_offset":1088,"inlinee":4477,"source_line":1179,"archive":"_pdb/B/modules/03767.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":3767,"record_offset":1508,"parent_record_offset":1476,"ancestor_record_offset":1088,"inlinee":4480,"source_line":1862,"archive":"_pdb/B/modules/03767.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":3767,"record_offset":1564,"parent_record_offset":1508,"ancestor_record_offset":1088,"inlinee":4515,"source_line":2301,"archive":"_pdb/B/modules/03767.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":3767,"record_offset":1920,"parent_record_offset":1508,"ancestor_record_offset":1088,"inlinee":4495,"source_line":1166,"archive":"_pdb/B/modules/03767.jsonl.gz"}
//   [pdb_inline_transitive] petgraph::graphmap::impl$9::next::closure$1
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"petgraph","version":"0.8.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/petgraph-0.8.1/src/graphmap.rs
//     evidence={"module":3767,"record_offset":2536,"parent_record_offset":2512,"ancestor_record_offset":1088,"inlinee":10594,"source_line":725,"archive":"_pdb/B/modules/03767.jsonl.gz"}
//   [pdb_inline_transitive] smallvec::SmallVec::spilled
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"smallvec","version":"1.13.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.13.2/src/lib.rs
//     evidence={"module":3767,"record_offset":3620,"parent_record_offset":3596,"ancestor_record_offset":1088,"inlinee":4604,"source_line":1001,"archive":"_pdb/B/modules/03767.jsonl.gz"}
//   [pdb_inline_transitive] smallvec::SmallVec::triple
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"smallvec","version":"1.13.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.13.2/src/lib.rs
//     evidence={"module":3767,"record_offset":3596,"parent_record_offset":3572,"ancestor_record_offset":1088,"inlinee":4603,"source_line":971,"archive":"_pdb/B/modules/03767.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":3767,"record_offset":4284,"parent_record_offset":4252,"ancestor_record_offset":1088,"inlinee":4750,"source_line":369,"archive":"_pdb/B/modules/03767.jsonl.gz"}
//   [pdb_inline_transitive] system_color::ui_color_decorator::ui_color_windows_and_flags::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-color","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/color/src/ui_color_decorator.rs
//     evidence={"module":3767,"record_offset":3724,"parent_record_offset":3688,"ancestor_record_offset":1088,"inlinee":121560,"source_line":87,"archive":"_pdb/B/modules/03767.jsonl.gz"}
//   [pdb_inline_transitive] system_color::ui_color_decorator::ui_color_windows_and_flags::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-color","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/color/src/ui_color_decorator.rs
//     evidence={"module":3767,"record_offset":3928,"parent_record_offset":3896,"ancestor_record_offset":1088,"inlinee":121561,"source_line":110,"archive":"_pdb/B/modules/03767.jsonl.gz"}
//   [pdb_inline_transitive] system_color::ui_color_decorator::ui_color_windows_and_flags::closure$2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-color","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/color/src/ui_color_decorator.rs
//     evidence={"module":3767,"record_offset":4352,"parent_record_offset":4324,"ancestor_record_offset":1088,"inlinee":121562,"source_line":134,"archive":"_pdb/B/modules/03767.jsonl.gz"}
//   [pdb_inline_transitive] system_color::ui_color_decorator::ui_color_windows_and_flags::closure$4
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-color","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/color/src/ui_color_decorator.rs
//     evidence={"module":3767,"record_offset":4440,"parent_record_offset":4412,"ancestor_record_offset":1088,"inlinee":121563,"source_line":180,"archive":"_pdb/B/modules/03767.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_color::ui_color_decorator::ui_color_windows_and_flags::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 87
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1DAD8
// 首条位置: {"module":3767,"inlinee":121560}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_color::ui_color_decorator::ui_color_windows_and_flags::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 110
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1DAD9
// 首条位置: {"module":3767,"inlinee":121561}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_color::ui_color_decorator::ui_color_windows_and_flags::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 134
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1DADA
// 首条位置: {"module":3767,"inlinee":121562}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::systems::ui::resources::widget_params::WidgetFlags::insert
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitflags","version":"1.3.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-1.3.2/src/lib.rs
//     evidence={"module":3767,"record_offset":4376,"parent_record_offset":4352,"ancestor_record_offset":4352,"inlinee":24904,"source_line":609,"archive":"_pdb/B/modules/03767.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: core::ops::function::FnOnce::call_once<system_color::ui_color_decorator::ui_color_windows_and_flags::closure_env$3,tuple$<ref$<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,ref$<country_core::asset_libraries::shader_library::AssetShaderLibrary>,ref_mut$<bevy_ecs::system::commands::Commands> > >
// 记录形式: compiled_procedure; instances=1
// 源码行: 145
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":2568,"record_offset":73216,"rva":30003488,"va":5398712608}
// 编译实例 RVAs: 0x1c9d120
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: ui_color_decorator.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-23631177","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398712608,"raw_hex":"e8b76a97fe","size":5,"soft_fail":false,"target_va":5375081488,"va":5398712660,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: ui_color_decorator.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-23631303","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398712608,"raw_hex":"e8396a97fe","size":5,"soft_fail":false,"target_va":5375081504,"va":5398712802,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t12828225","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398712608,"raw_hex":"e841bec300","size":5,"soft_fail":false,"target_va":5411540992,"va":5398712762,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_color::ui_color_wall_and_roof::color_cursor_icon
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"system-color","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-7288","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398712608,"raw_hex":"e888e3ffff","size":5,"soft_fail":false,"target_va":5398705424,"va":5398712707,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":2568,"record_offset":73612,"parent_record_offset":73216,"ancestor_record_offset":73216,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/02568.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":2568,"record_offset":73728,"parent_record_offset":73704,"ancestor_record_offset":73216,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/02568.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":2568,"record_offset":73752,"parent_record_offset":73728,"ancestor_record_offset":73216,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/02568.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":2568,"record_offset":73976,"parent_record_offset":73952,"ancestor_record_offset":73216,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/02568.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":2568,"record_offset":73676,"parent_record_offset":73636,"ancestor_record_offset":73216,"inlinee":4281,"source_line":347,"archive":"_pdb/B/modules/02568.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":2568,"record_offset":73704,"parent_record_offset":73676,"ancestor_record_offset":73216,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/02568.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":2568,"record_offset":73952,"parent_record_offset":73928,"ancestor_record_offset":73216,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/02568.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":2568,"record_offset":73928,"parent_record_offset":73904,"ancestor_record_offset":73216,"inlinee":4364,"source_line":1676,"archive":"_pdb/B/modules/02568.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":2568,"record_offset":73844,"parent_record_offset":73820,"ancestor_record_offset":73216,"inlinee":7066,"source_line":99,"archive":"_pdb/B/modules/02568.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":2568,"record_offset":73820,"parent_record_offset":73636,"ancestor_record_offset":73216,"inlinee":7065,"source_line":526,"archive":"_pdb/B/modules/02568.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::from_raw_parts_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":2568,"record_offset":73868,"parent_record_offset":73844,"ancestor_record_offset":73216,"inlinee":7067,"source_line":934,"archive":"_pdb/B/modules/02568.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":2568,"record_offset":73904,"parent_record_offset":73636,"ancestor_record_offset":73216,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/02568.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":2568,"record_offset":73776,"parent_record_offset":73752,"ancestor_record_offset":73216,"inlinee":4282,"source_line":1740,"archive":"_pdb/B/modules/02568.jsonl.gz"}
//   [pdb_inline_transitive] system_color::ui_color_decorator::ui_color_windows_and_flags::closure$3
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-color","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/color/src/ui_color_decorator.rs
//     evidence={"module":2568,"record_offset":73636,"parent_record_offset":73612,"ancestor_record_offset":73216,"inlinee":97824,"source_line":145,"archive":"_pdb/B/modules/02568.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_color::ui_color_decorator::ui_color_windows_and_flags::closure$3
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=3
// 源码行: 145
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x17E20
// 首条位置: {"module":2568,"inlinee":97824}
// 编译实例 RVAs: 0x1c9d120
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: ui_color_decorator.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-23631177","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398712608,"raw_hex":"e8b76a97fe","size":5,"soft_fail":false,"target_va":5375081488,"va":5398712660,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: ui_color_decorator.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-23631303","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398712608,"raw_hex":"e8396a97fe","size":5,"soft_fail":false,"target_va":5375081504,"va":5398712802,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t12828225","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398712608,"raw_hex":"e841bec300","size":5,"soft_fail":false,"target_va":5411540992,"va":5398712762,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_color::ui_color_wall_and_roof::color_cursor_icon
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"system-color","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-7288","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398712608,"raw_hex":"e888e3ffff","size":5,"soft_fail":false,"target_va":5398705424,"va":5398712707,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":2568,"record_offset":73676,"parent_record_offset":73636,"ancestor_record_offset":73636,"inlinee":4281,"source_line":347,"archive":"_pdb/B/modules/02568.jsonl.gz"}
//   [pdb_inline_direct] alloc::slice::impl$0::into_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":2568,"record_offset":73820,"parent_record_offset":73636,"ancestor_record_offset":73636,"inlinee":7065,"source_line":526,"archive":"_pdb/B/modules/02568.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":2568,"record_offset":73904,"parent_record_offset":73636,"ancestor_record_offset":73636,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/02568.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":2568,"record_offset":73728,"parent_record_offset":73704,"ancestor_record_offset":73636,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/02568.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":2568,"record_offset":73752,"parent_record_offset":73728,"ancestor_record_offset":73636,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/02568.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":2568,"record_offset":73976,"parent_record_offset":73952,"ancestor_record_offset":73636,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/02568.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":2568,"record_offset":73704,"parent_record_offset":73676,"ancestor_record_offset":73636,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/02568.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":2568,"record_offset":73952,"parent_record_offset":73928,"ancestor_record_offset":73636,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/02568.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":2568,"record_offset":73928,"parent_record_offset":73904,"ancestor_record_offset":73636,"inlinee":4364,"source_line":1676,"archive":"_pdb/B/modules/02568.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::into_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":2568,"record_offset":73844,"parent_record_offset":73820,"ancestor_record_offset":73636,"inlinee":7066,"source_line":99,"archive":"_pdb/B/modules/02568.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::from_raw_parts_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":2568,"record_offset":73868,"parent_record_offset":73844,"ancestor_record_offset":73636,"inlinee":7067,"source_line":934,"archive":"_pdb/B/modules/02568.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":2568,"record_offset":73776,"parent_record_offset":73752,"ancestor_record_offset":73636,"inlinee":4282,"source_line":1740,"archive":"_pdb/B/modules/02568.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_color::ui_color_decorator::ui_color_windows_and_flags::closure$4
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 180
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1DADB
// 首条位置: {"module":3767,"inlinee":121563}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::resources::walls::public_wall_state::PublicWallState::is_premade_shape
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_wall_state.rs
//     evidence={"module":3767,"record_offset":4464,"parent_record_offset":4440,"ancestor_record_offset":4440,"inlinee":24411,"source_line":113,"archive":"_pdb/B/modules/03767.jsonl.gz"}
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
// 完整结构及原始类型记录: ui_color_decorator.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: ui_color_decorator.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: ui_color_decorator.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

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

// ------------------------------------------------------------------
// 函数: core::ops::function::FnOnce::call_once<system_color::ui_color_decorator::ui_color_windows_and_flags::closure_env$3,tuple$<ref$<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,ref$<country_core::asset_libraries::shader_library::AssetShaderLibrary>,ref_mut$<bevy_ecs::system::commands::Commands> > >
// 信息 ID: e50dc82e8c35d95048c7b382
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [系统接口类型拼写] deferred_command_interface: bevy_ecs::system::commands::Commands
// [类型关系] ref$ -> ["country_core::asset_libraries::mesh_library::AssetMeshLibrary"]; 字段布局 = unknown。
// [类型关系] ref$ -> ["country_core::asset_libraries::shader_library::AssetShaderLibrary"]; 字段布局 = unknown。
// [类型关系] ref_mut$ -> ["bevy_ecs::system::commands::Commands"]; 字段布局 = unknown。
// [类型关系] tuple$ -> ["ref$<country_core::asset_libraries::mesh_library::AssetMeshLibrary>", "ref$<country_core::asset_libraries::shader_library::AssetShaderLibrary>", "ref_mut$<bevy_ecs::system::commands::Commands>"]; 字段布局 = unknown。
// [类型关系] core::ops::function::FnOnce::call_once -> ["system_color::ui_color_decorator::ui_color_windows_and_flags::closure_env$3", "tuple$<ref$<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,ref$<country_core::asset_libraries::shader_library::AssetShaderLibrary>,ref_mut$<bevy_ecs::system::commands::Commands> >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_color::ui_color_decorator::ui_color_windows_and_flags
// 信息 ID: c2ba537044eda373976e42af
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] f958a701b6fba0c3c4e4107e: void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::writer::EventWriter<system_color::show_color_menu::ShowColorMenuCmd>,bevy_ecs::event::writer::EventWriter<system_color::show_color_menu::HideColorMenuCmd>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>,bevy_ecs::event::writer::EventWriter<country_core::systems::wall::structure_outline::OutlineStairsCmd>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyRequests>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_state::StairsState>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::writer::EventWriter<system_color::show_color_menu::ShowColorMenuCmd>,bevy_ecs::event::writer::EventWriter<system_color::show_color_menu::HideColorMenuCmd>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>,bevy_ecs::event::writer::EventWriter<country_core::systems::wall::structure_outline::OutlineStairsCmd>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyRequests>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_state::StairsState>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::writer::EventWriter<system_color::show_color_menu::ShowColorMenuCmd>,bevy_ecs::event::writer::EventWriter<system_color::show_color_menu::HideColorMenuCmd>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>,bevy_ecs::event::writer::EventWriter<country_core::systems::wall::structure_outline::OutlineStairsCmd>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyRequests>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_state::StairsState>)> > > @ 0x1c90186 -> 0x22b0e00 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::schedule_system::impl$1::run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::writer::EventWriter<system_color::show_color_menu::ShowColorMenuCmd>,bevy_ecs::event::writer::EventWriter<system_color::show_color_menu::HideColorMenuCmd>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>,bevy_ecs::event::writer::EventWriter<country_core::systems::wall::structure_outline::OutlineStairsCmd>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyRequests>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_state::StairsState>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::writer::EventWriter<system_color::show_color_menu::ShowColorMenuCmd>,bevy_ecs::event::writer::EventWriter<system_color::show_color_menu::HideColorMenuCmd>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>,bevy_ecs::event::writer::EventWriter<country_core::systems::wall::structure_outline::OutlineStairsCmd>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyRequests>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_state::StairsState>)> > @ 0x1c9c2ec -> 0x22b0e00 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_color::ui_color_decorator::ui_color_windows_and_flags::closure$0
// 信息 ID: 0a9f6f7cf77aec219fc3f0fb
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_color::ui_color_decorator::ui_color_windows_and_flags", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_color::ui_color_decorator::ui_color_windows_and_flags; module=3767, inline_record=3724; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_color::ui_color_decorator::ui_color_windows_and_flags::closure$1
// 信息 ID: aa853eec51414aec30be8c38
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "system_color::ui_color_decorator::ui_color_windows_and_flags", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_color::ui_color_decorator::ui_color_windows_and_flags; module=3767, inline_record=3928; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_color::ui_color_decorator::ui_color_windows_and_flags::closure$2
// 信息 ID: 9a823bce46d48e34052d594c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "system_color::ui_color_decorator::ui_color_windows_and_flags", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_color::ui_color_decorator::ui_color_windows_and_flags; module=3767, inline_record=4352; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_color::ui_color_decorator::ui_color_windows_and_flags::closure$3
// 信息 ID: 966a9519917e10ac988dfc21
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 3, "lexical_owner": "system_color::ui_color_decorator::ui_color_windows_and_flags", "captures": "unknown"}
// [闭包环境类型引用] system_color::ui_color_decorator::ui_color_windows_and_flags::closure_env$3 <- core::ops::function::FnOnce::call_once<system_color::ui_color_decorator::ui_color_windows_and_flags::closure_env$3,tuple$<ref$<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,ref$<country_core::asset_libraries::shader_library::AssetShaderLibrary>,ref_mut$<bevy_ecs::system::commands::Commands> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_color::ui_color_decorator::ui_color_windows_and_flags::closure_env$3 <- country_core::systems::ui::resources::widget_params::WidgetParams::signifier<system_color::ui_color_decorator::ui_color_windows_and_flags::closure_env$3>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 2 条。
// [PDB 内联宿主] core::ops::function::FnOnce::call_once<system_color::ui_color_decorator::ui_color_windows_and_flags::closure_env$3,tuple$<ref$<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,ref$<country_core::asset_libraries::shader_library::AssetShaderLibrary>,ref_mut$<bevy_ecs::system::commands::Commands> > >; module=2568, inline_record=73636; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_color::ui_color_decorator::ui_color_windows_and_flags::closure$4
// 信息 ID: 1efe3a9199d034c7d2673060
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 4, "lexical_owner": "system_color::ui_color_decorator::ui_color_windows_and_flags", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_color::ui_color_decorator::ui_color_windows_and_flags; module=3767, inline_record=4440; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
