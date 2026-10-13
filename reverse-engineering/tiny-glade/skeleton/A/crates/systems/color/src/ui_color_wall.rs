// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/systems/color/src/ui_color_wall.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 665403636f45cb54855675d0af8bab7dc6226be0e095bc0ef11c95b8c210fa38
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: ui_color_wall.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 891, 2233
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
// use nonmax; // namespace candidate
//   package identities: [{"name":"nonmax","source":"PDB_registry_path_only","version":"0.5.5"}]
// use std; // namespace candidate
//   package identities: []
// use system_color; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_color::ui_color_wall::ui_color_wall
// 记录形式: compiled_procedure; instances=1
// 源码行: 25
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":2233,"record_offset":1100,"rva":21664944,"va":5390374064}
// 编译实例 RVAs: 0x14a94b0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: ui_color_wall.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-17626964","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390374064,"raw_hex":"e8ac08f3fe","size":5,"soft_fail":false,"target_va":5372749712,"va":5390376671,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406edb80
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: ui_color_wall.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-14402966","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390374064,"raw_hex":"e86a3a24ff","size":5,"soft_fail":false,"target_va":5375974272,"va":5390377233,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141aa77f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ui_color_wall.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t6283246","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390374064,"raw_hex":"e8eedf5f00","size":5,"soft_fail":false,"target_va":5396658160,"va":5390374909,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141ac59e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ui_color_wall.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t6405453","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390374064,"raw_hex":"e84dbd6100","size":5,"soft_fail":false,"target_va":5396781536,"va":5390376078,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141ac5d70
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ui_color_wall.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t6408088","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390374064,"raw_hex":"e898c76100","size":5,"soft_fail":false,"target_va":5396782448,"va":5390374355,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141ac6100
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ui_color_wall.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t6407247","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390374064,"raw_hex":"e84fc46100","size":5,"soft_fail":false,"target_va":5396783360,"va":5390376108,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141ac6380
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ui_color_wall.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t6409612","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390374064,"raw_hex":"e88ccd6100","size":5,"soft_fail":false,"target_va":5396784000,"va":5390374383,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141ac6490
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ui_color_wall.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t6408531","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390374064,"raw_hex":"e853c96100","size":5,"soft_fail":false,"target_va":5396784272,"va":5390375736,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-12846025","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390374064,"raw_hex":"e837fc3bff","size":5,"soft_fail":false,"target_va":5377529888,"va":5390375908,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve::do_reserve_and_handle
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t12721727","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390374064,"raw_hex":"e83f1ec200","size":5,"soft_fail":false,"target_va":5403097408,"va":5390375676,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t-19069580","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390374064,"raw_hex":"e87405ddfe","size":5,"soft_fail":false,"target_va":5371304880,"va":5390374455,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::expect_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t12602092","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390374064,"raw_hex":"e8ec4ac000","size":5,"soft_fail":false,"target_va":5402978912,"va":5390376815,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t12601994","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390374064,"raw_hex":"e88a4ac000","size":5,"soft_fail":false,"target_va":5402978880,"va":5390376881,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t12602150","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390374064,"raw_hex":"e8264bc000","size":5,"soft_fail":false,"target_va":5402978944,"va":5390376789,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<country_core::systems::ui::resources::widget_params::WidgetVolumeShape> >
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-15161302","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390374064,"raw_hex":"e82aa818ff","size":5,"soft_fail":false,"target_va":5375216304,"va":5390377601,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t12603256","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390374064,"raw_hex":"e8784fc000","size":5,"soft_fail":false,"target_va":5402980128,"va":5390376867,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::roofs::roof_shape::Roof::ty
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-14612084","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390374064,"raw_hex":"e88c0921ff","size":5,"soft_fail":false,"target_va":5375763488,"va":5390375567,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume::triangles
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-16315040","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390374064,"raw_hex":"e8600d07ff","size":5,"soft_fail":false,"target_va":5374060560,"va":5390375595,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_walls::PublicWalls::get
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-14939875","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390374064,"raw_hex":"e81d091cff","size":5,"soft_fail":false,"target_va":5375434976,"va":5390374846,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_walls::PublicWalls::roof_entity
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-14939291","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390374064,"raw_hex":"e8650b1cff","size":5,"soft_fail":false,"target_va":5375436032,"va":5390375318,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_walls::PublicWalls::wall_ids
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-14938996","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390374064,"raw_hex":"e88c0c1cff","size":5,"soft_fail":false,"target_va":5375435520,"va":5390374511,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::wall_trigger_volume::WallTriggerVolume::get_tris
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-8170184","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390374064,"raw_hex":"e8385583ff","size":5,"soft_fail":false,"target_va":5382205120,"va":5390375299,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::wall_trigger_volume::WallTriggerVolume::is_valid
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-8170164","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390374064,"raw_hex":"e84c5583ff","size":5,"soft_fail":false,"target_va":5382205104,"va":5390375263,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::wall_trigger_volume::WallTriggerVolumeParams::new
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-8170213","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390374064,"raw_hex":"e81b5583ff","size":5,"soft_fail":false,"target_va":5382205072,"va":5390375280,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::resources::ui_state::UiWidgetContext::clicked
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-14824117","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390374064,"raw_hex":"e84bcd1dff","size":5,"soft_fail":false,"target_va":5375550288,"va":5390374400,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::resources::ui_state::UiWidgetContext::is_active
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-14825688","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390374064,"raw_hex":"e828c71dff","size":5,"soft_fail":false,"target_va":5375550448,"va":5390376131,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::resources::widget_params::UiSounds::empty
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-14602719","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390374064,"raw_hex":"e8212e21ff","size":5,"soft_fail":false,"target_va":5375771552,"va":5390374266,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::resources::widget_params::UiSounds::on_click
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-14604367","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390374064,"raw_hex":"e8b12721ff","size":5,"soft_fail":false,"target_va":5375771664,"va":5390376026,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::resources::widget_params::WidgetParams::new
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-14603084","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390374064,"raw_hex":"e8b42c21ff","size":5,"soft_fail":false,"target_va":5375771168,"va":5390374247,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::resources::widget_params::WidgetParams::occlude_by_world
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-14602994","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390374064,"raw_hex":"e80e2d21ff","size":5,"soft_fail":false,"target_va":5375771344,"va":5390374333,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::resources::widget_params::WidgetParams::sounds
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-14603018","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390374064,"raw_hex":"e8f62c21ff","size":5,"soft_fail":false,"target_va":5375771280,"va":5390374293,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::resources::widget_params::WidgetVolumeShape::clearing_canvas
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-14602368","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390374064,"raw_hex":"e8802f21ff","size":5,"soft_fail":false,"target_va":5375771856,"va":5390374219,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":2233,"record_offset":6260,"parent_record_offset":1100,"ancestor_record_offset":1100,"inlinee":4097,"source_line":629,"archive":"_pdb/A/modules/02233.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$15::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":2233,"record_offset":1608,"parent_record_offset":1100,"ancestor_record_offset":1100,"inlinee":4248,"source_line":2920,"archive":"_pdb/A/modules/02233.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$18::extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":2233,"record_offset":5696,"parent_record_offset":1100,"ancestor_record_offset":1100,"inlinee":4432,"source_line":2961,"archive":"_pdb/A/modules/02233.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::into_iter::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":2233,"record_offset":1984,"parent_record_offset":1100,"ancestor_record_offset":1100,"inlinee":4425,"source_line":202,"archive":"_pdb/A/modules/02233.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":2233,"record_offset":1284,"parent_record_offset":1100,"ancestor_record_offset":1100,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/02233.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::EventWriter::send
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":2233,"record_offset":1344,"parent_record_offset":1100,"ancestor_record_offset":1100,"inlinee":7450,"source_line":624,"archive":"_pdb/A/modules/02233.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::query::Query::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/query.rs
//     evidence={"module":2233,"record_offset":3232,"parent_record_offset":1100,"ancestor_record_offset":1100,"inlinee":21384,"source_line":846,"archive":"_pdb/A/modules/02233.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::query::Query::iter
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/query.rs
//     evidence={"module":2233,"record_offset":6344,"parent_record_offset":1100,"ancestor_record_offset":1100,"inlinee":14606,"source_line":435,"archive":"_pdb/A/modules/02233.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::last
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":2233,"record_offset":6456,"parent_record_offset":1100,"ancestor_record_offset":1100,"inlinee":6170,"source_line":258,"archive":"_pdb/A/modules/02233.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::and_then
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":2233,"record_offset":4180,"parent_record_offset":1100,"ancestor_record_offset":1100,"inlinee":4151,"source_line":1408,"archive":"_pdb/A/modules/02233.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_none
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":2233,"record_offset":7024,"parent_record_offset":1100,"ancestor_record_offset":1100,"inlinee":4710,"source_line":651,"archive":"_pdb/A/modules/02233.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":2233,"record_offset":2100,"parent_record_offset":1100,"ancestor_record_offset":1100,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/02233.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":2233,"record_offset":7604,"parent_record_offset":1100,"ancestor_record_offset":1100,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/02233.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":2233,"record_offset":7896,"parent_record_offset":1100,"ancestor_record_offset":1100,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/02233.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$9::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":2233,"record_offset":2136,"parent_record_offset":1100,"ancestor_record_offset":1100,"inlinee":17187,"source_line":1342,"archive":"_pdb/A/modules/02233.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":2233,"record_offset":7840,"parent_record_offset":7816,"ancestor_record_offset":1100,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/02233.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":2233,"record_offset":7816,"parent_record_offset":7724,"ancestor_record_offset":1100,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/02233.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":2233,"record_offset":6308,"parent_record_offset":6284,"ancestor_record_offset":1100,"inlinee":4101,"source_line":637,"archive":"_pdb/A/modules/02233.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":2233,"record_offset":6004,"parent_record_offset":5976,"ancestor_record_offset":1100,"inlinee":4277,"source_line":270,"archive":"_pdb/A/modules/02233.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":2233,"record_offset":7752,"parent_record_offset":7724,"ancestor_record_offset":1100,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/02233.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::needs_to_grow
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":2233,"record_offset":5976,"parent_record_offset":5944,"ancestor_record_offset":1100,"inlinee":4276,"source_line":414,"archive":"_pdb/A/modules/02233.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":2233,"record_offset":1536,"parent_record_offset":1504,"ancestor_record_offset":1100,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/02233.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":2233,"record_offset":5944,"parent_record_offset":5920,"ancestor_record_offset":1100,"inlinee":4275,"source_line":318,"archive":"_pdb/A/modules/02233.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":2233,"record_offset":7724,"parent_record_offset":7700,"ancestor_record_offset":1100,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/02233.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":2233,"record_offset":1504,"parent_record_offset":1464,"ancestor_record_offset":1100,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/02233.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":2233,"record_offset":5772,"parent_record_offset":5748,"ancestor_record_offset":1100,"inlinee":5431,"source_line":3007,"archive":"_pdb/A/modules/02233.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":2233,"record_offset":6984,"parent_record_offset":6960,"ancestor_record_offset":1100,"inlinee":4292,"source_line":2202,"archive":"_pdb/A/modules/02233.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":2233,"record_offset":1464,"parent_record_offset":1428,"ancestor_record_offset":1100,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/02233.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":2233,"record_offset":5920,"parent_record_offset":5772,"ancestor_record_offset":1100,"inlinee":4274,"source_line":944,"archive":"_pdb/A/modules/02233.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":2233,"record_offset":7628,"parent_record_offset":7604,"ancestor_record_offset":1100,"inlinee":4436,"source_line":427,"archive":"_pdb/A/modules/02233.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop::impl$0::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":2233,"record_offset":7676,"parent_record_offset":7652,"ancestor_record_offset":1100,"inlinee":4438,"source_line":431,"archive":"_pdb/A/modules/02233.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":2233,"record_offset":6052,"parent_record_offset":5772,"ancestor_record_offset":1100,"inlinee":5454,"source_line":13,"archive":"_pdb/A/modules/02233.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":2233,"record_offset":5748,"parent_record_offset":5696,"ancestor_record_offset":1100,"inlinee":5430,"source_line":25,"archive":"_pdb/A/modules/02233.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::impl$24::eq
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/archetype.rs
//     evidence={"module":2233,"record_offset":3444,"parent_record_offset":3300,"ancestor_record_offset":1100,"inlinee":21390,"source_line":74,"archive":"_pdb/A/modules/02233.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$29::set_changed
//     owner=git_dependency_definition; count=6
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":2233,"record_offset":1312,"parent_record_offset":1284,"ancestor_record_offset":1100,"inlinee":5812,"source_line":309,"archive":"_pdb/A/modules/02233.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":2233,"record_offset":1368,"parent_record_offset":1344,"ancestor_record_offset":1100,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/02233.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::Entities::get
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/entity/mod.rs
//     evidence={"module":2233,"record_offset":3300,"parent_record_offset":3256,"ancestor_record_offset":1100,"inlinee":21387,"source_line":739,"archive":"_pdb/A/modules/02233.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::Events::send
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":2233,"record_offset":1428,"parent_record_offset":1344,"ancestor_record_offset":1100,"inlinee":7451,"source_line":231,"archive":"_pdb/A/modules/02233.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$21::fetch
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":2233,"record_offset":4088,"parent_record_offset":3256,"ancestor_record_offset":1100,"inlinee":14627,"source_line":963,"archive":"_pdb/A/modules/02233.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$21::set_archetype
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":2233,"record_offset":3720,"parent_record_offset":3256,"ancestor_record_offset":1100,"inlinee":21391,"source_line":933,"archive":"_pdb/A/modules/02233.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$21::set_table
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":2233,"record_offset":3744,"parent_record_offset":3720,"ancestor_record_offset":1100,"inlinee":14620,"source_line":948,"archive":"_pdb/A/modules/02233.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIter::fold_over_table_range
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs
//     evidence={"module":2233,"record_offset":6612,"parent_record_offset":6488,"ancestor_record_offset":1100,"inlinee":50844,"source_line":61,"archive":"_pdb/A/modules/02233.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIter::new
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs
//     evidence={"module":2233,"record_offset":6392,"parent_record_offset":6368,"ancestor_record_offset":1100,"inlinee":14609,"source_line":36,"archive":"_pdb/A/modules/02233.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::init
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs
//     evidence={"module":2233,"record_offset":6416,"parent_record_offset":6392,"ancestor_record_offset":1100,"inlinee":14611,"source_line":1520,"archive":"_pdb/A/modules/02233.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::impl$1::fold
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs
//     evidence={"module":2233,"record_offset":6488,"parent_record_offset":6456,"ancestor_record_offset":1100,"inlinee":50842,"source_line":899,"archive":"_pdb/A/modules/02233.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::state::QueryState::get_unchecked_manual
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/state.rs
//     evidence={"module":2233,"record_offset":3256,"parent_record_offset":3232,"ancestor_record_offset":1100,"inlinee":21385,"source_line":859,"archive":"_pdb/A/modules/02233.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::state::QueryState::iter_unchecked_manual
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/state.rs
//     evidence={"module":2233,"record_offset":6368,"parent_record_offset":6344,"ancestor_record_offset":1100,"inlinee":14607,"source_line":1240,"archive":"_pdb/A/modules/02233.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$2::fetch
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/world_query.rs
//     evidence={"module":2233,"record_offset":5484,"parent_record_offset":4256,"ancestor_record_offset":1100,"inlinee":14626,"source_line":201,"archive":"_pdb/A/modules/02233.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$2::set_archetype
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/world_query.rs
//     evidence={"module":2233,"record_offset":4720,"parent_record_offset":4256,"ancestor_record_offset":1100,"inlinee":21665,"source_line":179,"archive":"_pdb/A/modules/02233.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_slice
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":2233,"record_offset":4048,"parent_record_offset":4024,"ancestor_record_offset":1100,"inlinee":14643,"source_line":410,"archive":"_pdb/A/modules/02233.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseArray::get
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":2233,"record_offset":3820,"parent_record_offset":3796,"ancestor_record_offset":1100,"inlinee":6338,"source_line":56,"archive":"_pdb/A/modules/02233.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":2233,"record_offset":3796,"parent_record_offset":3772,"ancestor_record_offset":1100,"inlinee":6337,"source_line":369,"archive":"_pdb/A/modules/02233.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$11::get::closure$0
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":2233,"record_offset":3868,"parent_record_offset":3844,"ancestor_record_offset":1100,"inlinee":6340,"source_line":58,"archive":"_pdb/A/modules/02233.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$16::get::closure$0
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":2233,"record_offset":3956,"parent_record_offset":3932,"ancestor_record_offset":1100,"inlinee":6342,"source_line":370,"archive":"_pdb/A/modules/02233.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Column::get_data_slice
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":2233,"record_offset":4024,"parent_record_offset":3744,"ancestor_record_offset":1100,"inlinee":14642,"source_line":321,"archive":"_pdb/A/modules/02233.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::entity_count
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":2233,"record_offset":6960,"parent_record_offset":6488,"ancestor_record_offset":1100,"inlinee":50843,"source_line":749,"archive":"_pdb/A/modules/02233.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_column
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":2233,"record_offset":3772,"parent_record_offset":3744,"ancestor_record_offset":1100,"inlinee":14621,"source_line":692,"archive":"_pdb/A/modules/02233.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":2233,"record_offset":3692,"parent_record_offset":3256,"ancestor_record_offset":1100,"inlinee":14618,"source_line":834,"archive":"_pdb/A/modules/02233.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/query.rs
//     evidence={"module":2233,"record_offset":4232,"parent_record_offset":4208,"ancestor_record_offset":1100,"inlinee":21384,"source_line":846,"archive":"_pdb/A/modules/02233.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ptr::ThinSlicePtr::get
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ptr/src/lib.rs
//     evidence={"module":2233,"record_offset":4112,"parent_record_offset":4088,"ancestor_record_offset":1100,"inlinee":21393,"source_line":500,"archive":"_pdb/A/modules/02233.jsonl.gz"}
//   [pdb_inline_transitive] core::bool::impl$0::then
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\bool.rs
//     evidence={"module":2233,"record_offset":3508,"parent_record_offset":3476,"ancestor_record_offset":1100,"inlinee":4886,"source_line":59,"archive":"_pdb/A/modules/02233.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$24::ne
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":2233,"record_offset":3412,"parent_record_offset":3388,"ancestor_record_offset":1100,"inlinee":13915,"source_line":1471,"archive":"_pdb/A/modules/02233.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$54::lt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":2233,"record_offset":6724,"parent_record_offset":6696,"ancestor_record_offset":1100,"inlinee":4237,"source_line":1553,"archive":"_pdb/A/modules/02233.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":2233,"record_offset":2900,"parent_record_offset":2876,"ancestor_record_offset":1100,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/02233.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":2233,"record_offset":2444,"parent_record_offset":2420,"ancestor_record_offset":1100,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/02233.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":2233,"record_offset":2528,"parent_record_offset":2504,"ancestor_record_offset":1100,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/02233.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":2233,"record_offset":6844,"parent_record_offset":6612,"ancestor_record_offset":1100,"inlinee":4716,"source_line":329,"archive":"_pdb/A/modules/02233.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":2233,"record_offset":2468,"parent_record_offset":2444,"ancestor_record_offset":1100,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/02233.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::size_hint
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":2233,"record_offset":5808,"parent_record_offset":5772,"ancestor_record_offset":1100,"inlinee":6217,"source_line":112,"archive":"_pdb/A/modules/02233.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":2233,"record_offset":6696,"parent_record_offset":6668,"ancestor_record_offset":1100,"inlinee":4235,"source_line":753,"archive":"_pdb/A/modules/02233.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":2233,"record_offset":6668,"parent_record_offset":6612,"ancestor_record_offset":1100,"inlinee":4233,"source_line":844,"archive":"_pdb/A/modules/02233.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\collect.rs
//     evidence={"module":2233,"record_offset":5720,"parent_record_offset":5696,"ancestor_record_offset":1100,"inlinee":7553,"source_line":321,"archive":"_pdb/A/modules/02233.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":2233,"record_offset":6136,"parent_record_offset":5772,"ancestor_record_offset":1100,"inlinee":5220,"source_line":807,"archive":"_pdb/A/modules/02233.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":2233,"record_offset":6160,"parent_record_offset":6136,"ancestor_record_offset":1100,"inlinee":8548,"source_line":813,"archive":"_pdb/A/modules/02233.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::manually_drop::ManuallyDrop::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\manually_drop.rs
//     evidence={"module":2233,"record_offset":1640,"parent_record_offset":1608,"ancestor_record_offset":1100,"inlinee":4174,"source_line":70,"archive":"_pdb/A/modules/02233.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":2233,"record_offset":7780,"parent_record_offset":7752,"ancestor_record_offset":1100,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/02233.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":2233,"record_offset":2644,"parent_record_offset":2620,"ancestor_record_offset":1100,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/02233.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::impl$2::ne
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":2233,"record_offset":3388,"parent_record_offset":3300,"ancestor_record_offset":1100,"inlinee":21388,"source_line":136,"archive":"_pdb/A/modules/02233.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":2233,"record_offset":3892,"parent_record_offset":3868,"ancestor_record_offset":1100,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/02233.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":2233,"record_offset":7048,"parent_record_offset":7024,"ancestor_record_offset":1100,"inlinee":4711,"source_line":607,"archive":"_pdb/A/modules/02233.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":2233,"record_offset":3844,"parent_record_offset":3820,"ancestor_record_offset":1100,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/02233.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":2233,"record_offset":6284,"parent_record_offset":6260,"ancestor_record_offset":1100,"inlinee":4099,"source_line":1172,"archive":"_pdb/A/modules/02233.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::sub_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":2233,"record_offset":5880,"parent_record_offset":5856,"ancestor_record_offset":1100,"inlinee":6047,"source_line":809,"archive":"_pdb/A/modules/02233.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":2233,"record_offset":7652,"parent_record_offset":7628,"ancestor_record_offset":1100,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/02233.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":2233,"record_offset":1672,"parent_record_offset":1608,"ancestor_record_offset":1100,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/02233.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":2233,"record_offset":2812,"parent_record_offset":2788,"ancestor_record_offset":1100,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/02233.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":2233,"record_offset":2788,"parent_record_offset":2764,"ancestor_record_offset":1100,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/02233.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":2233,"record_offset":2040,"parent_record_offset":1984,"ancestor_record_offset":1100,"inlinee":6149,"source_line":971,"archive":"_pdb/A/modules/02233.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":2233,"record_offset":5856,"parent_record_offset":5832,"ancestor_record_offset":1100,"inlinee":6046,"source_line":953,"archive":"_pdb/A/modules/02233.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":2233,"record_offset":2012,"parent_record_offset":1984,"ancestor_record_offset":1100,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/02233.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":2233,"record_offset":2064,"parent_record_offset":2040,"ancestor_record_offset":1100,"inlinee":4136,"source_line":1246,"archive":"_pdb/A/modules/02233.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":2233,"record_offset":1568,"parent_record_offset":1464,"ancestor_record_offset":1100,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/02233.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":2233,"record_offset":3664,"parent_record_offset":3256,"ancestor_record_offset":1100,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/02233.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":2233,"record_offset":3332,"parent_record_offset":3300,"ancestor_record_offset":1100,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/02233.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":2233,"record_offset":3356,"parent_record_offset":3332,"ancestor_record_offset":1100,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/02233.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":2233,"record_offset":6532,"parent_record_offset":6488,"ancestor_record_offset":1100,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/02233.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::size_hint
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":2233,"record_offset":5832,"parent_record_offset":5808,"ancestor_record_offset":1100,"inlinee":6061,"source_line":171,"archive":"_pdb/A/modules/02233.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::impl$9::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":2233,"record_offset":2924,"parent_record_offset":2900,"ancestor_record_offset":1100,"inlinee":8043,"source_line":65,"archive":"_pdb/A/modules/02233.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::contains
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":2233,"record_offset":3476,"parent_record_offset":3256,"ancestor_record_offset":1100,"inlinee":4884,"source_line":392,"archive":"_pdb/A/modules/02233.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::contains_unchecked
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":2233,"record_offset":3560,"parent_record_offset":3536,"ancestor_record_offset":1100,"inlinee":4889,"source_line":408,"archive":"_pdb/A/modules/02233.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::get_unchecked
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":2233,"record_offset":3584,"parent_record_offset":3560,"ancestor_record_offset":1100,"inlinee":4890,"source_line":171,"archive":"_pdb/A/modules/02233.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::impl$2::contains::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":2233,"record_offset":3536,"parent_record_offset":3508,"ancestor_record_offset":1100,"inlinee":4888,"source_line":395,"archive":"_pdb/A/modules/02233.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\lib.rs
//     evidence={"module":2233,"record_offset":2876,"parent_record_offset":2852,"ancestor_record_offset":1100,"inlinee":4854,"source_line":170,"archive":"_pdb/A/modules/02233.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":2233,"record_offset":2204,"parent_record_offset":2168,"ancestor_record_offset":1100,"inlinee":5561,"source_line":1308,"archive":"_pdb/A/modules/02233.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":2233,"record_offset":2228,"parent_record_offset":2204,"ancestor_record_offset":1100,"inlinee":5562,"source_line":1351,"archive":"_pdb/A/modules/02233.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":2233,"record_offset":2852,"parent_record_offset":2712,"ancestor_record_offset":1100,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/02233.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":2233,"record_offset":2260,"parent_record_offset":2228,"ancestor_record_offset":1100,"inlinee":5617,"source_line":257,"archive":"_pdb/A/modules/02233.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2233,"record_offset":2764,"parent_record_offset":2740,"ancestor_record_offset":1100,"inlinee":5640,"source_line":513,"archive":"_pdb/A/modules/02233.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2233,"record_offset":2740,"parent_record_offset":2712,"ancestor_record_offset":1100,"inlinee":5639,"source_line":682,"archive":"_pdb/A/modules/02233.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2233,"record_offset":3120,"parent_record_offset":3096,"ancestor_record_offset":1100,"inlinee":5638,"source_line":355,"archive":"_pdb/A/modules/02233.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2233,"record_offset":3060,"parent_record_offset":2340,"ancestor_record_offset":1100,"inlinee":5645,"source_line":175,"archive":"_pdb/A/modules/02233.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2233,"record_offset":3096,"parent_record_offset":2312,"ancestor_record_offset":1100,"inlinee":5636,"source_line":997,"archive":"_pdb/A/modules/02233.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2233,"record_offset":2312,"parent_record_offset":2288,"ancestor_record_offset":1100,"inlinee":5620,"source_line":1456,"archive":"_pdb/A/modules/02233.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2233,"record_offset":2288,"parent_record_offset":2228,"ancestor_record_offset":1100,"inlinee":7330,"source_line":1479,"archive":"_pdb/A/modules/02233.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2233,"record_offset":2340,"parent_record_offset":2312,"ancestor_record_offset":1100,"inlinee":5622,"source_line":2168,"archive":"_pdb/A/modules/02233.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":2233,"record_offset":2592,"parent_record_offset":2560,"ancestor_record_offset":1100,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/02233.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":2233,"record_offset":2620,"parent_record_offset":2592,"ancestor_record_offset":1100,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/02233.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::remove_lowest_bit
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":2233,"record_offset":2680,"parent_record_offset":2560,"ancestor_record_offset":1100,"inlinee":5633,"source_line":37,"archive":"_pdb/A/modules/02233.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":2233,"record_offset":2560,"parent_record_offset":2340,"ancestor_record_offset":1100,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/02233.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::h2
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2233,"record_offset":2392,"parent_record_offset":2340,"ancestor_record_offset":1100,"inlinee":5624,"source_line":150,"archive":"_pdb/A/modules/02233.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2233,"record_offset":2712,"parent_record_offset":2340,"ancestor_record_offset":1100,"inlinee":5635,"source_line":1466,"archive":"_pdb/A/modules/02233.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":2233,"record_offset":2420,"parent_record_offset":2340,"ancestor_record_offset":1100,"inlinee":5626,"source_line":52,"archive":"_pdb/A/modules/02233.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_byte
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":2233,"record_offset":2504,"parent_record_offset":2340,"ancestor_record_offset":1100,"inlinee":5627,"source_line":79,"archive":"_pdb/A/modules/02233.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":2233,"record_offset":2968,"parent_record_offset":2340,"ancestor_record_offset":1100,"inlinee":5643,"source_line":97,"archive":"_pdb/A/modules/02233.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"nonmax","version":"0.5.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":2233,"record_offset":3980,"parent_record_offset":3956,"ancestor_record_offset":1100,"inlinee":4372,"source_line":153,"archive":"_pdb/A/modules/02233.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":2233,"record_offset":2168,"parent_record_offset":2136,"ancestor_record_offset":1100,"inlinee":7329,"source_line":877,"archive":"_pdb/A/modules/02233.jsonl.gz"}
//   [pdb_inline_transitive] system_color::ui_color_wall::ui_color_wall::closure$1
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/color/src/ui_color_wall.rs
//     evidence={"module":2233,"record_offset":4208,"parent_record_offset":4180,"ancestor_record_offset":1100,"inlinee":81102,"source_line":95,"archive":"_pdb/A/modules/02233.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_color::ui_color_wall::ui_color_wall::closure$0
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=3
// 源码行: 56
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xBE1F
// 首条位置: {"module":891,"inlinee":48671}
// 编译实例 RVAs: 0x9afac0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: ui_color_wall.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6116217","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378865856,"raw_hex":"e887aca2ff","size":5,"soft_fail":false,"target_va":5372749696,"va":5378865908,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: ui_color_wall.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6116343","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378865856,"raw_hex":"e809aca2ff","size":5,"soft_fail":false,"target_va":5372749712,"va":5378866050,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t24112131","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378865856,"raw_hex":"e803ec6f01","size":5,"soft_fail":false,"target_va":5402978144,"va":5378866008,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_color::ui_color_wall::color_cursor_icon
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t11511850","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378865856,"raw_hex":"e82aa8af00","size":5,"soft_fail":false,"target_va":5390377808,"va":5378865953,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":891,"record_offset":40348,"parent_record_offset":40316,"ancestor_record_offset":40316,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/00891.jsonl.gz"}
//   [pdb_inline_direct] alloc::slice::impl$0::into_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":891,"record_offset":40492,"parent_record_offset":40316,"ancestor_record_offset":40316,"inlinee":4441,"source_line":461,"archive":"_pdb/A/modules/00891.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":891,"record_offset":40576,"parent_record_offset":40316,"ancestor_record_offset":40316,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00891.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":891,"record_offset":40400,"parent_record_offset":40376,"ancestor_record_offset":40316,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00891.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":891,"record_offset":40424,"parent_record_offset":40400,"ancestor_record_offset":40316,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00891.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":891,"record_offset":40648,"parent_record_offset":40624,"ancestor_record_offset":40316,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00891.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":891,"record_offset":40376,"parent_record_offset":40348,"ancestor_record_offset":40316,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00891.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":891,"record_offset":40624,"parent_record_offset":40600,"ancestor_record_offset":40316,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00891.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":891,"record_offset":40600,"parent_record_offset":40576,"ancestor_record_offset":40316,"inlinee":4113,"source_line":1240,"archive":"_pdb/A/modules/00891.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::into_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":891,"record_offset":40516,"parent_record_offset":40492,"ancestor_record_offset":40316,"inlinee":4442,"source_line":100,"archive":"_pdb/A/modules/00891.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::from_raw_parts_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":891,"record_offset":40540,"parent_record_offset":40516,"ancestor_record_offset":40316,"inlinee":4443,"source_line":819,"archive":"_pdb/A/modules/00891.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":891,"record_offset":40448,"parent_record_offset":40424,"ancestor_record_offset":40316,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00891.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_color::ui_color_wall::ui_color_wall::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 95
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x13CCE
// 首条位置: {"module":2233,"inlinee":81102}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] bevy_ecs::system::query::Query::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/query.rs
//     evidence={"module":2233,"record_offset":4232,"parent_record_offset":4208,"ancestor_record_offset":4208,"inlinee":21384,"source_line":846,"archive":"_pdb/A/modules/02233.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::impl$24::eq
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/archetype.rs
//     evidence={"module":2233,"record_offset":4444,"parent_record_offset":4300,"ancestor_record_offset":4208,"inlinee":21390,"source_line":74,"archive":"_pdb/A/modules/02233.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::Entities::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/entity/mod.rs
//     evidence={"module":2233,"record_offset":4300,"parent_record_offset":4256,"ancestor_record_offset":4208,"inlinee":21387,"source_line":739,"archive":"_pdb/A/modules/02233.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$21::fetch
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":2233,"record_offset":5508,"parent_record_offset":5484,"ancestor_record_offset":4208,"inlinee":14627,"source_line":963,"archive":"_pdb/A/modules/02233.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$21::set_archetype
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":2233,"record_offset":4744,"parent_record_offset":4720,"ancestor_record_offset":4208,"inlinee":21391,"source_line":933,"archive":"_pdb/A/modules/02233.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$21::set_table
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":2233,"record_offset":4768,"parent_record_offset":4744,"ancestor_record_offset":4208,"inlinee":14620,"source_line":948,"archive":"_pdb/A/modules/02233.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::state::QueryState::get_unchecked_manual
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/state.rs
//     evidence={"module":2233,"record_offset":4256,"parent_record_offset":4232,"ancestor_record_offset":4208,"inlinee":21385,"source_line":859,"archive":"_pdb/A/modules/02233.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$2::fetch
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/world_query.rs
//     evidence={"module":2233,"record_offset":5484,"parent_record_offset":4256,"ancestor_record_offset":4208,"inlinee":14626,"source_line":201,"archive":"_pdb/A/modules/02233.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$2::set_archetype
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/world_query.rs
//     evidence={"module":2233,"record_offset":4720,"parent_record_offset":4256,"ancestor_record_offset":4208,"inlinee":21665,"source_line":179,"archive":"_pdb/A/modules/02233.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_slice
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":2233,"record_offset":5072,"parent_record_offset":5048,"ancestor_record_offset":4208,"inlinee":14643,"source_line":410,"archive":"_pdb/A/modules/02233.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseArray::get
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":2233,"record_offset":4844,"parent_record_offset":4820,"ancestor_record_offset":4208,"inlinee":6338,"source_line":56,"archive":"_pdb/A/modules/02233.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":2233,"record_offset":4820,"parent_record_offset":4796,"ancestor_record_offset":4208,"inlinee":6337,"source_line":369,"archive":"_pdb/A/modules/02233.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$11::get::closure$0
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":2233,"record_offset":4892,"parent_record_offset":4868,"ancestor_record_offset":4208,"inlinee":6340,"source_line":58,"archive":"_pdb/A/modules/02233.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$16::get::closure$0
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":2233,"record_offset":4980,"parent_record_offset":4956,"ancestor_record_offset":4208,"inlinee":6342,"source_line":370,"archive":"_pdb/A/modules/02233.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Column::get_data_slice
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":2233,"record_offset":5048,"parent_record_offset":4768,"ancestor_record_offset":4208,"inlinee":14642,"source_line":321,"archive":"_pdb/A/modules/02233.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_column
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":2233,"record_offset":4796,"parent_record_offset":4768,"ancestor_record_offset":4208,"inlinee":14621,"source_line":692,"archive":"_pdb/A/modules/02233.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":2233,"record_offset":4692,"parent_record_offset":4256,"ancestor_record_offset":4208,"inlinee":14618,"source_line":834,"archive":"_pdb/A/modules/02233.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ptr::ThinSlicePtr::get
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ptr/src/lib.rs
//     evidence={"module":2233,"record_offset":5532,"parent_record_offset":5508,"ancestor_record_offset":4208,"inlinee":21393,"source_line":500,"archive":"_pdb/A/modules/02233.jsonl.gz"}
//   [pdb_inline_transitive] core::bool::impl$0::then
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\bool.rs
//     evidence={"module":2233,"record_offset":4508,"parent_record_offset":4476,"ancestor_record_offset":4208,"inlinee":4886,"source_line":59,"archive":"_pdb/A/modules/02233.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$24::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":2233,"record_offset":4412,"parent_record_offset":4388,"ancestor_record_offset":4208,"inlinee":13915,"source_line":1471,"archive":"_pdb/A/modules/02233.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::impl$2::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":2233,"record_offset":4388,"parent_record_offset":4300,"ancestor_record_offset":4208,"inlinee":21388,"source_line":136,"archive":"_pdb/A/modules/02233.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":2233,"record_offset":4916,"parent_record_offset":4892,"ancestor_record_offset":4208,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/02233.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":2233,"record_offset":4868,"parent_record_offset":4844,"ancestor_record_offset":4208,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/02233.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":2233,"record_offset":4616,"parent_record_offset":4584,"ancestor_record_offset":4208,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/02233.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":2233,"record_offset":4664,"parent_record_offset":4256,"ancestor_record_offset":4208,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/02233.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":2233,"record_offset":4332,"parent_record_offset":4300,"ancestor_record_offset":4208,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/02233.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":2233,"record_offset":4356,"parent_record_offset":4332,"ancestor_record_offset":4208,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/02233.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::contains
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":2233,"record_offset":4476,"parent_record_offset":4256,"ancestor_record_offset":4208,"inlinee":4884,"source_line":392,"archive":"_pdb/A/modules/02233.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::contains_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":2233,"record_offset":4560,"parent_record_offset":4536,"ancestor_record_offset":4208,"inlinee":4889,"source_line":408,"archive":"_pdb/A/modules/02233.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::get_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":2233,"record_offset":4584,"parent_record_offset":4560,"ancestor_record_offset":4208,"inlinee":4890,"source_line":171,"archive":"_pdb/A/modules/02233.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::impl$2::contains::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":2233,"record_offset":4536,"parent_record_offset":4508,"ancestor_record_offset":4208,"inlinee":4888,"source_line":395,"archive":"_pdb/A/modules/02233.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"nonmax","version":"0.5.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":2233,"record_offset":5004,"parent_record_offset":4980,"ancestor_record_offset":4208,"inlinee":4372,"source_line":153,"archive":"_pdb/A/modules/02233.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_color::ui_color_wall::ui_color_wall::closure$2
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=3
// 源码行: 113
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xBE1E
// 首条位置: {"module":891,"inlinee":48670}
// 编译实例 RVAs: 0x9afba0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: ui_color_wall.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6116441","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378866080,"raw_hex":"e8a7aba2ff","size":5,"soft_fail":false,"target_va":5372749696,"va":5378866132,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: ui_color_wall.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6116567","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378866080,"raw_hex":"e829aba2ff","size":5,"soft_fail":false,"target_va":5372749712,"va":5378866274,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t24111907","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378866080,"raw_hex":"e823eb6f01","size":5,"soft_fail":false,"target_va":5402978144,"va":5378866232,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_color::ui_color_wall::color_cursor_icon
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t11511626","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378866080,"raw_hex":"e84aa7af00","size":5,"soft_fail":false,"target_va":5390377808,"va":5378866177,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":891,"record_offset":39560,"parent_record_offset":39528,"ancestor_record_offset":39528,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/00891.jsonl.gz"}
//   [pdb_inline_direct] alloc::slice::impl$0::into_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":891,"record_offset":39704,"parent_record_offset":39528,"ancestor_record_offset":39528,"inlinee":4441,"source_line":461,"archive":"_pdb/A/modules/00891.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":891,"record_offset":39788,"parent_record_offset":39528,"ancestor_record_offset":39528,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00891.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":891,"record_offset":39612,"parent_record_offset":39588,"ancestor_record_offset":39528,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00891.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":891,"record_offset":39636,"parent_record_offset":39612,"ancestor_record_offset":39528,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00891.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":891,"record_offset":39860,"parent_record_offset":39836,"ancestor_record_offset":39528,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00891.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":891,"record_offset":39588,"parent_record_offset":39560,"ancestor_record_offset":39528,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00891.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":891,"record_offset":39836,"parent_record_offset":39812,"ancestor_record_offset":39528,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00891.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":891,"record_offset":39812,"parent_record_offset":39788,"ancestor_record_offset":39528,"inlinee":4113,"source_line":1240,"archive":"_pdb/A/modules/00891.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::into_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":891,"record_offset":39728,"parent_record_offset":39704,"ancestor_record_offset":39528,"inlinee":4442,"source_line":100,"archive":"_pdb/A/modules/00891.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::from_raw_parts_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":891,"record_offset":39752,"parent_record_offset":39728,"ancestor_record_offset":39528,"inlinee":4443,"source_line":819,"archive":"_pdb/A/modules/00891.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":891,"record_offset":39660,"parent_record_offset":39636,"ancestor_record_offset":39528,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00891.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_color::ui_color_wall::color_cursor_icon
// 记录形式: compiled_procedure; instances=1
// 源码行: 153
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":2233,"record_offset":8648,"rva":21668688,"va":5390377808}
// 编译实例 RVAs: 0x14aa350
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: ui_color_wall.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-17628366","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390377808,"raw_hex":"e83203f3fe","size":5,"soft_fail":false,"target_va":5372749696,"va":5390378057,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t12599804","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390377808,"raw_hex":"e8fc41c000","size":5,"soft_fail":false,"target_va":5402978144,"va":5390378335,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve::do_reserve_and_handle
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t12540771","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390377808,"raw_hex":"e8635bbf00","size":5,"soft_fail":false,"target_va":5402919072,"va":5390378296,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::system::commands::EntityCommands::insert
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t6412041","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390377808,"raw_hex":"e809d76100","size":5,"soft_fail":false,"target_va":5396790208,"va":5390378162,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::world::command_queue::CommandQueue::get_raw
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-13131459","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390377808,"raw_hex":"e83da137ff","size":5,"soft_fail":false,"target_va":5377246528,"va":5390377982,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::animation_duration
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-15021225","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390377808,"raw_hex":"e857cb1aff","size":5,"soft_fail":false,"target_va":5375356704,"va":5390377924,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::build
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-15021057","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390377808,"raw_hex":"e8ffcb1aff","size":5,"soft_fail":false,"target_va":5375356896,"va":5390377948,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::new
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-15021923","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390377808,"raw_hex":"e89dc81aff","size":5,"soft_fail":false,"target_va":5375355984,"va":5390377902,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::system::commands::EntityCommands::insert
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs
//     evidence={"module":2233,"record_offset":8784,"parent_record_offset":8648,"ancestor_record_offset":8648,"inlinee":14636,"source_line":945,"archive":"_pdb/A/modules/02233.jsonl.gz"}
//   [pdb_inline_direct] country_core::components::Ui2dImage::new
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/components/mod.rs
//     evidence={"module":2233,"record_offset":8932,"parent_record_offset":8648,"ancestor_record_offset":8648,"inlinee":26374,"source_line":166,"archive":"_pdb/A/modules/02233.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":2233,"record_offset":9308,"parent_record_offset":9284,"ancestor_record_offset":8648,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/02233.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":2233,"record_offset":9332,"parent_record_offset":9308,"ancestor_record_offset":8648,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/02233.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":2233,"record_offset":9284,"parent_record_offset":9256,"ancestor_record_offset":8648,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/02233.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":2233,"record_offset":9256,"parent_record_offset":9228,"ancestor_record_offset":8648,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/02233.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":2233,"record_offset":9228,"parent_record_offset":9200,"ancestor_record_offset":8648,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/02233.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":2233,"record_offset":9404,"parent_record_offset":9228,"ancestor_record_offset":8648,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/02233.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":2233,"record_offset":9164,"parent_record_offset":9136,"ancestor_record_offset":8648,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/02233.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":2233,"record_offset":9136,"parent_record_offset":9108,"ancestor_record_offset":8648,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/02233.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":2233,"record_offset":9080,"parent_record_offset":9052,"ancestor_record_offset":8648,"inlinee":4415,"source_line":412,"archive":"_pdb/A/modules/02233.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":2233,"record_offset":9108,"parent_record_offset":9080,"ancestor_record_offset":8648,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/02233.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":2233,"record_offset":9052,"parent_record_offset":9024,"ancestor_record_offset":8648,"inlinee":4413,"source_line":822,"archive":"_pdb/A/modules/02233.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\str.rs
//     evidence={"module":2233,"record_offset":9024,"parent_record_offset":8996,"ancestor_record_offset":8648,"inlinee":4411,"source_line":210,"archive":"_pdb/A/modules/02233.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$44::from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":2233,"record_offset":8996,"parent_record_offset":8968,"ancestor_record_offset":8648,"inlinee":4938,"source_line":2705,"archive":"_pdb/A/modules/02233.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":2233,"record_offset":9200,"parent_record_offset":9164,"ancestor_record_offset":8648,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/02233.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::add
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs
//     evidence={"module":2233,"record_offset":8832,"parent_record_offset":8808,"ancestor_record_offset":8648,"inlinee":14638,"source_line":748,"archive":"_pdb/A/modules/02233.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::push
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs
//     evidence={"module":2233,"record_offset":8860,"parent_record_offset":8832,"ancestor_record_offset":8648,"inlinee":12140,"source_line":501,"archive":"_pdb/A/modules/02233.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::EntityCommands::add
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs
//     evidence={"module":2233,"record_offset":8808,"parent_record_offset":8784,"ancestor_record_offset":8648,"inlinee":14637,"source_line":1100,"archive":"_pdb/A/modules/02233.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::command_queue::CommandQueue::push
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/command_queue.rs
//     evidence={"module":2233,"record_offset":8888,"parent_record_offset":8860,"ancestor_record_offset":8648,"inlinee":12149,"source_line":78,"archive":"_pdb/A/modules/02233.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":2233,"record_offset":8968,"parent_record_offset":8932,"ancestor_record_offset":8648,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/02233.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":2233,"record_offset":9464,"parent_record_offset":9440,"ancestor_record_offset":8648,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/02233.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":2233,"record_offset":9440,"parent_record_offset":9164,"ancestor_record_offset":8648,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/02233.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":2233,"record_offset":9360,"parent_record_offset":9332,"ancestor_record_offset":8648,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/02233.jsonl.gz"}
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
// 完整结构及原始类型记录: ui_color_wall.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: ui_color_wall.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: ui_color_wall.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 18; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::WallEntities>,bevy_ecs::system::query::Query<ref$<country_core::resources::walls::wall_trigger_volume::WallTriggerVolume>,tuple$<> >,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::event::EventWriter<system_color::show_color_menu::ShowColorMenuCmd>,bevy_ecs::event::EventWriter<system_color::show_color_menu::HideColorMenuCmd>,bevy_ecs::system::query::Query<ref$<system_color::radial_menu::ColorRadialMenu>,tuple$<> >,bevy_ecs::event::EventWriter<country_core::systems::wall::wall_outline::OutlineWallCmd>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >", "bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::WallEntities>", "bevy_ecs::system::query::Query<ref$<country_core::resources::walls::wall_trigger_volume::WallTriggerVolume>,tuple$<> >", "bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume> >,tuple$<> >", "bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>", "bevy_ecs::event::EventWriter<system_color::show_color_menu::ShowColorMenuCmd>", "bevy_ecs::event::EventWriter<system_color::show_color_menu::HideColorMenuCmd>", "bevy_ecs::system::query::Query<ref$<system_color::radial_menu::ColorRadialMenu>,tuple$<> >", "bevy_ecs::event::EventWriter<country_core::systems::wall::wall_outline::OutlineWallCmd>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::WallEntities>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<ref$<country_core::resources::walls::wall_trigger_volume::WallTriggerVolume>,tuple$<> >
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume> >,tuple$<> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<system_color::show_color_menu::ShowColorMenuCmd>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<system_color::show_color_menu::HideColorMenuCmd>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<ref$<system_color::radial_menu::ColorRadialMenu>,tuple$<> >
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::systems::wall::wall_outline::OutlineWallCmd>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation>,ref$<country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::event::EventWriter<system_color::show_color_menu::ShowColorMenuCmd>,bevy_ecs::event::EventWriter<system_color::show_color_menu::HideColorMenuCmd>,bevy_ecs::system::query::Query<ref$<system_color::radial_menu::ColorRadialMenu>,tuple$<> >,bevy_ecs::event::EventWriter<country_core::systems::wall::wall_outline::OutlineWallCmd>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >", "bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>", "bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation>,ref$<country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume> >,tuple$<> >", "bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>", "bevy_ecs::event::EventWriter<system_color::show_color_menu::ShowColorMenuCmd>", "bevy_ecs::event::EventWriter<system_color::show_color_menu::HideColorMenuCmd>", "bevy_ecs::system::query::Query<ref$<system_color::radial_menu::ColorRadialMenu>,tuple$<> >", "bevy_ecs::event::EventWriter<country_core::systems::wall::wall_outline::OutlineWallCmd>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation>,ref$<country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume> >,tuple$<> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<system_color::show_color_menu::ShowColorMenuCmd>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<system_color::show_color_menu::HideColorMenuCmd>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<ref$<system_color::radial_menu::ColorRadialMenu>,tuple$<> >
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::systems::wall::wall_outline::OutlineWallCmd>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::EventWriter<system_color::show_color_menu::ShowColorMenuCmd>,bevy_ecs::event::EventWriter<system_color::show_color_menu::HideColorMenuCmd>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >", "bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>", "bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>", "bevy_ecs::event::EventWriter<system_color::show_color_menu::ShowColorMenuCmd>", "bevy_ecs::event::EventWriter<system_color::show_color_menu::HideColorMenuCmd>", "bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>", "bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>", "country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>", "bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<system_color::show_color_menu::ShowColorMenuCmd>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<system_color::show_color_menu::HideColorMenuCmd>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::walls::wooden_gates::Gates>,bevy_ecs::system::query::Query<ref$<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh> >,bevy_ecs::query::filter::With<country_core::components::WoodenGate> >,bevy_ecs::system::query::Query<ref$<country_core::components::WoodenGate>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::event::EventWriter<system_color::show_color_menu::ShowColorMenuCmd>,bevy_ecs::event::EventWriter<system_color::show_color_menu::HideColorMenuCmd>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >", "bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>", "bevy_ecs::change_detection::Res<country_core::resources::walls::wooden_gates::Gates>", "bevy_ecs::system::query::Query<ref$<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh> >,bevy_ecs::query::filter::With<country_core::components::WoodenGate> >", "bevy_ecs::system::query::Query<ref$<country_core::components::WoodenGate>,tuple$<> >", "bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>", "bevy_ecs::event::EventWriter<system_color::show_color_menu::ShowColorMenuCmd>", "bevy_ecs::event::EventWriter<system_color::show_color_menu::HideColorMenuCmd>", "bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::wooden_gates::Gates>
// [候选系统接口] query_with_filter: bevy_ecs::query::filter::With<country_core::components::WoodenGate>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<ref$<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh> >,bevy_ecs::query::filter::With<country_core::components::WoodenGate> >
// [候选系统接口] component_query: bevy_ecs::system::query::Query<ref$<country_core::components::WoodenGate>,tuple$<> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<system_color::show_color_menu::ShowColorMenuCmd>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<system_color::show_color_menu::HideColorMenuCmd>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::event::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::event::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::system::query::Query<ref$<system_color::radial_menu::ColorRadialMenu>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::event::EventWriter<system_color::show_color_menu::HideColorMenuCmd>)
// 候选参数类型: ["bevy_ecs::event::EventReader<country_core::resources::events::ModeJustChanged>", "bevy_ecs::event::EventReader<country_core::resources::gui::EscapeMenuJustOpened>", "bevy_ecs::system::query::Query<ref$<system_color::radial_menu::ColorRadialMenu>,tuple$<> >", "bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>", "bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>", "bevy_ecs::event::EventWriter<system_color::show_color_menu::HideColorMenuCmd>"]
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::resources::events::ModeJustChanged>
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::resources::gui::EscapeMenuJustOpened>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<ref$<system_color::radial_menu::ColorRadialMenu>,tuple$<> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<system_color::show_color_menu::HideColorMenuCmd>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::event::EventReader<system_color::show_color_menu::HideColorMenuCmd>,bevy_ecs::event::EventWriter<enum2$<country_core::systems::ui::resources::widget_params::PlayUiSound> >,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<system_color::radial_menu::ColorRadialMenu> >,tuple$<> >,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::color_icon::CustomizationIcon> >,tuple$<> >,bevy_ecs::system::commands::Commands,bevy_ecs::event::EventWriter<country_core::resources::color_id::SetWallColorIdCmd>,bevy_ecs::event::EventWriter<system_color::set_roof_color_id::SetRoofColorIdCmd>,bevy_ecs::event::EventWriter<system_color::set_decorator_color_id::SetDecoratorColorIdCmd>,bevy_ecs::event::EventWriter<country_core::resources::color_id::SetGateColorIdCmd>,bevy_ecs::event::EventWriter<country_core::resources::color_id::SetHalftimberColorIdCmd>,bevy_ecs::event::EventWriter<country_core::systems::wall::set_wall_style::SetWallStyleCmd>,bevy_ecs::event::EventWriter<country_core::resources::walls::decorator::SetFlagConnectionCmd>)
// 候选参数类型: ["bevy_ecs::event::EventReader<system_color::show_color_menu::HideColorMenuCmd>", "bevy_ecs::event::EventWriter<enum2$<country_core::systems::ui::resources::widget_params::PlayUiSound> >", "bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<system_color::radial_menu::ColorRadialMenu> >,tuple$<> >", "bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::color_icon::CustomizationIcon> >,tuple$<> >", "bevy_ecs::system::commands::Commands", "bevy_ecs::event::EventWriter<country_core::resources::color_id::SetWallColorIdCmd>", "bevy_ecs::event::EventWriter<system_color::set_roof_color_id::SetRoofColorIdCmd>", "bevy_ecs::event::EventWriter<system_color::set_decorator_color_id::SetDecoratorColorIdCmd>", "bevy_ecs::event::EventWriter<country_core::resources::color_id::SetGateColorIdCmd>", "bevy_ecs::event::EventWriter<country_core::resources::color_id::SetHalftimberColorIdCmd>", "bevy_ecs::event::EventWriter<country_core::systems::wall::set_wall_style::SetWallStyleCmd>", "bevy_ecs::event::EventWriter<country_core::resources::walls::decorator::SetFlagConnectionCmd>"]
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<system_color::show_color_menu::HideColorMenuCmd>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<enum2$<country_core::systems::ui::resources::widget_params::PlayUiSound> >
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<system_color::radial_menu::ColorRadialMenu> >,tuple$<> >
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::color_icon::CustomizationIcon> >,tuple$<> >
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::resources::color_id::SetWallColorIdCmd>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<system_color::set_roof_color_id::SetRoofColorIdCmd>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<system_color::set_decorator_color_id::SetDecoratorColorIdCmd>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::resources::color_id::SetGateColorIdCmd>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::resources::color_id::SetHalftimberColorIdCmd>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::systems::wall::set_wall_style::SetWallStyleCmd>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::resources::walls::decorator::SetFlagConnectionCmd>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::color_icon::CustomizationIcon>,ref$<country_core::resources::color_icon::CustomizationIconMarkedForDeletion> >,tuple$<> >,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::color_icon::CustomizationIcon>,ref$<country_core::resources::color_icon::CustomizationIconMarkedForDeletion> >,tuple$<> >", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::color_icon::CustomizationIcon>,ref$<country_core::resources::color_icon::CustomizationIconMarkedForDeletion> >,tuple$<> >
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::event::EventReader<system_color::show_color_menu::ShowColorMenuCmd>,bevy_ecs::change_detection::Res<country_core::resources::color_id::WallColorIds>,bevy_ecs::change_detection::Res<country_core::resources::color_id::GateColorIds>,bevy_ecs::change_detection::Res<country_core::resources::color_id::HalftimberColorIds>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::event::EventReader<system_color::show_color_menu::ShowColorMenuCmd>", "bevy_ecs::change_detection::Res<country_core::resources::color_id::WallColorIds>", "bevy_ecs::change_detection::Res<country_core::resources::color_id::GateColorIds>", "bevy_ecs::change_detection::Res<country_core::resources::color_id::HalftimberColorIds>", "bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >", "bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>", "bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>", "country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<system_color::show_color_menu::ShowColorMenuCmd>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::color_id::WallColorIds>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::color_id::GateColorIds>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::color_id::HalftimberColorIds>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: system_color::ui_color_wall::color_cursor_icon
// 信息 ID: 3ef33b48faac3c019b07699b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 5 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] core::ops::function::FnOnce::call_once<system_color::ui_color_wall::ui_color_wall::closure_env$2,tuple$<ref$<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,ref$<country_core::asset_libraries::shader_library::AssetShaderLibrary>,ref_mut$<bevy_ecs::system::commands::Commands> > > | core::ops::function::FnOnce::call_once<system_color::ui_color_roof::ui_color_roof::closure_env$0,tuple$<ref$<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,ref$<country_core::asset_libraries::shader_library::AssetShaderLibrary>,ref_mut$<bevy_ecs::system::commands::Commands> > > | core::ops::function::FnOnce::call_once<system_color::ui_color_gates::ui_color_gates::closure_env$1,tuple$<ref$<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,ref$<country_core::asset_libraries::shader_library::AssetShaderLibrary>,ref_mut$<bevy_ecs::system::commands::Commands> > > @ 0x9ae418 -> 0x14aa350 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::FnOnce::call_once<system_color::ui_color_wall::ui_color_wall::closure_env$0,tuple$<ref$<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,ref$<country_core::asset_libraries::shader_library::AssetShaderLibrary>,ref_mut$<bevy_ecs::system::commands::Commands> > > @ 0x9ae518 -> 0x14aa350 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_color::ui_color_wall::ui_color_wall::closure$0 @ 0x9afb21 -> 0x14aa350 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_color::ui_color_wall::ui_color_wall::closure$2 | system_color::ui_color_roof::ui_color_roof::closure$0 | system_color::ui_color_gates::ui_color_gates::closure$1 @ 0x9afc01 -> 0x14aa350 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::FnOnce::call_once<system_color::ui_color_decorator::ui_color_windows_and_flags::closure_env$1,tuple$<ref$<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,ref$<country_core::asset_libraries::shader_library::AssetShaderLibrary>,ref_mut$<bevy_ecs::system::commands::Commands> > > | system_color::ui_color_decorator::ui_color_windows_and_flags::closure$1 @ 0x14ac0b3 -> 0x14aa350 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_color::ui_color_wall::ui_color_wall
// 信息 ID: c8b72d72573a5a4fb75e5a2e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] bc6488178942790833d6476e: void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::WallEntities>,bevy_ecs::system::query::Query<ref$<country_core::resources::walls::wall_trigger_volume::WallTriggerVolume>,tuple$<> >,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::event::EventWriter<system_color::show_color_menu::ShowColorMenuCmd>,bevy_ecs::event::EventWriter<system_color::show_color_menu::HideColorMenuCmd>,bevy_ecs::system::query::Query<ref$<system_color::radial_menu::ColorRadialMenu>,tuple$<> >,bevy_ecs::event::EventWriter<country_core::systems::wall::wall_outline::OutlineWallCmd>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::WallEntities>,bevy_ecs::system::query::Query<ref$<country_core::resources::walls::wall_trigger_volume::WallTriggerVolume>,tuple$<> >,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::event::EventWriter<system_color::show_color_menu::ShowColorMenuCmd>,bevy_ecs::event::EventWriter<system_color::show_color_menu::HideColorMenuCmd>,bevy_ecs::system::query::Query<ref$<system_color::radial_menu::ColorRadialMenu>,tuple$<> >,bevy_ecs::event::EventWriter<country_core::systems::wall::wall_outline::OutlineWallCmd>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::WallEntities>,bevy_ecs::system::query::Query<ref$<country_core::resources::walls::wall_trigger_volume::WallTriggerVolume>,tuple$<> >,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::event::EventWriter<system_color::show_color_menu::ShowColorMenuCmd>,bevy_ecs::event::EventWriter<system_color::show_color_menu::HideColorMenuCmd>,bevy_ecs::system::query::Query<ref$<system_color::radial_menu::ColorRadialMenu>,tuple$<> >,bevy_ecs::event::EventWriter<country_core::systems::wall::wall_outline::OutlineWallCmd>)> @ 0x9ae09e -> 0x14a94b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::WallEntities>,bevy_ecs::system::query::Query<ref$<country_core::resources::walls::wall_trigger_volume::WallTriggerVolume>,tuple$<> >,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::event::EventWriter<system_color::show_color_menu::ShowColorMenuCmd>,bevy_ecs::event::EventWriter<system_color::show_color_menu::HideColorMenuCmd>,bevy_ecs::system::query::Query<ref$<system_color::radial_menu::ColorRadialMenu>,tuple$<> >,bevy_ecs::event::EventWriter<country_core::systems::wall::wall_outline::OutlineWallCmd>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::WallEntities>,bevy_ecs::system::query::Query<ref$<country_core::resources::walls::wall_trigger_volume::WallTriggerVolume>,tuple$<> >,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::event::EventWriter<system_color::show_color_menu::ShowColorMenuCmd>,bevy_ecs::event::EventWriter<system_color::show_color_menu::HideColorMenuCmd>,bevy_ecs::system::query::Query<ref$<system_color::radial_menu::ColorRadialMenu>,tuple$<> >,bevy_ecs::event::EventWriter<country_core::systems::wall::wall_outline::OutlineWallCmd>)> > @ 0x149ccb0 -> 0x14a94b0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_color::ui_color_wall::ui_color_wall::closure$0
// 信息 ID: 4e62acd8d569b48999a70300
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_color::ui_color_wall::ui_color_wall", "captures": "unknown"}
// [闭包环境类型引用] system_color::ui_color_wall::ui_color_wall::closure_env$0 <- core::ops::function::FnOnce::call_once<system_color::ui_color_wall::ui_color_wall::closure_env$0,tuple$<ref$<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,ref$<country_core::asset_libraries::shader_library::AssetShaderLibrary>,ref_mut$<bevy_ecs::system::commands::Commands> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_color::ui_color_wall::ui_color_wall::closure_env$0 <- core::ptr::drop_in_place<system_color::ui_color_wall::ui_color_wall::closure_env$0>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_color::ui_color_wall::ui_color_wall::closure_env$0 <- country_core::systems::ui::resources::widget_params::WidgetParams::signifier<system_color::ui_color_wall::ui_color_wall::closure_env$0>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 3 条。
// [PDB 内联宿主] core::ops::function::FnOnce::call_once<system_color::ui_color_wall::ui_color_wall::closure_env$0,tuple$<ref$<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,ref$<country_core::asset_libraries::shader_library::AssetShaderLibrary>,ref_mut$<bevy_ecs::system::commands::Commands> > >; module=891, inline_record=40316; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_color::ui_color_wall::ui_color_wall::closure$1
// 信息 ID: d9e38f493e4f0092c1e666d3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "system_color::ui_color_wall::ui_color_wall", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_color::ui_color_wall::ui_color_wall; module=2233, inline_record=4208; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_color::ui_color_wall::ui_color_wall::closure$2
// 信息 ID: 3f5b5658a05c0fe3e3460a4b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "system_color::ui_color_wall::ui_color_wall", "captures": "unknown"}
// [闭包环境类型引用] system_color::ui_color_wall::ui_color_wall::closure_env$2 <- core::ops::function::FnOnce::call_once<system_color::ui_color_wall::ui_color_wall::closure_env$2,tuple$<ref$<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,ref$<country_core::asset_libraries::shader_library::AssetShaderLibrary>,ref_mut$<bevy_ecs::system::commands::Commands> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_color::ui_color_wall::ui_color_wall::closure_env$2 <- country_core::systems::ui::resources::widget_params::WidgetParams::signifier<system_color::ui_color_wall::ui_color_wall::closure_env$2>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 2 条。
// [PDB 内联宿主] core::ops::function::FnOnce::call_once<system_color::ui_color_wall::ui_color_wall::closure_env$2,tuple$<ref$<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,ref$<country_core::asset_libraries::shader_library::AssetShaderLibrary>,ref_mut$<bevy_ecs::system::commands::Commands> > >; module=891, inline_record=39528; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
