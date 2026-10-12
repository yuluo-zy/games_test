// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/country-core/src/systems/despawn.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 524761609c5fef9585adb2d0ac181839eb30a982d695b49d977969d9caf57bc0
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: despawn.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 463, 2826
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use bevy_ecs; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::systems::despawn::despawn_glade_entities
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=3
// 源码行: 5
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6750
// 首条位置: {"module":463,"record_offset":7860,"rva":6551152,"va":5375260272}
// 编译实例 RVAs: 0x63f670
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] bevy_ecs::entity::Entities::contains
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1984856","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375260272,"raw_hex":"e858491e00","size":5,"soft_fail":false,"target_va":5377245296,"va":5375260435,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::system::commands::Commands::entity::panic_no_entity
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t27711540","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375260272,"raw_hex":"e834d8a601","size":5,"soft_fail":false,"target_va":5402972080,"va":5375260535,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::system::commands::EntityCommands::despawn
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2045055","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375260272,"raw_hex":"e87f341f00","size":5,"soft_fail":false,"target_va":5377305552,"va":5375260492,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::query::iter::impl$1::next
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs
//     evidence={"module":463,"record_offset":8204,"parent_record_offset":7860,"ancestor_record_offset":7860,"inlinee":14613,"source_line":881,"archive":"_pdb/A/modules/00463.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::commands::Commands::entity
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs
//     evidence={"module":463,"record_offset":8524,"parent_record_offset":7860,"ancestor_record_offset":7860,"inlinee":14632,"source_line":398,"archive":"_pdb/A/modules/00463.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::query::Query::iter
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/query.rs
//     evidence={"module":463,"record_offset":8008,"parent_record_offset":7860,"ancestor_record_offset":7860,"inlinee":14606,"source_line":435,"archive":"_pdb/A/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIter::new
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs
//     evidence={"module":463,"record_offset":8056,"parent_record_offset":8032,"ancestor_record_offset":7860,"inlinee":14609,"source_line":36,"archive":"_pdb/A/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::init
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs
//     evidence={"module":463,"record_offset":8080,"parent_record_offset":8056,"ancestor_record_offset":7860,"inlinee":14611,"source_line":1520,"archive":"_pdb/A/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::next
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs
//     evidence={"module":463,"record_offset":8232,"parent_record_offset":8204,"ancestor_record_offset":7860,"inlinee":14614,"source_line":1587,"archive":"_pdb/A/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::state::QueryState::iter_unchecked_manual
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/state.rs
//     evidence={"module":463,"record_offset":8032,"parent_record_offset":8008,"ancestor_record_offset":7860,"inlinee":14607,"source_line":1240,"archive":"_pdb/A/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::entities
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":463,"record_offset":8488,"parent_record_offset":8232,"ancestor_record_offset":7860,"inlinee":14616,"source_line":568,"archive":"_pdb/A/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":463,"record_offset":8404,"parent_record_offset":8232,"ancestor_record_offset":7860,"inlinee":14618,"source_line":834,"archive":"_pdb/A/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::get_entity
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs
//     evidence={"module":463,"record_offset":8556,"parent_record_offset":8524,"ancestor_record_offset":7860,"inlinee":14633,"source_line":447,"archive":"_pdb/A/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] core::bool::impl$0::then_some
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\bool.rs
//     evidence={"module":463,"record_offset":8580,"parent_record_offset":8556,"ancestor_record_offset":7860,"inlinee":14634,"source_line":33,"archive":"_pdb/A/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":463,"record_offset":8152,"parent_record_offset":8128,"ancestor_record_offset":7860,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":463,"record_offset":8368,"parent_record_offset":8344,"ancestor_record_offset":7860,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":463,"record_offset":8316,"parent_record_offset":8280,"ancestor_record_offset":7860,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":463,"record_offset":8428,"parent_record_offset":8404,"ancestor_record_offset":7860,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":463,"record_offset":8104,"parent_record_offset":8080,"ancestor_record_offset":7860,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":463,"record_offset":8452,"parent_record_offset":8428,"ancestor_record_offset":7860,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":463,"record_offset":8128,"parent_record_offset":8104,"ancestor_record_offset":7860,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":463,"record_offset":8344,"parent_record_offset":8280,"ancestor_record_offset":7860,"inlinee":5091,"source_line":101,"archive":"_pdb/A/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":463,"record_offset":8280,"parent_record_offset":8232,"ancestor_record_offset":7860,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/00463.jsonl.gz"}
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
// 完整结构及原始类型记录: despawn.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: despawn.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: despawn.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 5; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::query::Query<bevy_ecs::entity::Entity,bevy_ecs::query::filter::Without<country_core::components::KeepOnLevelUnload> >,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::system::query::Query<bevy_ecs::entity::Entity,bevy_ecs::query::filter::Without<country_core::components::KeepOnLevelUnload> >", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] query_without_filter: bevy_ecs::query::filter::Without<country_core::components::KeepOnLevelUnload>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<bevy_ecs::entity::Entity,bevy_ecs::query::filter::Without<country_core::components::KeepOnLevelUnload> >
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::ivy::ivy_storage::IvyStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::ivy::ivy_leaf_storage::IvyLeafStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>,bevy_ecs::change_detection::Res<country_core::resources::walls::wall_holes::PrevWallHoles>,bevy_ecs::event::EventReader<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::resources::ivy::ivy_storage::IvyStorage>", "bevy_ecs::change_detection::ResMut<country_core::resources::ivy::ivy_leaf_storage::IvyLeafStorage>", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::change_detection::Res<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>", "bevy_ecs::change_detection::Res<country_core::resources::walls::wall_holes::PrevWallHoles>", "bevy_ecs::event::EventReader<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::ivy::ivy_storage::IvyStorage>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::ivy::ivy_leaf_storage::IvyLeafStorage>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::wall_holes::PrevWallHoles>
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::ivy::ivy_storage::IvyStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::ivy::ivy_leaf_storage::IvyLeafStorage>,bevy_ecs::event::EventReader<country_core::systems::ivy::ivy_pruner::RemoveIvyCmd>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::resources::ivy::ivy_storage::IvyStorage>", "bevy_ecs::change_detection::ResMut<country_core::resources::ivy::ivy_leaf_storage::IvyLeafStorage>", "bevy_ecs::event::EventReader<country_core::systems::ivy::ivy_pruner::RemoveIvyCmd>", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::ivy::ivy_storage::IvyStorage>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::ivy::ivy_leaf_storage::IvyLeafStorage>
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::systems::ivy::ivy_pruner::RemoveIvyCmd>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::ivy::ivy_leaf_storage::IvyLeafStorage>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::resources::ivy::ivy_storage::IvyStorage>,bevy_ecs::event::EventWriter<country_core::systems::ivy::ivy_leaf_animation::IvyLeafIsGrowing>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::resources::ivy::ivy_leaf_storage::IvyLeafStorage>", "bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>", "bevy_ecs::change_detection::Res<country_core::resources::ivy::ivy_storage::IvyStorage>", "bevy_ecs::event::EventWriter<country_core::systems::ivy::ivy_leaf_animation::IvyLeafIsGrowing>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::ivy::ivy_leaf_storage::IvyLeafStorage>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::ivy::ivy_storage::IvyStorage>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::systems::ivy::ivy_leaf_animation::IvyLeafIsGrowing>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::wall::shapes::rectangle::WipRectangle> >,bevy_ecs::event::EventWriter<country_core::systems::wall::shapes::rectangle::EditRectangleCmd>,bevy_ecs::event::EventWriter<country_core::systems::wall::shapes::rectangle::FinishEditRectangleCmd>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::event::EventWriter<country_core::systems::wall::wall_highlight::HighlightWallCmd>,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::event::EventReader<country_core::resources::window::CursorMoved>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::event::EventReader<country_core::systems::main_camera::MainCameraMoved>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::system::system_param::Local<country_core::systems::wall::shapes::rectangle::ui_edit_rectangle_orientation::ClosestCornerCache>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::event::EventWriter<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::event::EventWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder> >,bevy_ecs::event::EventWriter<country_core::resources::tree_storage::HideTreesinAreaCmd>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::wall::shapes::rectangle::WipRectangle> >", "bevy_ecs::event::EventWriter<country_core::systems::wall::shapes::rectangle::EditRectangleCmd>", "bevy_ecs::event::EventWriter<country_core::systems::wall::shapes::rectangle::FinishEditRectangleCmd>", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>", "bevy_ecs::event::EventWriter<country_core::systems::wall::wall_highlight::HighlightWallCmd>", "bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>", "bevy_ecs::event::EventReader<country_core::resources::window::CursorMoved>", "bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>", "bevy_ecs::event::EventReader<country_core::systems::main_camera::MainCameraMoved>", "bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>", "bevy_ecs::system::system_param::Local<country_core::systems::wall::shapes::rectangle::ui_edit_rectangle_orientation::ClosestCornerCache>", "bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>", "tuple$<bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::event::EventWriter<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::event::EventWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder> >", "bevy_ecs::event::EventWriter<country_core::resources::tree_storage::HideTreesinAreaCmd>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::wall::shapes::rectangle::WipRectangle> >
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::systems::wall::shapes::rectangle::EditRectangleCmd>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::systems::wall::shapes::rectangle::FinishEditRectangleCmd>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::systems::wall::wall_highlight::HighlightWallCmd>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::resources::window::CursorMoved>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::systems::main_camera::MainCameraMoved>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>
// [候选系统接口] system_local: bevy_ecs::system::system_param::Local<country_core::systems::wall::shapes::rectangle::ui_edit_rectangle_orientation::ClosestCornerCache>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::resources::glade_border::DisplayOutOfBorder>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::resources::tree_storage::HideTreesinAreaCmd>

