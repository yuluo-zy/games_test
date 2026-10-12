// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/systems/color/src/ui_color_decorator.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: e3aa0953f6f9307b822819616ac75f602e1c237033e644ef395a8571f9eceb37
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: ui_color_decorator.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 2707, 4027
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use bevy_ecs; // namespace candidate
//   package identities: [{"name":"bevy_ecs","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.3"}]
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"bitflags","source":"PDB_registry_path_only","version":"1.3.2"},{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use equivalent; // namespace candidate
//   package identities: [{"name":"equivalent","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.1"}]
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.30.10"}]
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.3"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.5"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.15.2"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.1"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.1"}]
// use indexmap; // namespace candidate
//   package identities: [{"name":"indexmap","source":"registry+https://github.com/rust-lang/crates.io-index","version":"2.13.0"}]
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
// 首条位置: {"module":4027,"record_offset":408,"rva":42664160,"va":5411373280}
// 编译实例 RVAs: 0x28b00e0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-35697578","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411373280,"raw_hex":"e8564cdffd","size":5,"soft_fail":false,"target_va":5375678704,"va":5411376277,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1409a3e90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ui_color_decorator.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-32558405","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411373280,"raw_hex":"e8bb320ffe","size":5,"soft_fail":false,"target_va":5378817680,"va":5411376080,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a577d0
//     owner=unresolved_namespace; count=2
//     target_aliases=3 candidates; complete list: ui_color_decorator.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-31821648","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411373280,"raw_hex":"e8b0701afe","size":5,"soft_fail":false,"target_va":5379553232,"va":5411374875,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c5a060
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ui_color_decorator.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-29713575","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411373280,"raw_hex":"e8599b3afe","size":5,"soft_fail":false,"target_va":5381660768,"va":5411374338,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c663b0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: ui_color_decorator.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-29663246","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411373280,"raw_hex":"e8f25f3bfe","size":5,"soft_fail":false,"target_va":5381710768,"va":5411374009,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140fbb6a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ui_color_decorator.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-26169539","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411373280,"raw_hex":"e83daf70fe","size":5,"soft_fail":false,"target_va":5385205408,"va":5411374942,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140fbb800
//     owner=unresolved_namespace; count=2
//     target_aliases=4 candidates; complete list: ui_color_decorator.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-26168297","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411373280,"raw_hex":"e817b470fe","size":5,"soft_fail":false,"target_va":5385205760,"va":5411374052,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1419affd0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ui_color_decorator.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-15730555","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411373280,"raw_hex":"e885f80fff","size":5,"soft_fail":false,"target_va":5395644368,"va":5411374918,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1428840d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ui_color_decorator.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-180549","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411373280,"raw_hex":"e8bb3efdff","size":5,"soft_fail":false,"target_va":5411193040,"va":5411373584,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142893cd0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ui_color_decorator.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-117218","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411373280,"raw_hex":"e81e36feff","size":5,"soft_fail":false,"target_va":5411257552,"va":5411374765,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142894170
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ui_color_decorator.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-116061","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411373280,"raw_hex":"e8a33afeff","size":5,"soft_fail":false,"target_va":5411258736,"va":5411374792,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142b14b90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ui_color_decorator.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2507803","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411373280,"raw_hex":"e81b442600","size":5,"soft_fail":false,"target_va":5413882768,"va":5411374960,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f320
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ui_color_decorator.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4974651","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411373280,"raw_hex":"e83be84b00","size":5,"soft_fail":false,"target_va":5416350496,"va":5411375840,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f550
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ui_color_decorator.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4975385","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411373280,"raw_hex":"e819eb4b00","size":5,"soft_fail":false,"target_va":5416351056,"va":5411375666,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: ui_color_decorator.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5682468","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411373280,"raw_hex":"e824b55600","size":5,"soft_fail":false,"target_va":5417057056,"va":5411374583,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-26579591","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411373280,"raw_hex":"e8796d6afe","size":5,"soft_fail":false,"target_va":5384794864,"va":5411374450,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t5246212","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411373280,"raw_hex":"e8040d5000","size":5,"soft_fail":false,"target_va":5416622208,"va":5411375991,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t5245748","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411373280,"raw_hex":"e8340b5000","size":5,"soft_fail":false,"target_va":5416621762,"va":5411376009,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::stairs::stairs_state::StairsState::graph
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-31821554","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411373280,"raw_hex":"e80e711afe","size":5,"soft_fail":false,"target_va":5379552272,"va":5411373821,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms::transform
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-29719707","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411373280,"raw_hex":"e865833afe","size":5,"soft_fail":false,"target_va":5381655760,"va":5411375462,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator_outline::DecoratorOutline::display
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-33182633","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411373280,"raw_hex":"e857ac05fe","size":5,"soft_fail":false,"target_va":5378192976,"va":5411375604,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator_storage::DecoratorStorage::decorator_info
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-31891289","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411373280,"raw_hex":"e8a76019fe","size":5,"soft_fail":false,"target_va":5379482496,"va":5411373780,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator_storage::DecoratorStorage::lookup_decorator_info
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-31870963","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411373280,"raw_hex":"e80db019fe","size":5,"soft_fail":false,"target_va":5379502992,"va":5411373950,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator_storage::DecoratorStorage::lookup_storage
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-31890884","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411373280,"raw_hex":"e83c6219fe","size":5,"soft_fail":false,"target_va":5379483104,"va":5411373983,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator_storage::DecoratorStorageAddr::as_wall
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-31866030","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411373280,"raw_hex":"e852c319fe","size":5,"soft_fail":false,"target_va":5379509504,"va":5411375529,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary::get_outline
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-29723966","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411373280,"raw_hex":"e8c2723afe","size":5,"soft_fail":false,"target_va":5381651536,"va":5411375497,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo::subtype
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-29720054","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411373280,"raw_hex":"e80a823afe","size":5,"soft_fail":false,"target_va":5381654320,"va":5411374369,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_walls::PublicWalls::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-30472016","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411373280,"raw_hex":"e8b0082ffe","size":5,"soft_fail":false,"target_va":5380903536,"va":5411375547,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::collision::world::RaycastWorld::legacy_raycast_w_params
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-32208540","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411373280,"raw_hex":"e8648914fe","size":5,"soft_fail":false,"target_va":5379164880,"va":5411373415,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::resources::ui_state::UiWidgetContext::clicked
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-33254869","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411373280,"raw_hex":"e82b9204fe","size":5,"soft_fail":false,"target_va":5378120752,"va":5411375616,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::resources::ui_state::UiWidgetContext::is_active
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-33253991","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411373280,"raw_hex":"e8999504fe","size":5,"soft_fail":false,"target_va":5378120848,"va":5411374834,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::resources::widget_params::UiSounds::empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-31349263","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411373280,"raw_hex":"e8f1a521fe","size":5,"soft_fail":false,"target_va":5380025344,"va":5411374602,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::resources::widget_params::UiSounds::on_click
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-31349321","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411373280,"raw_hex":"e8b7a521fe","size":5,"soft_fail":false,"target_va":5380025376,"va":5411374692,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::resources::widget_params::UiSounds::on_hover_start
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-31349516","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411373280,"raw_hex":"e8f4a421fe","size":5,"soft_fail":false,"target_va":5380025136,"va":5411374647,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::resources::widget_params::WidgetParams::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-31349774","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411373280,"raw_hex":"e8f2a321fe","size":5,"soft_fail":false,"target_va":5380024784,"va":5411374553,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::resources::widget_params::WidgetParams::sounds
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-31349828","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411373280,"raw_hex":"e8bca321fe","size":5,"soft_fail":false,"target_va":5380024896,"va":5411374719,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::components::transform::Transform::compute_affine
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-33867383","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411373280,"raw_hex":"e88939fbfd","size":5,"soft_fail":false,"target_va":5377508096,"va":5411375474,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":4027,"record_offset":3688,"parent_record_offset":408,"ancestor_record_offset":408,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/04027.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::message::message_writer::MessageWriter::write
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/message_writer.rs
//     evidence={"module":4027,"record_offset":5624,"parent_record_offset":408,"ancestor_record_offset":408,"inlinee":11206,"source_line":69,"archive":"_pdb/C/modules/04027.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":4027,"record_offset":2368,"parent_record_offset":408,"ancestor_record_offset":408,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/04027.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_some_and
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":4027,"record_offset":2340,"parent_record_offset":408,"ancestor_record_offset":408,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/04027.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::map
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":4027,"record_offset":3456,"parent_record_offset":408,"ancestor_record_offset":408,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/04027.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":4027,"record_offset":6352,"parent_record_offset":408,"ancestor_record_offset":408,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/04027.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::last
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":4027,"record_offset":3428,"parent_record_offset":408,"ancestor_record_offset":408,"inlinee":5756,"source_line":281,"archive":"_pdb/C/modules/04027.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$171::any
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":4027,"record_offset":3516,"parent_record_offset":408,"ancestor_record_offset":408,"inlinee":8261,"source_line":326,"archive":"_pdb/C/modules/04027.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":4027,"record_offset":3632,"parent_record_offset":408,"ancestor_record_offset":408,"inlinee":26241,"source_line":273,"archive":"_pdb/C/modules/04027.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui::resources::widget_params::WidgetParams::with_flags
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/resources/widget_params.rs
//     evidence={"module":4027,"record_offset":4024,"parent_record_offset":408,"ancestor_record_offset":408,"inlinee":26773,"source_line":125,"archive":"_pdb/C/modules/04027.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::distance
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":4027,"record_offset":3772,"parent_record_offset":408,"ancestor_record_offset":408,"inlinee":11749,"source_line":578,"archive":"_pdb/C/modules/04027.jsonl.gz"}
//   [pdb_inline_direct] petgraph::graphmap::GraphMap::neighbors
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"petgraph","version":"0.8.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/petgraph-0.8.1/src/graphmap.rs
//     evidence={"module":4027,"record_offset":1588,"parent_record_offset":408,"ancestor_record_offset":408,"inlinee":12046,"source_line":454,"archive":"_pdb/C/modules/04027.jsonl.gz"}
//   [pdb_inline_direct] petgraph::graphmap::impl$9::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"petgraph","version":"0.8.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/petgraph-0.8.1/src/graphmap.rs
//     evidence={"module":4027,"record_offset":4112,"parent_record_offset":408,"ancestor_record_offset":408,"inlinee":11051,"source_line":719,"archive":"_pdb/C/modules/04027.jsonl.gz"}
//   [pdb_inline_direct] smallvec::impl$17::deref
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"smallvec","version":"1.13.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.13.2/src/lib.rs
//     evidence={"module":4027,"record_offset":3344,"parent_record_offset":408,"ancestor_record_offset":408,"inlinee":12071,"source_line":1846,"archive":"_pdb/C/modules/04027.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":4027,"record_offset":608,"parent_record_offset":408,"ancestor_record_offset":408,"inlinee":7239,"source_line":997,"archive":"_pdb/C/modules/04027.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4027,"record_offset":6528,"parent_record_offset":6504,"ancestor_record_offset":408,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/04027.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4027,"record_offset":6552,"parent_record_offset":6528,"ancestor_record_offset":408,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/04027.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4027,"record_offset":6576,"parent_record_offset":6552,"ancestor_record_offset":408,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/04027.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4027,"record_offset":6504,"parent_record_offset":6448,"ancestor_record_offset":408,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/04027.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":4027,"record_offset":3736,"parent_record_offset":3712,"ancestor_record_offset":408,"inlinee":6853,"source_line":659,"archive":"_pdb/C/modules/04027.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4027,"record_offset":1860,"parent_record_offset":1836,"ancestor_record_offset":408,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/04027.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4027,"record_offset":6476,"parent_record_offset":6448,"ancestor_record_offset":408,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/04027.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4027,"record_offset":6448,"parent_record_offset":6424,"ancestor_record_offset":408,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/04027.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4027,"record_offset":1908,"parent_record_offset":1884,"ancestor_record_offset":408,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/04027.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4027,"record_offset":1884,"parent_record_offset":1860,"ancestor_record_offset":408,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/04027.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4027,"record_offset":6424,"parent_record_offset":6400,"ancestor_record_offset":408,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/04027.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":4027,"record_offset":5820,"parent_record_offset":5780,"ancestor_record_offset":408,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/04027.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":4027,"record_offset":1836,"parent_record_offset":1808,"ancestor_record_offset":408,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/04027.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":4027,"record_offset":1808,"parent_record_offset":1780,"ancestor_record_offset":408,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/04027.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":4027,"record_offset":5748,"parent_record_offset":5712,"ancestor_record_offset":408,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/04027.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":4027,"record_offset":5780,"parent_record_offset":5748,"ancestor_record_offset":408,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/04027.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":4027,"record_offset":1780,"parent_record_offset":1752,"ancestor_record_offset":408,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/04027.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":4027,"record_offset":5684,"parent_record_offset":5648,"ancestor_record_offset":408,"inlinee":7224,"source_line":397,"archive":"_pdb/C/modules/04027.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":4027,"record_offset":5648,"parent_record_offset":5624,"ancestor_record_offset":408,"inlinee":7222,"source_line":434,"archive":"_pdb/C/modules/04027.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::messages::Messages::write_with_caller
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/messages.rs
//     evidence={"module":4027,"record_offset":5712,"parent_record_offset":5648,"ancestor_record_offset":408,"inlinee":8681,"source_line":129,"archive":"_pdb/C/modules/04027.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":4027,"record_offset":1320,"parent_record_offset":1296,"ancestor_record_offset":408,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/04027.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":4027,"record_offset":3656,"parent_record_offset":3632,"ancestor_record_offset":408,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/04027.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":4027,"record_offset":888,"parent_record_offset":864,"ancestor_record_offset":408,"inlinee":4309,"source_line":1333,"archive":"_pdb/C/modules/04027.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":4027,"record_offset":972,"parent_record_offset":948,"ancestor_record_offset":408,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/04027.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":4027,"record_offset":3980,"parent_record_offset":3956,"ancestor_record_offset":408,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/04027.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":4027,"record_offset":1416,"parent_record_offset":752,"ancestor_record_offset":408,"inlinee":5448,"source_line":433,"archive":"_pdb/C/modules/04027.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":4027,"record_offset":1120,"parent_record_offset":1092,"ancestor_record_offset":408,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/04027.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":4027,"record_offset":1092,"parent_record_offset":1064,"ancestor_record_offset":408,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/04027.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":4027,"record_offset":3840,"parent_record_offset":3804,"ancestor_record_offset":408,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/04027.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":4027,"record_offset":3712,"parent_record_offset":3688,"ancestor_record_offset":408,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/04027.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":4027,"record_offset":912,"parent_record_offset":888,"ancestor_record_offset":408,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/04027.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":4027,"record_offset":6376,"parent_record_offset":6352,"ancestor_record_offset":408,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/04027.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":4027,"record_offset":1236,"parent_record_offset":1212,"ancestor_record_offset":408,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/04027.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":4027,"record_offset":3568,"parent_record_offset":3544,"ancestor_record_offset":408,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/04027.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":4027,"record_offset":5940,"parent_record_offset":5780,"ancestor_record_offset":408,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/04027.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":4027,"record_offset":3544,"parent_record_offset":3516,"ancestor_record_offset":408,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/04027.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::stairs::stairs_assembly_params::impl$26::eq
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/stairs/stairs_assembly_params.rs
//     evidence={"module":4027,"record_offset":3128,"parent_record_offset":3104,"ancestor_record_offset":408,"inlinee":8003,"source_line":124,"archive":"_pdb/C/modules/04027.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::stairs::stairs_state::impl$17::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/stairs/stairs_state.rs
//     evidence={"module":4027,"record_offset":1344,"parent_record_offset":1320,"ancestor_record_offset":408,"inlinee":8701,"source_line":38,"archive":"_pdb/C/modules/04027.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator::impl$33::eq
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":4027,"record_offset":1368,"parent_record_offset":1344,"ancestor_record_offset":408,"inlinee":7427,"source_line":155,"archive":"_pdb/C/modules/04027.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::public_wall_state::PublicWallState::is_premade_shape
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_wall_state.rs
//     evidence={"module":4027,"record_offset":5588,"parent_record_offset":5564,"ancestor_record_offset":408,"inlinee":26091,"source_line":128,"archive":"_pdb/C/modules/04027.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::ui::resources::widget_params::WidgetFlags::insert
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitflags","version":"1.3.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-1.3.2/src/lib.rs
//     evidence={"module":4027,"record_offset":4076,"parent_record_offset":4052,"ancestor_record_offset":408,"inlinee":26777,"source_line":609,"archive":"_pdb/C/modules/04027.jsonl.gz"}
//   [pdb_inline_transitive] equivalent::impl$0::equivalent
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"equivalent","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/equivalent-1.0.1/src/lib.rs
//     evidence={"module":4027,"record_offset":1964,"parent_record_offset":1680,"ancestor_record_offset":408,"inlinee":4329,"source_line":88,"archive":"_pdb/C/modules/04027.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":4027,"record_offset":3924,"parent_record_offset":3804,"ancestor_record_offset":408,"inlinee":9777,"source_line":248,"archive":"_pdb/C/modules/04027.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":4027,"record_offset":3804,"parent_record_offset":3772,"ancestor_record_offset":408,"inlinee":9774,"source_line":1729,"archive":"_pdb/C/modules/04027.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":4027,"record_offset":1032,"parent_record_offset":1004,"ancestor_record_offset":408,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/04027.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":4027,"record_offset":1064,"parent_record_offset":1032,"ancestor_record_offset":408,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/04027.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":4027,"record_offset":1004,"parent_record_offset":752,"ancestor_record_offset":408,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/04027.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":4027,"record_offset":864,"parent_record_offset":752,"ancestor_record_offset":408,"inlinee":4307,"source_line":51,"archive":"_pdb/C/modules/04027.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":4027,"record_offset":1444,"parent_record_offset":752,"ancestor_record_offset":408,"inlinee":4337,"source_line":94,"archive":"_pdb/C/modules/04027.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":4027,"record_offset":948,"parent_record_offset":752,"ancestor_record_offset":408,"inlinee":4311,"source_line":76,"archive":"_pdb/C/modules/04027.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\tag.rs
//     evidence={"module":4027,"record_offset":808,"parent_record_offset":752,"ancestor_record_offset":408,"inlinee":4405,"source_line":36,"archive":"_pdb/C/modules/04027.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\lib.rs
//     evidence={"module":4027,"record_offset":1296,"parent_record_offset":1272,"ancestor_record_offset":408,"inlinee":5439,"source_line":173,"archive":"_pdb/C/modules/04027.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":4027,"record_offset":640,"parent_record_offset":608,"ancestor_record_offset":408,"inlinee":4299,"source_line":1304,"archive":"_pdb/C/modules/04027.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":4027,"record_offset":1272,"parent_record_offset":1164,"ancestor_record_offset":408,"inlinee":4327,"source_line":224,"archive":"_pdb/C/modules/04027.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":4027,"record_offset":676,"parent_record_offset":640,"ancestor_record_offset":408,"inlinee":4403,"source_line":253,"archive":"_pdb/C/modules/04027.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":4027,"record_offset":1212,"parent_record_offset":1188,"ancestor_record_offset":408,"inlinee":4324,"source_line":312,"archive":"_pdb/C/modules/04027.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":4027,"record_offset":1536,"parent_record_offset":752,"ancestor_record_offset":408,"inlinee":4339,"source_line":83,"archive":"_pdb/C/modules/04027.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":4027,"record_offset":1188,"parent_record_offset":1164,"ancestor_record_offset":408,"inlinee":4322,"source_line":785,"archive":"_pdb/C/modules/04027.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":4027,"record_offset":728,"parent_record_offset":704,"ancestor_record_offset":408,"inlinee":4302,"source_line":1226,"archive":"_pdb/C/modules/04027.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":4027,"record_offset":704,"parent_record_offset":640,"ancestor_record_offset":408,"inlinee":4301,"source_line":1249,"archive":"_pdb/C/modules/04027.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":4027,"record_offset":752,"parent_record_offset":728,"ancestor_record_offset":408,"inlinee":4304,"source_line":2049,"archive":"_pdb/C/modules/04027.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":4027,"record_offset":836,"parent_record_offset":752,"ancestor_record_offset":408,"inlinee":4305,"source_line":2488,"archive":"_pdb/C/modules/04027.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":4027,"record_offset":1164,"parent_record_offset":752,"ancestor_record_offset":408,"inlinee":4321,"source_line":1236,"archive":"_pdb/C/modules/04027.jsonl.gz"}
//   [pdb_inline_transitive] indexmap::inner::Core::as_entries
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"indexmap","version":"2.13.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/indexmap-2.13.0/src/inner.rs
//     evidence={"module":4027,"record_offset":1752,"parent_record_offset":1724,"ancestor_record_offset":408,"inlinee":9868,"source_line":125,"archive":"_pdb/C/modules/04027.jsonl.gz"}
//   [pdb_inline_transitive] indexmap::map::IndexMap::as_entries
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"indexmap","version":"2.13.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/indexmap-2.13.0/src/map.rs
//     evidence={"module":4027,"record_offset":1724,"parent_record_offset":1680,"ancestor_record_offset":408,"inlinee":9867,"source_line":193,"archive":"_pdb/C/modules/04027.jsonl.gz"}
//   [pdb_inline_transitive] indexmap::map::IndexMap::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"indexmap","version":"2.13.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/indexmap-2.13.0/src/map.rs
//     evidence={"module":4027,"record_offset":1632,"parent_record_offset":1588,"ancestor_record_offset":408,"inlinee":9866,"source_line":838,"archive":"_pdb/C/modules/04027.jsonl.gz"}
//   [pdb_inline_transitive] indexmap::map::IndexMap::get_index_of
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"indexmap","version":"2.13.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/indexmap-2.13.0/src/map.rs
//     evidence={"module":4027,"record_offset":1680,"parent_record_offset":1632,"ancestor_record_offset":408,"inlinee":14846,"source_line":885,"archive":"_pdb/C/modules/04027.jsonl.gz"}
//   [pdb_inline_transitive] indexmap::map::IndexMap::hash
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"indexmap","version":"2.13.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/indexmap-2.13.0/src/map.rs
//     evidence={"module":4027,"record_offset":2020,"parent_record_offset":1680,"ancestor_record_offset":408,"inlinee":14822,"source_line":819,"archive":"_pdb/C/modules/04027.jsonl.gz"}
//   [pdb_inline_transitive] petgraph::graphmap::impl$9::next::closure$1
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"petgraph","version":"0.8.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/petgraph-0.8.1/src/graphmap.rs
//     evidence={"module":4027,"record_offset":4220,"parent_record_offset":4144,"ancestor_record_offset":408,"inlinee":11054,"source_line":725,"archive":"_pdb/C/modules/04027.jsonl.gz"}
//   [pdb_inline_transitive] smallvec::SmallVec::spilled
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"smallvec","version":"1.13.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.13.2/src/lib.rs
//     evidence={"module":4027,"record_offset":3392,"parent_record_offset":3368,"ancestor_record_offset":408,"inlinee":4822,"source_line":1001,"archive":"_pdb/C/modules/04027.jsonl.gz"}
//   [pdb_inline_transitive] smallvec::SmallVec::triple
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"smallvec","version":"1.13.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.13.2/src/lib.rs
//     evidence={"module":4027,"record_offset":3368,"parent_record_offset":3344,"ancestor_record_offset":408,"inlinee":12072,"source_line":971,"archive":"_pdb/C/modules/04027.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":4027,"record_offset":3956,"parent_record_offset":3924,"ancestor_record_offset":408,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/04027.jsonl.gz"}
//   [pdb_inline_transitive] system_color::ui_color_decorator::ui_color_windows_and_flags::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-color","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/color/src/ui_color_decorator.rs
//     evidence={"module":4027,"record_offset":5360,"parent_record_offset":5332,"ancestor_record_offset":408,"inlinee":134977,"source_line":90,"archive":"_pdb/C/modules/04027.jsonl.gz"}
//   [pdb_inline_transitive] system_color::ui_color_decorator::ui_color_windows_and_flags::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-color","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/color/src/ui_color_decorator.rs
//     evidence={"module":4027,"record_offset":5476,"parent_record_offset":5392,"ancestor_record_offset":408,"inlinee":134978,"source_line":97,"archive":"_pdb/C/modules/04027.jsonl.gz"}
//   [pdb_inline_transitive] system_color::ui_color_decorator::ui_color_windows_and_flags::closure$3
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-color","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/color/src/ui_color_decorator.rs
//     evidence={"module":4027,"record_offset":3484,"parent_record_offset":3456,"ancestor_record_offset":408,"inlinee":134974,"source_line":124,"archive":"_pdb/C/modules/04027.jsonl.gz"}
//   [pdb_inline_transitive] system_color::ui_color_decorator::ui_color_windows_and_flags::closure$4
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-color","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/color/src/ui_color_decorator.rs
//     evidence={"module":4027,"record_offset":3600,"parent_record_offset":3516,"ancestor_record_offset":408,"inlinee":134975,"source_line":128,"archive":"_pdb/C/modules/04027.jsonl.gz"}
//   [pdb_inline_transitive] system_color::ui_color_decorator::ui_color_windows_and_flags::closure$5
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-color","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/color/src/ui_color_decorator.rs
//     evidence={"module":4027,"record_offset":4052,"parent_record_offset":4024,"ancestor_record_offset":408,"inlinee":134976,"source_line":151,"archive":"_pdb/C/modules/04027.jsonl.gz"}
//   [pdb_inline_transitive] system_color::ui_color_decorator::ui_color_windows_and_flags::closure$7
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-color","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/color/src/ui_color_decorator.rs
//     evidence={"module":4027,"record_offset":5564,"parent_record_offset":5536,"ancestor_record_offset":408,"inlinee":134979,"source_line":196,"archive":"_pdb/C/modules/04027.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_color::ui_color_decorator::ui_color_windows_and_flags::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 90
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x20F41
// 首条位置: {"module":4027,"inlinee":134977}
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
// 源码行: 97
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x20F42
// 首条位置: {"module":4027,"inlinee":134978}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_color::ui_color_decorator::ui_color_windows_and_flags::closure$3
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 124
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x20F3E
// 首条位置: {"module":4027,"inlinee":134974}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_color::ui_color_decorator::ui_color_windows_and_flags::closure$4
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 128
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x20F3F
// 首条位置: {"module":4027,"inlinee":134975}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_color::ui_color_decorator::ui_color_windows_and_flags::closure$5
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 151
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x20F40
// 首条位置: {"module":4027,"inlinee":134976}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::systems::ui::resources::widget_params::WidgetFlags::insert
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitflags","version":"1.3.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-1.3.2/src/lib.rs
//     evidence={"module":4027,"record_offset":4076,"parent_record_offset":4052,"ancestor_record_offset":4052,"inlinee":26777,"source_line":609,"archive":"_pdb/C/modules/04027.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: core::ops::function::FnOnce::call_once<system_color::ui_color_decorator::ui_color_windows_and_flags::closure_env$6,tuple$<ref$<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,ref$<country_core::asset_libraries::shader_library::AssetShaderLibrary>,ref_mut$<bevy_ecs::system::commands::Commands> > >
// 记录形式: compiled_procedure; instances=1
// 源码行: 162
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":2707,"record_offset":56356,"rva":35089840,"va":5403798960}
// 编译实例 RVAs: 0x2176db0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-28120327","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403798960,"raw_hex":"e8f9ea52fe","size":5,"soft_fail":false,"target_va":5375678688,"va":5403799010,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-28120455","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403798960,"raw_hex":"e879ea52fe","size":5,"soft_fail":false,"target_va":5375678704,"va":5403799154,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=1
//     target_aliases=1699 candidates; complete list: ui_color_decorator.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-35076808","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403798960,"raw_hex":"e838c5e8fd","size":5,"soft_fail":false,"target_va":5368722192,"va":5403798995,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t12821651","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403798960,"raw_hex":"e893a4c300","size":5,"soft_fail":false,"target_va":5416620769,"va":5403799113,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_color::ui_color_wall_and_roof::color_cursor_icon
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"system-color","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-10214","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403798960,"raw_hex":"e81ad8ffff","size":5,"soft_fail":false,"target_va":5403788848,"va":5403799057,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":2707,"record_offset":56752,"parent_record_offset":56356,"ancestor_record_offset":56356,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2707,"record_offset":56896,"parent_record_offset":56872,"ancestor_record_offset":56356,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2707,"record_offset":56920,"parent_record_offset":56896,"ancestor_record_offset":56356,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2707,"record_offset":57148,"parent_record_offset":57124,"ancestor_record_offset":56356,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2707,"record_offset":57172,"parent_record_offset":57148,"ancestor_record_offset":56356,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2707,"record_offset":56944,"parent_record_offset":56920,"ancestor_record_offset":56356,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2707,"record_offset":57196,"parent_record_offset":57172,"ancestor_record_offset":56356,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2707,"record_offset":56872,"parent_record_offset":56844,"ancestor_record_offset":56356,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2707,"record_offset":57124,"parent_record_offset":57100,"ancestor_record_offset":56356,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_assume_init_into_vec_unsafe
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":2707,"record_offset":56992,"parent_record_offset":56776,"ancestor_record_offset":56356,"inlinee":4459,"source_line":262,"archive":"_pdb/C/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":2707,"record_offset":56844,"parent_record_offset":56816,"ancestor_record_offset":56356,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new_uninit
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":2707,"record_offset":56816,"parent_record_offset":56776,"ancestor_record_offset":56356,"inlinee":4443,"source_line":311,"archive":"_pdb/C/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$10::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":2707,"record_offset":57100,"parent_record_offset":57076,"ancestor_record_offset":56356,"inlinee":4463,"source_line":1913,"archive":"_pdb/C/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":2707,"record_offset":57016,"parent_record_offset":56992,"ancestor_record_offset":56356,"inlinee":4460,"source_line":480,"archive":"_pdb/C/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::from_raw_parts_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2707,"record_offset":57040,"parent_record_offset":57016,"ancestor_record_offset":56356,"inlinee":4461,"source_line":1181,"archive":"_pdb/C/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":2707,"record_offset":57076,"parent_record_offset":56776,"ancestor_record_offset":56356,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] system_color::ui_color_decorator::ui_color_windows_and_flags::closure$6
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-color","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/color/src/ui_color_decorator.rs
//     evidence={"module":2707,"record_offset":56776,"parent_record_offset":56752,"ancestor_record_offset":56356,"inlinee":108332,"source_line":162,"archive":"_pdb/C/modules/02707.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_color::ui_color_decorator::ui_color_windows_and_flags::closure$6
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=3
// 源码行: 162
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1A72C
// 首条位置: {"module":2707,"inlinee":108332}
// 编译实例 RVAs: 0x2176db0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-28120327","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403798960,"raw_hex":"e8f9ea52fe","size":5,"soft_fail":false,"target_va":5375678688,"va":5403799010,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-28120455","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403798960,"raw_hex":"e879ea52fe","size":5,"soft_fail":false,"target_va":5375678704,"va":5403799154,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=1
//     target_aliases=1699 candidates; complete list: ui_color_decorator.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-35076808","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403798960,"raw_hex":"e838c5e8fd","size":5,"soft_fail":false,"target_va":5368722192,"va":5403798995,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t12821651","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403798960,"raw_hex":"e893a4c300","size":5,"soft_fail":false,"target_va":5416620769,"va":5403799113,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_color::ui_color_wall_and_roof::color_cursor_icon
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"system-color","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-10214","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403798960,"raw_hex":"e81ad8ffff","size":5,"soft_fail":false,"target_va":5403788848,"va":5403799057,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::boxed::box_assume_init_into_vec_unsafe
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":2707,"record_offset":56100,"parent_record_offset":55760,"ancestor_record_offset":55760,"inlinee":4459,"source_line":262,"archive":"_pdb/C/modules/02707.jsonl.gz"}
//   [pdb_inline_direct] alloc::boxed::impl$0::new_uninit
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":2707,"record_offset":55924,"parent_record_offset":55760,"ancestor_record_offset":55760,"inlinee":4443,"source_line":311,"archive":"_pdb/C/modules/02707.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":2707,"record_offset":56184,"parent_record_offset":55760,"ancestor_record_offset":55760,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2707,"record_offset":56004,"parent_record_offset":55980,"ancestor_record_offset":55760,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2707,"record_offset":56028,"parent_record_offset":56004,"ancestor_record_offset":55760,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2707,"record_offset":56256,"parent_record_offset":56232,"ancestor_record_offset":55760,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2707,"record_offset":56280,"parent_record_offset":56256,"ancestor_record_offset":55760,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2707,"record_offset":56052,"parent_record_offset":56028,"ancestor_record_offset":55760,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2707,"record_offset":56304,"parent_record_offset":56280,"ancestor_record_offset":55760,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2707,"record_offset":55980,"parent_record_offset":55952,"ancestor_record_offset":55760,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2707,"record_offset":56232,"parent_record_offset":56208,"ancestor_record_offset":55760,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":2707,"record_offset":55952,"parent_record_offset":55924,"ancestor_record_offset":55760,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$10::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":2707,"record_offset":56208,"parent_record_offset":56184,"ancestor_record_offset":55760,"inlinee":4463,"source_line":1913,"archive":"_pdb/C/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::into_vec
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":2707,"record_offset":56124,"parent_record_offset":56100,"ancestor_record_offset":55760,"inlinee":4460,"source_line":480,"archive":"_pdb/C/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::from_raw_parts_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2707,"record_offset":56148,"parent_record_offset":56124,"ancestor_record_offset":55760,"inlinee":4461,"source_line":1181,"archive":"_pdb/C/modules/02707.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_color::ui_color_decorator::ui_color_windows_and_flags::closure$7
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 196
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x20F43
// 首条位置: {"module":4027,"inlinee":134979}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::resources::walls::public_wall_state::PublicWallState::is_premade_shape
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_wall_state.rs
//     evidence={"module":4027,"record_offset":5588,"parent_record_offset":5564,"ancestor_record_offset":5564,"inlinee":26091,"source_line":128,"archive":"_pdb/C/modules/04027.jsonl.gz"}
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
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::message::message_writer::MessageWriter<system_color::show_color_menu::ShowColorMenuCmd>,bevy_ecs::message::message_writer::MessageWriter<system_color::show_color_menu::HideColorMenuCmd>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::wall::structure_outline::OutlineStairsCmd>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyRequests>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_state::StairsState>,bevy_ecs::change_detection::Res<country_core::resources::stairs::visual::StairsVisualState>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >", "bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>", "bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>", "bevy_ecs::message::message_writer::MessageWriter<system_color::show_color_menu::ShowColorMenuCmd>", "bevy_ecs::message::message_writer::MessageWriter<system_color::show_color_menu::HideColorMenuCmd>", "bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>", "bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>", "country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>", "bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>", "bevy_ecs::message::message_writer::MessageWriter<country_core::systems::wall::structure_outline::OutlineStairsCmd>", "bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyRequests>", "bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_state::StairsState>", "bevy_ecs::change_detection::Res<country_core::resources::stairs::visual::StairsVisualState>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<system_color::show_color_menu::ShowColorMenuCmd>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<system_color::show_color_menu::HideColorMenuCmd>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::systems::wall::structure_outline::OutlineStairsCmd>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyRequests>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_state::StairsState>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::stairs::visual::StairsVisualState>

