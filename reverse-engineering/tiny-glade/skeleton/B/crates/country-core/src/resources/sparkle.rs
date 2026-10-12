// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/country-core/src/resources/sparkle.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: 57de1eb46f1363d46584a07a9f90e0eef5532970acbe4c595922dcaa4b39730a
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: sparkle.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 560
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// 当前没有可证实的 crate 引用信息。
// END GENERATED DEPENDENCY ANNOTATIONS


// PDB 中没有可直接归属到此文件的过程/内联函数记录。
// 文件仍保留 checksum、编译单元引用及存在的源码行记录。

// === PDB STRUCTURES V1 BEGIN ===
// 数据结构事实按版本保留；PDB 定义 / 仅名称关系 / 原指令观察分别标注。
// 完整结构及原始类型记录: sparkle.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::resources::sparkle::SparkleSsbo
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 21
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::sparkle::SparkleSsbo>", "constructor": "country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation", "arguments": ["country_core::resources::sparkle::SparkleSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::option::Option<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::sparkle::SparkleSsbo> >", "constructor": "core::option::Option", "arguments": ["country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::sparkle::SparkleSsbo>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<core::option::Option<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::sparkle::SparkleSsbo> > >", "constructor": "enum2$", "arguments": ["core::option::Option<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::sparkle::SparkleSsbo> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<core::option::Option<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::sparkle::SparkleSsbo> > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<core::option::Option<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::sparkle::SparkleSsbo> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::ssbo::TypedStorageBuffer<country_core::resources::sparkle::SparkleSsbo>", "constructor": "country_core::render::ssbo::TypedStorageBuffer", "arguments": ["country_core::resources::sparkle::SparkleSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<country_core::render::ssbo::TypedStorageBuffer<country_core::resources::sparkle::SparkleSsbo> >", "constructor": "core::ptr::drop_in_place", "arguments": ["country_core::render::ssbo::TypedStorageBuffer<country_core::resources::sparkle::SparkleSsbo>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::sparse_instance_buffer::SparseInstanceBuffer<country_core::resources::sparkle::SparkleSsbo>", "constructor": "country_core::render::sparse_instance_buffer::SparseInstanceBuffer", "arguments": ["country_core::resources::sparkle::SparkleSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<country_core::render::sparse_instance_buffer::SparseInstanceBuffer<country_core::resources::sparkle::SparkleSsbo> >", "constructor": "core::ptr::drop_in_place", "arguments": ["country_core::render::sparse_instance_buffer::SparseInstanceBuffer<country_core::resources::sparkle::SparkleSsbo>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::sparse_instance_buffer::SparseInstanceBufferWriter<country_core::resources::sparkle::SparkleSsbo>", "constructor": "country_core::render::sparse_instance_buffer::SparseInstanceBufferWriter", "arguments": ["country_core::resources::sparkle::SparkleSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<country_core::render::sparse_instance_buffer::SparseInstanceBufferWriter<country_core::resources::sparkle::SparkleSsbo> >", "constructor": "core::ptr::drop_in_place", "arguments": ["country_core::render::sparse_instance_buffer::SparseInstanceBufferWriter<country_core::resources::sparkle::SparkleSsbo>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::sparkle::SparkleSsbo> >", "constructor": "core::ptr::drop_in_place", "arguments": ["country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::sparkle::SparkleSsbo>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::pass_builder::pass_resource_access::read_access_buffer<country_core::resources::sparkle::SparkleSsbo>", "constructor": "country_core::render::pass_builder::pass_resource_access::read_access_buffer", "arguments": ["country_core::resources::sparkle::SparkleSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_mut<country_core::resources::sparkle::SparkleSsbo>", "constructor": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_mut", "arguments": ["country_core::resources::sparkle::SparkleSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::sparse_instance_buffer::SparseInstanceBuffer::gpu_flush<country_core::resources::sparkle::SparkleSsbo>", "constructor": "country_core::render::sparse_instance_buffer::SparseInstanceBuffer::gpu_flush", "arguments": ["country_core::resources::sparkle::SparkleSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::sparse_instance_buffer::SparseInstanceBufferWriter::push<country_core::resources::sparkle::SparkleSsbo>", "constructor": "country_core::render::sparse_instance_buffer::SparseInstanceBufferWriter::push", "arguments": ["country_core::resources::sparkle::SparkleSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::add_new<country_core::resources::sparkle::SparkleSsbo>", "constructor": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::add_new", "arguments": ["country_core::resources::sparkle::SparkleSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_rg_buffer<country_core::resources::sparkle::SparkleSsbo>", "constructor": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_rg_buffer", "arguments": ["country_core::resources::sparkle::SparkleSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::any::impl$0::type_id<country_core::render::sparse_instance_buffer::SparseInstanceBuffer<country_core::resources::sparkle::SparkleSsbo> >", "constructor": "core::any::impl$0::type_id", "arguments": ["country_core::render::sparse_instance_buffer::SparseInstanceBuffer<country_core::resources::sparkle::SparkleSsbo>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::sparse_instance_buffer::impl$10::new_storage_buffer<country_core::resources::sparkle::SparkleSsbo>", "constructor": "country_core::render::sparse_instance_buffer::impl$10::new_storage_buffer", "arguments": ["country_core::resources::sparkle::SparkleSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::sparse_instance_buffer::impl$12::as_any<country_core::resources::sparkle::SparkleSsbo>", "constructor": "country_core::render::sparse_instance_buffer::impl$12::as_any", "arguments": ["country_core::resources::sparkle::SparkleSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::sparse_instance_buffer::impl$12::as_any_mut<country_core::resources::sparkle::SparkleSsbo>", "constructor": "country_core::render::sparse_instance_buffer::impl$12::as_any_mut", "arguments": ["country_core::resources::sparkle::SparkleSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::ssbo::TypedStorageBuffer::new<country_core::resources::sparkle::SparkleSsbo>", "constructor": "country_core::render::ssbo::TypedStorageBuffer::new", "arguments": ["country_core::resources::sparkle::SparkleSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::pass_builder::draw_builder::impl$2::read_storage_buffer<country_core::resources::sparkle::SparkleSsbo>", "constructor": "country_core::render::pass_builder::draw_builder::impl$2::read_storage_buffer", "arguments": ["country_core::resources::sparkle::SparkleSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::ssbo::TypedStorageBuffer::gpu_write_ranges_nonoverlapping<country_core::resources::sparkle::SparkleSsbo>", "constructor": "country_core::render::ssbo::TypedStorageBuffer::gpu_write_ranges_nonoverlapping", "arguments": ["country_core::resources::sparkle::SparkleSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: sparkle.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// [已有模块证据关联；不自动成为字段事实] evidence/parallel-camera/B-decompile/report.json; explicit B; [{"kind": "exact_PDB_named_entity_mention", "name": "country_core::resources::sparkle::SparkleSsbo", "status": "evidence_association; original field ownership unknown"}]
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: sparkle.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 5; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::audio_resources::AudioSamplesLoadedPromise>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::resources::audio_resources::AudioSamplesLoadedPromise>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::audio_resources::AudioSamplesLoadedPromise>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::history::history_process::PendingHistoryCommands>,bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::history::replay_mode::HoldToFastReplay> >,bevy_ecs::change_detection::ResMut<country_core::systems::history::replay_mode::RestoreAppTo>,bevy_ecs::change_detection::ResMut<country_core::systems::history::signifier_replay_active::ReplayModeElapsedTime>,bevy_ecs::event::writer::EventWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::timelapse_replay::ReplaySpeedMultiplierOverride>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >", "bevy_ecs::change_detection::Res<country_core::systems::history::history_process::PendingHistoryCommands>", "bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>", "bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>", "bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::history::replay_mode::HoldToFastReplay> >", "bevy_ecs::change_detection::ResMut<country_core::systems::history::replay_mode::RestoreAppTo>", "bevy_ecs::change_detection::ResMut<country_core::systems::history::signifier_replay_active::ReplayModeElapsedTime>", "bevy_ecs::event::writer::EventWriter<country_core::systems::history::history_process::HistoryCmd>", "bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>", "bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>", "bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>", "bevy_ecs::change_detection::Res<country_core::systems::timelapse_replay::ReplaySpeedMultiplierOverride>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::history::history_process::PendingHistoryCommands>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::history::replay_mode::HoldToFastReplay> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::history::replay_mode::RestoreAppTo>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::history::signifier_replay_active::ReplayModeElapsedTime>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::systems::history::history_process::HistoryCmd>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::timelapse_replay::ReplaySpeedMultiplierOverride>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::ResMut<country_core::systems::ui_clutter::DeletedAutoClutter>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>", "bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>", "bevy_ecs::change_detection::ResMut<country_core::systems::ui_clutter::DeletedAutoClutter>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::ui_clutter::DeletedAutoClutter>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::ivy::ivy_storage::IvyStorage>,bevy_ecs::change_detection::Res<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>,bevy_ecs::system::system_param::Local<country_core::resources::ivy::ivy_direction_proposer::IvyDirectionProposer>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::walls::wall_holes::PrevWallHoles>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::event::writer::EventWriter<country_core::systems::ivy::ivy_leaf_spawner::IvySegmentDoneGrowingMsg>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::event::reader::EventReader<country_core::systems::ivy::ivy_pruner::RemoveIvyCmd>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::resources::ivy::ivy_storage::IvyStorage>", "bevy_ecs::change_detection::Res<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>", "bevy_ecs::system::system_param::Local<country_core::resources::ivy::ivy_direction_proposer::IvyDirectionProposer>", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::change_detection::Res<country_core::resources::walls::wall_holes::PrevWallHoles>", "bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>", "bevy_ecs::event::writer::EventWriter<country_core::systems::ivy::ivy_leaf_spawner::IvySegmentDoneGrowingMsg>", "bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>", "bevy_ecs::event::reader::EventReader<country_core::systems::ivy::ivy_pruner::RemoveIvyCmd>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::ivy::ivy_storage::IvyStorage>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>
// [候选系统接口] system_local: bevy_ecs::system::system_param::Local<country_core::resources::ivy::ivy_direction_proposer::IvyDirectionProposer>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::wall_holes::PrevWallHoles>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::systems::ivy::ivy_leaf_spawner::IvySegmentDoneGrowingMsg>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::systems::ivy::ivy_pruner::RemoveIvyCmd>

// === PDB ENRICHMENT V1 END ===
