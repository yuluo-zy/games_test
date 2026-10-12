// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/systems/tree-tool/src/startup_tree_tool.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: 933b74884eb3b87e4b8308c83baaaeb7050fddaf6388f9ce2bf6c0c23877989c
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: startup_tree_tool.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 2449, 2457, 2458, 3654
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
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.3"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.5"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.15.2"}]
// use std; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_tree::startup_tree_tool::reset_tree_tool
// 记录形式: inline_instance, inlinee_declaration; instances=5
// 源码行: 4
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x17CBD
// 首条位置: {"module":2449,"inlinee":97469}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":2449,"record_offset":3040,"parent_record_offset":2616,"ancestor_record_offset":2616,"inlinee":4457,"source_line":434,"archive":"_pdb/B/modules/02449.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":2449,"record_offset":3096,"parent_record_offset":2616,"ancestor_record_offset":2616,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/02449.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::tree_storage::impl$6::default
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/tree_storage.rs
//     evidence={"module":2449,"record_offset":2644,"parent_record_offset":2616,"ancestor_record_offset":2616,"inlinee":18326,"source_line":33,"archive":"_pdb/B/modules/02449.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":2449,"record_offset":3452,"parent_record_offset":3424,"ancestor_record_offset":2616,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/02449.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":2449,"record_offset":3424,"parent_record_offset":3272,"ancestor_record_offset":2616,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/02449.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":2449,"record_offset":3064,"parent_record_offset":3040,"ancestor_record_offset":2616,"inlinee":4459,"source_line":397,"archive":"_pdb/B/modules/02449.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":2449,"record_offset":2872,"parent_record_offset":2844,"ancestor_record_offset":2616,"inlinee":4798,"source_line":541,"archive":"_pdb/B/modules/02449.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":2449,"record_offset":2924,"parent_record_offset":2900,"ancestor_record_offset":2616,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/02449.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":2449,"record_offset":2900,"parent_record_offset":2844,"ancestor_record_offset":2616,"inlinee":4434,"source_line":428,"archive":"_pdb/B/modules/02449.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":2449,"record_offset":2948,"parent_record_offset":2924,"ancestor_record_offset":2616,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/02449.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":2449,"record_offset":3360,"parent_record_offset":3336,"ancestor_record_offset":2616,"inlinee":4812,"source_line":486,"archive":"_pdb/B/modules/02449.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":2458,"record_offset":136716,"parent_record_offset":136660,"ancestor_record_offset":136428,"inlinee":6331,"source_line":1980,"archive":"_pdb/B/modules/02458.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":2449,"record_offset":2816,"parent_record_offset":2788,"ancestor_record_offset":2616,"inlinee":4795,"source_line":296,"archive":"_pdb/B/modules/02449.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":2449,"record_offset":3120,"parent_record_offset":3096,"ancestor_record_offset":2616,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/02449.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":2449,"record_offset":3392,"parent_record_offset":3304,"ancestor_record_offset":2616,"inlinee":4723,"source_line":1129,"archive":"_pdb/B/modules/02449.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":2449,"record_offset":2972,"parent_record_offset":2948,"ancestor_record_offset":2616,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/02449.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::allocation_info
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":2449,"record_offset":3304,"parent_record_offset":3272,"ancestor_record_offset":2616,"inlinee":6131,"source_line":2973,"archive":"_pdb/B/modules/02449.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::drop_inner_table
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":2449,"record_offset":3216,"parent_record_offset":3192,"ancestor_record_offset":2616,"inlinee":6201,"source_line":2124,"archive":"_pdb/B/modules/02449.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::free_buckets
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":2449,"record_offset":3272,"parent_record_offset":3216,"ancestor_record_offset":2616,"inlinee":6130,"source_line":2943,"archive":"_pdb/B/modules/02449.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::is_empty_singleton
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":2449,"record_offset":3244,"parent_record_offset":3216,"ancestor_record_offset":2616,"inlinee":6139,"source_line":2509,"archive":"_pdb/B/modules/02449.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::TableLayout::calculate_layout_for
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":2449,"record_offset":3336,"parent_record_offset":3304,"ancestor_record_offset":2616,"inlinee":4811,"source_line":162,"archive":"_pdb/B/modules/02449.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$18::drop
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":2449,"record_offset":3192,"parent_record_offset":3168,"ancestor_record_offset":2616,"inlinee":6180,"source_line":3306,"archive":"_pdb/B/modules/02449.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$8::default
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":2449,"record_offset":2672,"parent_record_offset":2644,"ancestor_record_offset":2616,"inlinee":7397,"source_line":1406,"archive":"_pdb/B/modules/02449.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":2449,"record_offset":2728,"parent_record_offset":2700,"ancestor_record_offset":2616,"inlinee":4794,"source_line":56,"archive":"_pdb/B/modules/02449.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":2449,"record_offset":2844,"parent_record_offset":2788,"ancestor_record_offset":2616,"inlinee":4797,"source_line":72,"archive":"_pdb/B/modules/02449.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":2449,"record_offset":2700,"parent_record_offset":2672,"ancestor_record_offset":2616,"inlinee":4792,"source_line":151,"archive":"_pdb/B/modules/02449.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":2449,"record_offset":2788,"parent_record_offset":2756,"ancestor_record_offset":2616,"inlinee":4421,"source_line":305,"archive":"_pdb/B/modules/02449.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":2449,"record_offset":2756,"parent_record_offset":2728,"ancestor_record_offset":2616,"inlinee":4420,"source_line":270,"archive":"_pdb/B/modules/02449.jsonl.gz"}
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
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 12; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::tree_storage::TreeStorage>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::resources::tree_storage::TreeStorage>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::tree_storage::TreeStorage>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::components::TreeMoveBend>,ref_mut$<country_core::systems::prefab::PrefabInstanceData> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)
// 候选参数类型: ["bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::components::TreeMoveBend>,ref_mut$<country_core::systems::prefab::PrefabInstanceData> >,tuple$<> >", "bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>"]
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::components::TreeMoveBend>,ref_mut$<country_core::systems::prefab::PrefabInstanceData> >,tuple$<> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::components::TreeAnimation>,ref$<country_core::components::TreeMarkedForDeletion> >,tuple$<> >,bevy_ecs::event::writer::EventWriter<country_core::systems::prefab::DespawnPrefabCmd>)
// 候选参数类型: ["bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::components::TreeAnimation>,ref$<country_core::components::TreeMarkedForDeletion> >,tuple$<> >", "bevy_ecs::event::writer::EventWriter<country_core::systems::prefab::DespawnPrefabCmd>"]
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::components::TreeAnimation>,ref$<country_core::components::TreeMarkedForDeletion> >,tuple$<> >
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::systems::prefab::DespawnPrefabCmd>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::systems::prefab::PrefabInstanceData>,ref$<country_core::resources::prefab::PrefabInstance>,ref$<country_core::components::TreeComponent> >,tuple$<> >,bevy_ecs::event::reader::EventReader<country_core::resources::path_stroke::PathDrawEvent>,bevy_ecs::event::writer::EventWriter<country_core::resources::history::tree_edit::DeleteTreeCmd>,bevy_ecs::change_detection::Res<country_core::resources::tree_storage::TreeStorage>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::systems::wall::eraser::EraserSizeMultiplier>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >", "bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>", "bevy_ecs::system::query::Query<tuple$<ref$<country_core::systems::prefab::PrefabInstanceData>,ref$<country_core::resources::prefab::PrefabInstance>,ref$<country_core::components::TreeComponent> >,tuple$<> >", "bevy_ecs::event::reader::EventReader<country_core::resources::path_stroke::PathDrawEvent>", "bevy_ecs::event::writer::EventWriter<country_core::resources::history::tree_edit::DeleteTreeCmd>", "bevy_ecs::change_detection::Res<country_core::resources::tree_storage::TreeStorage>", "bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>", "bevy_ecs::change_detection::ResMut<country_core::resources::history::History>", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>", "bevy_ecs::change_detection::Res<country_core::systems::wall::eraser::EraserSizeMultiplier>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<ref$<country_core::systems::prefab::PrefabInstanceData>,ref$<country_core::resources::prefab::PrefabInstance>,ref$<country_core::components::TreeComponent> >,tuple$<> >
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::resources::path_stroke::PathDrawEvent>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::resources::history::tree_edit::DeleteTreeCmd>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::tree_storage::TreeStorage>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::history::History>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::wall::eraser::EraserSizeMultiplier>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::event::writer::EventWriter<system_tree::ui_place_tree::PlaceTreeCmd>,bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::event::writer::EventWriter<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::event::writer::EventWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder> >)
// 候选参数类型: ["bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >", "bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>", "bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>", "bevy_ecs::event::writer::EventWriter<system_tree::ui_place_tree::PlaceTreeCmd>", "bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>", "bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>", "bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>", "bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>", "bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>", "tuple$<bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::event::writer::EventWriter<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::event::writer::EventWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder> >"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<system_tree::ui_place_tree::PlaceTreeCmd>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::resources::glade_border::DisplayOutOfBorder>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::resources::tree_storage::TreeStorage>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::systems::prefab::PrefabInstanceData>,ref$<country_core::resources::prefab::PrefabInstance> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::event::writer::EventWriter<country_core::resources::tree_storage::MoveTreeCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::system::system_param::Local<system_tree::ui_move_tree::TreeMoveCache>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::event::writer::EventWriter<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::event::writer::EventWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder> >)
// 候选参数类型: ["bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >", "bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>", "bevy_ecs::change_detection::Res<country_core::resources::tree_storage::TreeStorage>", "bevy_ecs::system::query::Query<tuple$<ref$<country_core::systems::prefab::PrefabInstanceData>,ref$<country_core::resources::prefab::PrefabInstance> >,tuple$<> >", "bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>", "bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >", "bevy_ecs::event::writer::EventWriter<country_core::resources::tree_storage::MoveTreeCmd>", "bevy_ecs::change_detection::ResMut<country_core::resources::history::History>", "bevy_ecs::system::system_param::Local<system_tree::ui_move_tree::TreeMoveCache>", "tuple$<bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::event::writer::EventWriter<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::event::writer::EventWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder> >"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::tree_storage::TreeStorage>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<ref$<country_core::systems::prefab::PrefabInstanceData>,ref$<country_core::resources::prefab::PrefabInstance> >,tuple$<> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::resources::tree_storage::MoveTreeCmd>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::history::History>
// [候选系统接口] system_local: bevy_ecs::system::system_param::Local<system_tree::ui_move_tree::TreeMoveCache>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::resources::glade_border::DisplayOutOfBorder>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::event::writer::EventWriter<country_core::resources::history::tree_edit::InsertTreeCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::tree_storage::MoveTreeCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::history::tree_edit::DeleteTreeCmd>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>", "bevy_ecs::event::writer::EventWriter<country_core::resources::history::tree_edit::InsertTreeCmd>", "bevy_ecs::event::writer::EventWriter<country_core::resources::tree_storage::MoveTreeCmd>", "bevy_ecs::event::writer::EventWriter<country_core::resources::history::tree_edit::DeleteTreeCmd>", "bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::resources::history::tree_edit::InsertTreeCmd>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::resources::tree_storage::MoveTreeCmd>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::resources::history::tree_edit::DeleteTreeCmd>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::query::Query<ref_mut$<country_core::systems::prefab::PrefabInstanceData>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::event::reader::EventReader<country_core::systems::terrain::delayed_terrain_changed_msg::OnTerrainChange>)
// 候选参数类型: ["bevy_ecs::system::query::Query<ref_mut$<country_core::systems::prefab::PrefabInstanceData>,tuple$<> >", "bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >", "bevy_ecs::event::reader::EventReader<country_core::systems::terrain::delayed_terrain_changed_msg::OnTerrainChange>"]
// [候选系统接口] component_query: bevy_ecs::system::query::Query<ref_mut$<country_core::systems::prefab::PrefabInstanceData>,tuple$<> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::systems::terrain::delayed_terrain_changed_msg::OnTerrainChange>
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: system_tree::startup_tree_tool::reset_tree_tool
// 信息 ID: ffead668da5fb6e770914a57
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 5c61907c31a24a2541aa7125: void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::tree_storage::TreeStorage>)
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] bevy_ecs::system::schedule_system::impl$1::run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::tree_storage::TreeStorage>),void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::tree_storage::TreeStorage>)> >; module=2449, inline_record=2616; 不是独立调用边。
// [PDB 内联宿主] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::tree_storage::TreeStorage>),void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::tree_storage::TreeStorage>)> > >; module=2458, inline_record=136428; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
