// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/systems/tree-tool/src/startup_tree_tool.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 933b74884eb3b87e4b8308c83baaaeb7050fddaf6388f9ce2bf6c0c23877989c
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: startup_tree_tool.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 2109, 2110, 3379
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use bevy_ecs; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: []
// use std; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_tree::startup_tree_tool::reset_tree_tool
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 4
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x13BA1
// 首条位置: {"module":2109,"inlinee":80801}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":2109,"record_offset":137768,"parent_record_offset":137256,"ancestor_record_offset":137256,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/02109.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":2109,"record_offset":137824,"parent_record_offset":137256,"ancestor_record_offset":137256,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/02109.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::tree_storage::impl$8::default
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/tree_storage.rs
//     evidence={"module":2109,"record_offset":137292,"parent_record_offset":137256,"ancestor_record_offset":137256,"inlinee":15835,"source_line":28,"archive":"_pdb/A/modules/02109.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":2110,"record_offset":3196,"parent_record_offset":3168,"ancestor_record_offset":2232,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/02110.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":2110,"record_offset":3168,"parent_record_offset":2952,"ancestor_record_offset":2232,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/02110.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$29::set_changed
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":2109,"record_offset":137792,"parent_record_offset":137768,"ancestor_record_offset":137256,"inlinee":5812,"source_line":309,"archive":"_pdb/A/modules/02109.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":2109,"record_offset":137588,"parent_record_offset":137560,"ancestor_record_offset":137256,"inlinee":5551,"source_line":518,"archive":"_pdb/A/modules/02109.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":2109,"record_offset":137652,"parent_record_offset":137616,"ancestor_record_offset":137256,"inlinee":4790,"source_line":479,"archive":"_pdb/A/modules/02109.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":2110,"record_offset":2580,"parent_record_offset":2524,"ancestor_record_offset":2232,"inlinee":4789,"source_line":410,"archive":"_pdb/A/modules/02110.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":2109,"record_offset":137676,"parent_record_offset":137652,"ancestor_record_offset":137256,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/02109.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":2110,"record_offset":3044,"parent_record_offset":3016,"ancestor_record_offset":2232,"inlinee":5978,"source_line":686,"archive":"_pdb/A/modules/02110.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":2110,"record_offset":3068,"parent_record_offset":3044,"ancestor_record_offset":2232,"inlinee":5979,"source_line":2173,"archive":"_pdb/A/modules/02110.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":2109,"record_offset":137616,"parent_record_offset":137560,"ancestor_record_offset":137256,"inlinee":6614,"source_line":1639,"archive":"_pdb/A/modules/02109.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":2109,"record_offset":137516,"parent_record_offset":137492,"ancestor_record_offset":137256,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/02109.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":2109,"record_offset":137848,"parent_record_offset":137824,"ancestor_record_offset":137256,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/02109.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":2110,"record_offset":3132,"parent_record_offset":3108,"ancestor_record_offset":2232,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/02110.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":2110,"record_offset":3108,"parent_record_offset":2984,"ancestor_record_offset":2232,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/02110.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":2109,"record_offset":137700,"parent_record_offset":137676,"ancestor_record_offset":137256,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/02109.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::allocation_info
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2110,"record_offset":2984,"parent_record_offset":2952,"ancestor_record_offset":2232,"inlinee":6232,"source_line":3289,"archive":"_pdb/A/modules/02110.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::drop_inner_table
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2110,"record_offset":2896,"parent_record_offset":2872,"ancestor_record_offset":2232,"inlinee":6229,"source_line":2430,"archive":"_pdb/A/modules/02110.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::free_buckets
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2110,"record_offset":2952,"parent_record_offset":2896,"ancestor_record_offset":2232,"inlinee":6231,"source_line":3259,"archive":"_pdb/A/modules/02110.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::is_empty_singleton
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2110,"record_offset":2924,"parent_record_offset":2896,"ancestor_record_offset":2232,"inlinee":6230,"source_line":2827,"archive":"_pdb/A/modules/02110.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::TableLayout::calculate_layout_for
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2110,"record_offset":3016,"parent_record_offset":2984,"ancestor_record_offset":2232,"inlinee":6234,"source_line":256,"archive":"_pdb/A/modules/02110.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$18::drop
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2109,"record_offset":137920,"parent_record_offset":137896,"ancestor_record_offset":137256,"inlinee":5572,"source_line":3669,"archive":"_pdb/A/modules/02109.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$8::default
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":2109,"record_offset":137316,"parent_record_offset":137292,"ancestor_record_offset":137256,"inlinee":5535,"source_line":1322,"archive":"_pdb/A/modules/02109.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":2109,"record_offset":137364,"parent_record_offset":137340,"ancestor_record_offset":137256,"inlinee":5539,"source_line":55,"archive":"_pdb/A/modules/02109.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::__getit
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":2109,"record_offset":137440,"parent_record_offset":137412,"ancestor_record_offset":137256,"inlinee":5544,"source_line":85,"archive":"_pdb/A/modules/02109.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":2109,"record_offset":137560,"parent_record_offset":137412,"ancestor_record_offset":137256,"inlinee":5550,"source_line":71,"archive":"_pdb/A/modules/02109.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":2109,"record_offset":137340,"parent_record_offset":137316,"ancestor_record_offset":137256,"inlinee":5537,"source_line":151,"archive":"_pdb/A/modules/02109.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::fast_local::Key::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":2109,"record_offset":137464,"parent_record_offset":137440,"ancestor_record_offset":137256,"inlinee":5546,"source_line":165,"archive":"_pdb/A/modules/02109.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::lazy::LazyKeyInner::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\mod.rs
//     evidence={"module":2109,"record_offset":137492,"parent_record_offset":137464,"ancestor_record_offset":137256,"inlinee":5548,"source_line":41,"archive":"_pdb/A/modules/02109.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":2109,"record_offset":137412,"parent_record_offset":137388,"ancestor_record_offset":137256,"inlinee":5542,"source_line":278,"archive":"_pdb/A/modules/02109.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":2109,"record_offset":137388,"parent_record_offset":137364,"ancestor_record_offset":137256,"inlinee":5541,"source_line":256,"archive":"_pdb/A/modules/02109.jsonl.gz"}
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
// 完整结构及原始类型记录: startup_tree_tool.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: startup_tree_tool.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: startup_tree_tool.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 12; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::query::Query<ref_mut$<country_core::systems::prefab::PrefabInstanceData>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeights>,bevy_ecs::event::EventReader<country_core::systems::terrain::delayed_terrain_changed_msg::OnTerrainChange>)
// 候选参数类型: ["bevy_ecs::system::query::Query<ref_mut$<country_core::systems::prefab::PrefabInstanceData>,tuple$<> >", "bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeights>", "bevy_ecs::event::EventReader<country_core::systems::terrain::delayed_terrain_changed_msg::OnTerrainChange>"]
// [候选系统接口] component_query: bevy_ecs::system::query::Query<ref_mut$<country_core::systems::prefab::PrefabInstanceData>,tuple$<> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeights>
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::systems::terrain::delayed_terrain_changed_msg::OnTerrainChange>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::event::EventWriter<country_core::resources::history::tree_edit::InsertTreeCmd>,bevy_ecs::event::EventWriter<country_core::resources::tree_storage::MoveTreeCmd>,bevy_ecs::event::EventWriter<country_core::resources::history::tree_edit::DeleteTreeCmd>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>", "bevy_ecs::event::EventWriter<country_core::resources::history::tree_edit::InsertTreeCmd>", "bevy_ecs::event::EventWriter<country_core::resources::tree_storage::MoveTreeCmd>", "bevy_ecs::event::EventWriter<country_core::resources::history::tree_edit::DeleteTreeCmd>", "bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::resources::history::tree_edit::InsertTreeCmd>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::resources::tree_storage::MoveTreeCmd>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::resources::history::tree_edit::DeleteTreeCmd>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::event::EventWriter<system_tree::ui_place_tree::PlaceTreeCmd>,bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >", "bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>", "bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>", "bevy_ecs::event::EventWriter<system_tree::ui_place_tree::PlaceTreeCmd>", "bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>", "bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>", "bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>", "bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>", "bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<system_tree::ui_place_tree::PlaceTreeCmd>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::tree_storage::TreeStorage>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::prefab::PrefabInstanceData>,tuple$<> >,bevy_ecs::event::EventReader<country_core::resources::tree_storage::MoveTreeCmd>,bevy_ecs::event::EventReader<country_core::resources::history::tree_edit::DeleteTreeCmd>,bevy_ecs::event::EventReader<country_core::resources::history::tree_edit::InsertTreeCmd>,bevy_ecs::event::EventWriter<country_core::systems::prefab::SpawnPrefabCmd>,bevy_ecs::system::commands::Commands,bevy_ecs::event::EventWriter<country_core::systems::audio::tree_audio::PlayTreeSoundCmd>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::EventWriter<country_core::systems::prefab::DespawnPrefabCmd>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::resources::tree_storage::TreeStorage>", "bevy_ecs::system::query::Query<ref_mut$<country_core::systems::prefab::PrefabInstanceData>,tuple$<> >", "bevy_ecs::event::EventReader<country_core::resources::tree_storage::MoveTreeCmd>", "bevy_ecs::event::EventReader<country_core::resources::history::tree_edit::DeleteTreeCmd>", "bevy_ecs::event::EventReader<country_core::resources::history::tree_edit::InsertTreeCmd>", "bevy_ecs::event::EventWriter<country_core::systems::prefab::SpawnPrefabCmd>", "bevy_ecs::system::commands::Commands", "bevy_ecs::event::EventWriter<country_core::systems::audio::tree_audio::PlayTreeSoundCmd>", "bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>", "bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>", "bevy_ecs::event::EventWriter<country_core::systems::prefab::DespawnPrefabCmd>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::tree_storage::TreeStorage>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<ref_mut$<country_core::systems::prefab::PrefabInstanceData>,tuple$<> >
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::resources::tree_storage::MoveTreeCmd>
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::resources::history::tree_edit::DeleteTreeCmd>
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::resources::history::tree_edit::InsertTreeCmd>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::systems::prefab::SpawnPrefabCmd>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::systems::audio::tree_audio::PlayTreeSoundCmd>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::systems::prefab::DespawnPrefabCmd>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::components::TreeMoveBend>,ref_mut$<country_core::systems::prefab::PrefabInstanceData> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)
// 候选参数类型: ["bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::components::TreeMoveBend>,ref_mut$<country_core::systems::prefab::PrefabInstanceData> >,tuple$<> >", "bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>"]
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::components::TreeMoveBend>,ref_mut$<country_core::systems::prefab::PrefabInstanceData> >,tuple$<> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::resources::tree_storage::TreeStorage>,bevy_ecs::system::query::Query<ref$<country_core::systems::prefab::PrefabInstanceData>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::event::EventWriter<country_core::resources::tree_storage::MoveTreeCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::system::system_param::Local<system_tree::ui_move_tree::TreeStartingPosition>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >", "bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>", "bevy_ecs::change_detection::Res<country_core::resources::tree_storage::TreeStorage>", "bevy_ecs::system::query::Query<ref$<country_core::systems::prefab::PrefabInstanceData>,tuple$<> >", "bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>", "bevy_ecs::event::EventWriter<country_core::resources::tree_storage::MoveTreeCmd>", "bevy_ecs::change_detection::ResMut<country_core::resources::history::History>", "bevy_ecs::system::system_param::Local<system_tree::ui_move_tree::TreeStartingPosition>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::tree_storage::TreeStorage>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<ref$<country_core::systems::prefab::PrefabInstanceData>,tuple$<> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::resources::tree_storage::MoveTreeCmd>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::history::History>
// [候选系统接口] system_local: bevy_ecs::system::system_param::Local<system_tree::ui_move_tree::TreeStartingPosition>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::tree_storage::TreeStorage>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::resources::tree_storage::TreeStorage>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::tree_storage::TreeStorage>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::event::EventReader<country_core::resources::history::tree_edit::SpawnPreplacedTrees>,bevy_ecs::event::EventWriter<country_core::resources::history::tree_edit::DeleteTreeCmd>,bevy_ecs::system::system_param::Local<system_tree::convert_preplaced_trees::Wait>,bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::GardenRaster>,bevy_ecs::change_detection::Res<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::tree_storage::TreeStorage>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::systems::prefab::PrefabInstanceData>,ref$<country_core::components::ConvertedPreplacedTree> >,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>)
// 候选参数类型: ["bevy_ecs::event::EventReader<country_core::resources::history::tree_edit::SpawnPreplacedTrees>", "bevy_ecs::event::EventWriter<country_core::resources::history::tree_edit::DeleteTreeCmd>", "bevy_ecs::system::system_param::Local<system_tree::convert_preplaced_trees::Wait>", "bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::PathRaster>", "bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>", "bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::GardenRaster>", "bevy_ecs::change_detection::Res<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>", "bevy_ecs::change_detection::Res<country_core::resources::tree_storage::TreeStorage>", "bevy_ecs::system::query::Query<tuple$<ref$<country_core::systems::prefab::PrefabInstanceData>,ref$<country_core::components::ConvertedPreplacedTree> >,tuple$<> >", "bevy_ecs::change_detection::ResMut<country_core::resources::history::History>"]
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::resources::history::tree_edit::SpawnPreplacedTrees>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::resources::history::tree_edit::DeleteTreeCmd>
// [候选系统接口] system_local: bevy_ecs::system::system_param::Local<system_tree::convert_preplaced_trees::Wait>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::PathRaster>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::GardenRaster>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::tree_storage::TreeStorage>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<ref$<country_core::systems::prefab::PrefabInstanceData>,ref$<country_core::components::ConvertedPreplacedTree> >,tuple$<> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::history::History>
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: system_tree::startup_tree_tool::reset_tree_tool
// 信息 ID: 127cb54e92c4e2f64ab57a04
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 5c61907c31a24a2541aa7125: void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::tree_storage::TreeStorage>)
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::tree_storage::TreeStorage>),void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::tree_storage::TreeStorage>)> >; module=2109, inline_record=137256; 不是独立调用边。
// [PDB 内联宿主] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::tree_storage::TreeStorage>),void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::tree_storage::TreeStorage>)>; module=2110, inline_record=2232; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