// ------------------------------------------------------------------
// 函数: core::ops::function::FnOnce::call_once<system_color::ui_color_decorator::ui_color_windows_and_flags::closure_env$6,tuple$<ref$<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,ref$<country_core::asset_libraries::shader_library::AssetShaderLibrary>,ref_mut$<bevy_ecs::system::commands::Commands> > >
// 信息 ID: fe9e1ed97b708bc527f79a5e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [系统接口类型拼写] deferred_command_interface: bevy_ecs::system::commands::Commands
// [类型关系] ref$ -> ["country_core::asset_libraries::mesh_library::AssetMeshLibrary"]; 字段布局 = unknown。
// [类型关系] ref$ -> ["country_core::asset_libraries::shader_library::AssetShaderLibrary"]; 字段布局 = unknown。
// [类型关系] ref_mut$ -> ["bevy_ecs::system::commands::Commands"]; 字段布局 = unknown。
// [类型关系] tuple$ -> ["ref$<country_core::asset_libraries::mesh_library::AssetMeshLibrary>", "ref$<country_core::asset_libraries::shader_library::AssetShaderLibrary>", "ref_mut$<bevy_ecs::system::commands::Commands>"]; 字段布局 = unknown。
// [类型关系] core::ops::function::FnOnce::call_once -> ["system_color::ui_color_decorator::ui_color_windows_and_flags::closure_env$6", "tuple$<ref$<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,ref$<country_core::asset_libraries::shader_library::AssetShaderLibrary>,ref_mut$<bevy_ecs::system::commands::Commands> >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_color::ui_color_decorator::ui_color_windows_and_flags
// 信息 ID: eca9f4970798395d15bd721a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 088903f17b9eaad55a17dafb: void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::message::message_writer::MessageWriter<system_color::show_color_menu::ShowColorMenuCmd>,bevy_ecs::message::message_writer::MessageWriter<system_color::show_color_menu::HideColorMenuCmd>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::wall::structure_outline::OutlineStairsCmd>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyRequests>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_state::StairsState>,bevy_ecs::change_detection::Res<country_core::resources::stairs::visual::StairsVisualState>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::message::message_writer::MessageWriter<system_color::show_color_menu::ShowColorMenuCmd>,bevy_ecs::message::message_writer::MessageWriter<system_color::show_color_menu::HideColorMenuCmd>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::wall::structure_outline::OutlineStairsCmd>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyRequests>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_state::StairsState>,bevy_ecs::change_detection::Res<country_core::resources::stairs::visual::StairsVisualState>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::message::message_writer::MessageWriter<system_color::show_color_menu::ShowColorMenuCmd>,bevy_ecs::message::message_writer::MessageWriter<system_color::show_color_menu::HideColorMenuCmd>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::wall::structure_outline::OutlineStairsCmd>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyRequests>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_state::StairsState>,bevy_ecs::change_detection::Res<country_core::resources::stairs::visual::StairsVisualState>)> > @ 0x21664d2 -> 0x28b00e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$12::run_unsafe<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::message::message_writer::MessageWriter<system_color::show_color_menu::ShowColorMenuCmd>,bevy_ecs::message::message_writer::MessageWriter<system_color::show_color_menu::HideColorMenuCmd>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::wall::structure_outline::OutlineStairsCmd>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyRequests>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_state::StairsState>,bevy_ecs::change_detection::Res<country_core::resources::stairs::visual::StairsVisualState>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::message::message_writer::MessageWriter<system_color::show_color_menu::ShowColorMenuCmd>,bevy_ecs::message::message_writer::MessageWriter<system_color::show_color_menu::HideColorMenuCmd>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::wall::structure_outline::OutlineStairsCmd>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyRequests>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_state::StairsState>,bevy_ecs::change_detection::Res<country_core::resources::stairs::visual::StairsVisualState>)> @ 0x2176765 -> 0x28b00e0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_color::ui_color_decorator::ui_color_windows_and_flags::closure$0
// 信息 ID: 1ef181448636a0bc25396988
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_color::ui_color_decorator::ui_color_windows_and_flags", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_color::ui_color_decorator::ui_color_windows_and_flags; module=4027, inline_record=5360; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_color::ui_color_decorator::ui_color_windows_and_flags::closure$1
// 信息 ID: 0624beeae616824a7f43cbc1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "system_color::ui_color_decorator::ui_color_windows_and_flags", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_color::ui_color_decorator::ui_color_windows_and_flags; module=4027, inline_record=5476; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_color::ui_color_decorator::ui_color_windows_and_flags::closure$3
// 信息 ID: 2a2d4fabb7bc75c4714219bf
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 3, "lexical_owner": "system_color::ui_color_decorator::ui_color_windows_and_flags", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_color::ui_color_decorator::ui_color_windows_and_flags; module=4027, inline_record=3484; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_color::ui_color_decorator::ui_color_windows_and_flags::closure$4
// 信息 ID: 86ed37139b733dadfb54ebe7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 4, "lexical_owner": "system_color::ui_color_decorator::ui_color_windows_and_flags", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_color::ui_color_decorator::ui_color_windows_and_flags; module=4027, inline_record=3600; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_color::ui_color_decorator::ui_color_windows_and_flags::closure$5
// 信息 ID: 6da4a3da1c24f172872fd989
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 5, "lexical_owner": "system_color::ui_color_decorator::ui_color_windows_and_flags", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_color::ui_color_decorator::ui_color_windows_and_flags; module=4027, inline_record=4052; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_color::ui_color_decorator::ui_color_windows_and_flags::closure$6
// 信息 ID: e3e14bd30f823ad38b784927
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 6, "lexical_owner": "system_color::ui_color_decorator::ui_color_windows_and_flags", "captures": "unknown"}
// [闭包环境类型引用] system_color::ui_color_decorator::ui_color_windows_and_flags::closure_env$6 <- core::ops::function::FnOnce::call_once<system_color::ui_color_decorator::ui_color_windows_and_flags::closure_env$6,tuple$<ref$<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,ref$<country_core::asset_libraries::shader_library::AssetShaderLibrary>,ref_mut$<bevy_ecs::system::commands::Commands> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_color::ui_color_decorator::ui_color_windows_and_flags::closure_env$6 <- country_core::systems::ui::resources::widget_params::WidgetParams::signifier<system_color::ui_color_decorator::ui_color_windows_and_flags::closure_env$6>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 2 条。
// [PDB 内联宿主] core::ops::function::FnOnce::call_once<system_color::ui_color_decorator::ui_color_windows_and_flags::closure_env$6,tuple$<ref$<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,ref$<country_core::asset_libraries::shader_library::AssetShaderLibrary>,ref_mut$<bevy_ecs::system::commands::Commands> > >; module=2707, inline_record=56776; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] namespace_unique_source_candidate; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: system_color::ui_color_decorator::ui_color_windows_and_flags::closure$7
// 信息 ID: e20c654e6b885a30b3bee8fc
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 7, "lexical_owner": "system_color::ui_color_decorator::ui_color_windows_and_flags", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_color::ui_color_decorator::ui_color_windows_and_flags; module=4027, inline_record=5564; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