// ------------------------------------------------------------------
// 函数: country_core::systems::despawn::despawn_glade_entities
// 信息 ID: cd46fc94c25c0e1e75a33505
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] cc85d15c013966493a85b83e: void (*)(bevy_ecs::system::query::Query<bevy_ecs::entity::Entity,bevy_ecs::query::filter::Without<country_core::components::KeepOnLevelUnload> >,bevy_ecs::system::commands::Commands)
// 引用方: 原程序直接转移 1 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<bevy_ecs::entity::Entity,bevy_ecs::query::filter::Without<country_core::components::KeepOnLevelUnload> >,bevy_ecs::system::commands::Commands),void (*)(bevy_ecs::system::query::Query<bevy_ecs::entity::Entity,bevy_ecs::query::filter::Without<country_core::components::KeepOnLevelUnload> >,bevy_ecs::system::commands::Commands)> > @ 0xe531db -> 0x63f670 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::system::query::Query<bevy_ecs::entity::Entity,bevy_ecs::query::filter::Without<country_core::components::KeepOnLevelUnload> >,bevy_ecs::system::commands::Commands),void (*)(bevy_ecs::system::query::Query<bevy_ecs::entity::Entity,bevy_ecs::query::filter::Without<country_core::components::KeepOnLevelUnload> >,bevy_ecs::system::commands::Commands)>; module=2826, inline_record=33504; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
