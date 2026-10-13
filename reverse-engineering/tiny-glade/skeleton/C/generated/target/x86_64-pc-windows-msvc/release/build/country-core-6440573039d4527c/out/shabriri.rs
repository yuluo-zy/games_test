// Generated from PDB metadata by skeleton/_tools/enrich_skeleton.py.
// Version: C
// Skeleton path: generated/target/x86_64-pc-windows-msvc/release/build/country-core-6440573039d4527c/out/shabriri.rs
// Original source: /home/h3/code/pouncelight/country-slice-private/target/x86_64-pc-windows-msvc/release/build/country-core-6440573039d4527c/out/shabriri.rs
// 函数体、未记录参数名和布局 = unknown。

// === PDB STRUCTURES V1 BEGIN ===
// 数据结构事实按版本保留；PDB 定义 / 仅名称关系 / 原指令观察分别标注。
// 完整结构及原始类型记录: shabriri.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: shabriri.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: shabriri.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 132; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_state::StairsState>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >", "bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_state::StairsState>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_state::StairsState>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::systems::ui_clutter::debug_spawn_clutter::DebugSpawnAllClutterCmd>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::ui_clutter::debug_spawn_clutter::DebugSpawnClutterCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::ui_clutter::ui_autoclutter::InstaCreateDeco>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >)
// 候选参数类型: ["bevy_ecs::message::message_reader::MessageReader<country_core::systems::ui_clutter::debug_spawn_clutter::DebugSpawnAllClutterCmd>", "bevy_ecs::message::message_reader::MessageReader<country_core::systems::ui_clutter::debug_spawn_clutter::DebugSpawnClutterCmd>", "bevy_ecs::message::message_writer::MessageWriter<country_core::systems::ui_clutter::ui_autoclutter::InstaCreateDeco>", "bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >", "bevy_ecs::change_detection::ResMut<country_core::resources::history::History>", "bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >"]
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::systems::ui_clutter::debug_spawn_clutter::DebugSpawnAllClutterCmd>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::systems::ui_clutter::debug_spawn_clutter::DebugSpawnClutterCmd>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::systems::ui_clutter::ui_autoclutter::InstaCreateDeco>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::history::History>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_state::StairsState>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator::DeferredDecoratorOpEvents>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >", "bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_state::StairsState>", "bevy_ecs::change_detection::Res<country_core::resources::walls::decorator::DeferredDecoratorOpEvents>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_state::StairsState>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::decorator::DeferredDecoratorOpEvents>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::history::replay_mode::RestoreAppTo>,bevy_ecs::change_detection::ResMut<country_core::resources::toolbar::Toolbar>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::ClearCanvasCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::mode_manager::UiHistoryPlaySelectAnimation>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::mode_manager::RequestAppModeChange>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::mode_manager::RequestToolbarCategoryChange>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::mode_manager::RequestHistoryCommand>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::message::message_writer::MessageWriter<enum2$<country_core::systems::ui::resources::widget_params::PlayUiSound> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::TimeOfDayScreen>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::mode_manager::AppMode> >", "bevy_ecs::change_detection::ResMut<country_core::systems::history::replay_mode::RestoreAppTo>", "bevy_ecs::change_detection::ResMut<country_core::resources::toolbar::Toolbar>", "bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>", "bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::ClearCanvasCmd>", "bevy_ecs::message::message_writer::MessageWriter<country_core::systems::mode_manager::UiHistoryPlaySelectAnimation>", "bevy_ecs::message::message_writer::MessageWriter<country_core::systems::history::history_process::HistoryCmd>", "bevy_ecs::message::message_reader::MessageReader<country_core::systems::save::load_game::LoadingGameComplete>", "bevy_ecs::message::message_reader::MessageReader<country_core::systems::mode_manager::RequestAppModeChange>", "bevy_ecs::message::message_reader::MessageReader<country_core::systems::mode_manager::RequestToolbarCategoryChange>", "bevy_ecs::message::message_reader::MessageReader<country_core::systems::mode_manager::RequestHistoryCommand>", "bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>", "bevy_ecs::change_detection::Res<country_core::resources::history::History>", "bevy_ecs::message::message_writer::MessageWriter<enum2$<country_core::systems::ui::resources::widget_params::PlayUiSound> >", "bevy_ecs::change_detection::ResMut<country_core::resources::gui::TimeOfDayScreen>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::history::replay_mode::RestoreAppTo>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::toolbar::Toolbar>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::ClearCanvasCmd>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::systems::mode_manager::UiHistoryPlaySelectAnimation>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::systems::history::history_process::HistoryCmd>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::systems::save::load_game::LoadingGameComplete>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::systems::mode_manager::RequestAppModeChange>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::systems::mode_manager::RequestToolbarCategoryChange>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::systems::mode_manager::RequestHistoryCommand>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::history::History>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<enum2$<country_core::systems::ui::resources::widget_params::PlayUiSound> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::gui::TimeOfDayScreen>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,country_core::systems::collision::world::Raycasting,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>,bevy_ecs::change_detection::ResMut<system_decorator::ui_systems::ui_decorator_affordance_dispatch::DecoratorAffordanceDispatcher>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_state::StairsState>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<system_decorator::blueprint::PrevDecoratorBlueprints>,bevy_ecs::change_detection::Res<country_core::systems::mod_key::Modifiers>,tuple$<bevy_ecs::message::message_reader::MessageReader<country_core::systems::ui_clutter::ui_autoclutter::StartPlaceDecoClutterInteraction>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::ui_clutter::ui_focus_decoclutter::RequestMoveClutter>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::ui_clutter::ui_focus_decoclutter::RequestRotateDecorator>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::onboarding::NotifyHintSystemCanContinueStairs> >,bevy_ecs::change_detection::Res<country_core::systems::ui_clutter::ui_focus_decoclutter::LastClonedClutter>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_state::StairsState>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >", "country_core::systems::collision::world::Raycasting", "bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>", "bevy_ecs::change_detection::ResMut<system_decorator::ui_systems::ui_decorator_affordance_dispatch::DecoratorAffordanceDispatcher>", "bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>", "bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>", "tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_state::StairsState>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms> >", "bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >", "bevy_ecs::change_detection::Res<system_decorator::blueprint::PrevDecoratorBlueprints>", "bevy_ecs::change_detection::Res<country_core::systems::mod_key::Modifiers>", "tuple$<bevy_ecs::message::message_reader::MessageReader<country_core::systems::ui_clutter::ui_autoclutter::StartPlaceDecoClutterInteraction>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::ui_clutter::ui_focus_decoclutter::RequestMoveClutter>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::ui_clutter::ui_focus_decoclutter::RequestRotateDecorator>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::onboarding::NotifyHintSystemCanContinueStairs> >", "bevy_ecs::change_detection::Res<country_core::systems::ui_clutter::ui_focus_decoclutter::LastClonedClutter>", "bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_state::StairsState>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_decorator::ui_systems::ui_decorator_affordance_dispatch::DecoratorAffordanceDispatcher>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_state::StairsState>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<system_decorator::blueprint::PrevDecoratorBlueprints>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::mod_key::Modifiers>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::systems::ui_clutter::ui_autoclutter::StartPlaceDecoClutterInteraction>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::systems::ui_clutter::ui_focus_decoclutter::RequestMoveClutter>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::systems::ui_clutter::ui_focus_decoclutter::RequestRotateDecorator>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::resources::onboarding::NotifyHintSystemCanContinueStairs>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui_clutter::ui_focus_decoclutter::LastClonedClutter>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_state::StairsState>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::audio::forest_audio::ForestAudioState>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::tree_storage::InteractableForestTree>,ref$<country_core::systems::prefab::PrefabInstanceData> >,tuple$<> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::systems::audio::forest_audio::ForestAudioState>", "bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::tree_storage::InteractableForestTree>,ref$<country_core::systems::prefab::PrefabInstanceData> >,tuple$<> >", "bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::audio::forest_audio::ForestAudioState>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::tree_storage::InteractableForestTree>,ref$<country_core::systems::prefab::PrefabInstanceData> >,tuple$<> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,alloc::vec::Vec<country_core::systems::render_loop::compute_passes::image_baker::PendingImageBake,alloc::alloc::Global> > (*)()
// 候选参数类型: []
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: country_core::shader::apply_screen_shadows::impl$2::bind_entry_point<rhapsody::rg::binding::RgProtocol,rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder>
// 信息 ID: 7c7297e44817b49987243b4d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::shader::apply_screen_shadows::impl$2::bind_entry_point -> ["rhapsody::rg::binding::RgProtocol", "rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<country_core::shader::apply_screen_shadows::main<rhapsody::rg::binding::RgProtocol> > @ 0x178b3cd -> 0xb75a60 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_namespace_source_candidate; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: country_core::shader::apply_volumetrics::impl$2::bind_entry_point<rhapsody::rg::binding::RgProtocol,rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder>
// 信息 ID: 961318b8c086df5cc2ad597d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::shader::apply_volumetrics::impl$2::bind_entry_point -> ["rhapsody::rg::binding::RgProtocol", "rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<country_core::shader::apply_volumetrics::main<rhapsody::rg::binding::RgProtocol> > @ 0x177d55b -> 0x14f9e10 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_namespace_source_candidate; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: country_core::shader::blur_downsample::impl$2::bind_entry_point<rhapsody::rg::binding::RgProtocol,rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder>
// 信息 ID: 84b22cd3f4c64948922025c3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::shader::blur_downsample::impl$2::bind_entry_point -> ["rhapsody::rg::binding::RgProtocol", "rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<country_core::shader::blur_downsample::main<rhapsody::rg::binding::RgProtocol> > | rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<country_core::shader::blur_downsample::cs_main_multisampled<rhapsody::rg::binding::RgProtocol> > @ 0x47770c -> 0x21f220 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_namespace_source_candidate; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: country_core::shader::buildings::inc::building_mask::impl$0::descriptors<shabriri_rt::api::EmptyProtocol>
// 信息 ID: 03b2aae05b296871547226e4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::shader::buildings::inc::building_mask::impl$0::descriptors -> ["shabriri_rt::api::EmptyProtocol"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] shaderface::binding_set::BindingSetDesc::with<country_core::shader::buildings::inc::building_mask::BuildingMaskTex<shabriri_rt::api::EmptyProtocol> > @ 0x1a67dc0 -> 0xad59c0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_namespace_source_candidate; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: country_core::shader::capture_panic_screenshot::impl$2::bind_entry_point<rhapsody::rg::binding::RgProtocol,rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder>
// 信息 ID: c3fc1c696495a82a737e2400
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::shader::capture_panic_screenshot::impl$2::bind_entry_point -> ["rhapsody::rg::binding::RgProtocol", "rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<country_core::shader::capture_panic_screenshot::main<rhapsody::rg::binding::RgProtocol> > @ 0x477126 -> 0x5368c0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_namespace_source_candidate; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: country_core::shader::compute_froxel_lighting::impl$2::bind_entry_point<rhapsody::rg::binding::RgProtocol,rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder>
// 信息 ID: a1043070b279c6763f99121e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::shader::compute_froxel_lighting::impl$2::bind_entry_point -> ["rhapsody::rg::binding::RgProtocol", "rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<country_core::shader::compute_froxel_lighting::main<rhapsody::rg::binding::RgProtocol> > @ 0x1797d8d -> 0x9029d0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_namespace_source_candidate; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: country_core::shader::compute_sky_visibility_volume::impl$2::bind_entry_point<rhapsody::rg::binding::RgProtocol,rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder>
// 信息 ID: e34849997e00780c699d00f3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::shader::compute_sky_visibility_volume::impl$2::bind_entry_point -> ["rhapsody::rg::binding::RgProtocol", "rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<country_core::shader::compute_sky_visibility_volume::main<rhapsody::rg::binding::RgProtocol> > @ 0x177f095 -> 0x904000 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_namespace_source_candidate; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: country_core::shader::compute_volumetric_sm::impl$2::bind_entry_point<rhapsody::rg::binding::RgProtocol,rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder>
// 信息 ID: 21a9af2524f552407f1ec5f2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::shader::compute_volumetric_sm::impl$2::bind_entry_point -> ["rhapsody::rg::binding::RgProtocol", "rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<country_core::shader::compute_volumetric_sm::main<rhapsody::rg::binding::RgProtocol> > @ 0x1778ddb -> 0xa63d20 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_namespace_source_candidate; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: country_core::shader::critters::duck_animated::impl$6::bind<rhapsody::raw::binding_set::RawProtocol,rhapsody::raw::binding_set::RawBinder>
// 信息 ID: a1ed7ad1f4d5d039dd1645f9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::shader::critters::duck_animated::impl$6::bind -> ["rhapsody::raw::binding_set::RawProtocol", "rhapsody::raw::binding_set::RawBinder"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::update_binding_set<country_core::shader::critters::duck_animated::MaterialData> @ 0x155ed58 -> 0xa04b40 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::update_binding_set<country_core::shader::critters::duck_animated::MaterialData> @ 0x155efb5 -> 0xa04b40 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_namespace_source_candidate; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: country_core::shader::critters::firefly_vis_query::impl$2::bind_entry_point<rhapsody::rg::binding::RgProtocol,rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder>
// 信息 ID: 7a9547ca4aa2e455adf110b4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::shader::critters::firefly_vis_query::impl$2::bind_entry_point -> ["rhapsody::rg::binding::RgProtocol", "rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<country_core::shader::critters::firefly_vis_query::main<rhapsody::rg::binding::RgProtocol> > @ 0x17852dc -> 0x1bf2d90 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_namespace_source_candidate; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: country_core::shader::deferred::solid_light_sun_sky::impl$2::bind_entry_point<rhapsody::rg::binding::RgProtocol,rhapsody::rg::binding::RgEntryBinder>
// 信息 ID: 483b742e974da69b16451983
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::shader::deferred::solid_light_sun_sky::impl$2::bind_entry_point -> ["rhapsody::rg::binding::RgProtocol", "rhapsody::rg::binding::RgEntryBinder"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::node_builder::draw_stream::RasterStreamRecorder::shader_with_compatible_args<country_core::shader::deferred::solid_light_sun_sky::vs_main<rhapsody::rg::binding::RgProtocol> > @ 0x197dae2 -> 0xae00b0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_namespace_source_candidate; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: country_core::shader::dof::calc_autofocus::impl$2::bind_entry_point<rhapsody::rg::binding::RgProtocol,rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder>
// 信息 ID: cff0a9b67a0f1081b1304546
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::shader::dof::calc_autofocus::impl$2::bind_entry_point -> ["rhapsody::rg::binding::RgProtocol", "rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<country_core::shader::dof::calc_autofocus::main<rhapsody::rg::binding::RgProtocol> > @ 0x1780baa -> 0xad3ff0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_namespace_source_candidate; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: country_core::shader::dof::prepare::impl$2::bind_entry_point<rhapsody::rg::binding::RgProtocol,rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder>
// 信息 ID: b38759c1fc523e1311836b9b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::shader::dof::prepare::impl$2::bind_entry_point -> ["rhapsody::rg::binding::RgProtocol", "rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<country_core::shader::dof::prepare::main<rhapsody::rg::binding::RgProtocol> > @ 0x1789043 -> 0xb10690 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_namespace_source_candidate; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: country_core::shader::extract_half_res_depth_normals::impl$2::bind_entry_point<rhapsody::rg::binding::RgProtocol,rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder>
// 信息 ID: f6e771ccb50440cc2f31346c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::shader::extract_half_res_depth_normals::impl$2::bind_entry_point -> ["rhapsody::rg::binding::RgProtocol", "rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<country_core::shader::extract_half_res_depth_normals::main<rhapsody::rg::binding::RgProtocol> > @ 0x178f7d7 -> 0x1ab9760 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_namespace_source_candidate; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: country_core::shader::final_post::impl$2::bind_entry_point<rhapsody::rg::binding::RgProtocol,rhapsody::rg::binding::RgEntryBinder>
// 信息 ID: 116cc9e76a2a1a49e034df7a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::shader::final_post::impl$2::bind_entry_point -> ["rhapsody::rg::binding::RgProtocol", "rhapsody::rg::binding::RgEntryBinder"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::node_builder::draw_stream::RasterStreamRecorder::shader<country_core::shader::final_post::vs_main<rhapsody::rg::binding::RgProtocol> > @ 0x1992e40 -> 0x1805080 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_namespace_source_candidate; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: country_core::shader::garden::marching_squares::impl$2::bind_entry_point<rhapsody::rg::binding::RgProtocol,rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder>
// 信息 ID: 12035ed9cdd188c23ad51bc7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::shader::garden::marching_squares::impl$2::bind_entry_point -> ["rhapsody::rg::binding::RgProtocol", "rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<country_core::shader::garden::marching_squares::main<rhapsody::rg::binding::RgProtocol> > @ 0x1787e23 -> 0x19bab40 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_namespace_source_candidate; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: country_core::shader::garden::read_bush_positions_for_cpu::impl$2::bind_entry_point<rhapsody::rg::binding::RgProtocol,rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder>
// 信息 ID: 59d510cc64e120202a6943e8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::shader::garden::read_bush_positions_for_cpu::impl$2::bind_entry_point -> ["rhapsody::rg::binding::RgProtocol", "rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<country_core::shader::garden::read_bush_positions_for_cpu::main<rhapsody::rg::binding::RgProtocol> > @ 0x17830db -> 0x9dd9f0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_namespace_source_candidate; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: country_core::shader::garden::spawn_bushes_buildings::impl$2::bind_entry_point<rhapsody::rg::binding::RgProtocol,rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder>
// 信息 ID: d90e5dbc5361b8e88fffe6ce
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::shader::garden::spawn_bushes_buildings::impl$2::bind_entry_point -> ["rhapsody::rg::binding::RgProtocol", "rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<country_core::shader::garden::spawn_bushes_buildings::main<rhapsody::rg::binding::RgProtocol> > | rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<country_core::shader::garden::spawn_bushes_garden::main<rhapsody::rg::binding::RgProtocol> > @ 0x17836db -> 0xb99fd0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_namespace_source_candidate; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: country_core::shader::garden::spawn_small_flowers_garden::impl$2::bind_entry_point<rhapsody::rg::binding::RgProtocol,rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder>
// 信息 ID: 4ed3503404d18007d8d1c79b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::shader::garden::spawn_small_flowers_garden::impl$2::bind_entry_point -> ["rhapsody::rg::binding::RgProtocol", "rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<country_core::shader::garden::spawn_flowers_buildings::main<rhapsody::rg::binding::RgProtocol> > | rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<country_core::shader::garden::spawn_small_flowers_garden::main<rhapsody::rg::binding::RgProtocol> > | rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<country_core::shader::garden::spawn_flowers_garden::main<rhapsody::rg::binding::RgProtocol> > | rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<country_core::shader::water::spawn_reeds::main<rhapsody::rg::binding::RgProtocol> > @ 0x1779763 -> 0xb364a0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_namespace_source_candidate; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: country_core::shader::gauss_blur::impl$2::bind_entry_point<rhapsody::rg::binding::RgProtocol,rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder>
// 信息 ID: 65e567d37655acb88c7c8bbb
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::shader::gauss_blur::impl$2::bind_entry_point -> ["rhapsody::rg::binding::RgProtocol", "rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<country_core::shader::gauss_blur::main<rhapsody::rg::binding::RgProtocol> > @ 0x17858cb -> 0xc7c220 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_namespace_source_candidate; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: country_core::shader::gauss_blur::impl$3::to_shader_permutation_key<rhapsody::rg::binding::RgProtocol>
// 信息 ID: 15d864f754174afd27d03f62
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::shader::gauss_blur::impl$3::to_shader_permutation_key -> ["rhapsody::rg::binding::RgProtocol"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::rasterize_terrain_stroke @ 0x9b8358 -> 0xc7c9e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::rasterize_terrain_stroke @ 0x9b8451 -> 0xc7c9e0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_namespace_source_candidate; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: country_core::shader::gauss_blur_building_mask::impl$2::bind_entry_point<rhapsody::rg::binding::RgProtocol,rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder>
// 信息 ID: 8df0b40f780e9c4211778d7c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::shader::gauss_blur_building_mask::impl$2::bind_entry_point -> ["rhapsody::rg::binding::RgProtocol", "rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<country_core::shader::gauss_blur_building_mask::main<rhapsody::rg::binding::RgProtocol> > @ 0x178fdc2 -> 0xc7d1c0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_namespace_source_candidate; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: country_core::shader::gauss_blur_mask::impl$2::bind_entry_point<rhapsody::rg::binding::RgProtocol,rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder>
// 信息 ID: b6a0b745a43aadc144c29c14
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::shader::gauss_blur_mask::impl$2::bind_entry_point -> ["rhapsody::rg::binding::RgProtocol", "rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<country_core::shader::gauss_blur_mask::main<rhapsody::rg::binding::RgProtocol> > @ 0x1792d5f -> 0xc7c4a0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_namespace_source_candidate; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: country_core::shader::gauss_blur_r16f::impl$2::bind_entry_point<rhapsody::rg::binding::RgProtocol,rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder>
// 信息 ID: 623f9722a9d36d1d902662f3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::shader::gauss_blur_r16f::impl$2::bind_entry_point -> ["rhapsody::rg::binding::RgProtocol", "rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<country_core::shader::gauss_blur_r16f::main<rhapsody::rg::binding::RgProtocol> > @ 0x1798993 -> 0xc7c750 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_namespace_source_candidate; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: country_core::shader::generate_hzb::impl$2::bind_entry_point<rhapsody::rg::binding::RgProtocol,rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder>
// 信息 ID: 93cffc28e29912884889b5b9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::shader::generate_hzb::impl$2::bind_entry_point -> ["rhapsody::rg::binding::RgProtocol", "rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<country_core::shader::generate_hzb::main<rhapsody::rg::binding::RgProtocol> > @ 0x1796ae7 -> 0x9d8170 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_namespace_source_candidate; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: country_core::shader::generate_hzb::impl$3::to_shader_permutation_key<rhapsody::rg::binding::RgProtocol>
// 信息 ID: 2f92c3beffd593cfb453c1ed
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::shader::generate_hzb::impl$3::to_shader_permutation_key -> ["rhapsody::rg::binding::RgProtocol"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::render_loop::compute_passes::generate_hzb::generate_hzb @ 0x9b38ef -> 0x9d8900 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_namespace_source_candidate; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: country_core::shader::gi::trace_gi_candidates::impl$2::bind_entry_point<rhapsody::rg::binding::RgProtocol,rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder>
// 信息 ID: 4f1d023d08e2e73eed4faf59
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::shader::gi::trace_gi_candidates::impl$2::bind_entry_point -> ["rhapsody::rg::binding::RgProtocol", "rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<country_core::shader::gi::trace_gi_candidates::main<rhapsody::rg::binding::RgProtocol> > @ 0x178bfe3 -> 0x903540 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_namespace_source_candidate; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: country_core::shader::grass2::active_cull_dispatch_args::impl$2::bind_entry_point<rhapsody::rg::binding::RgProtocol,rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder>
// 信息 ID: 868d9cd715cb0b9cfa97ee70
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::shader::grass2::active_cull_dispatch_args::impl$2::bind_entry_point -> ["rhapsody::rg::binding::RgProtocol", "rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<country_core::shader::grass2::active_cull_dispatch_args::main<rhapsody::rg::binding::RgProtocol> > @ 0x177f8ad -> 0x1ab69f0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_namespace_source_candidate; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: country_core::shader::grass2::bake_world_albedo_tint::impl$2::bind_entry_point<rhapsody::rg::binding::RgProtocol,rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder>
// 信息 ID: 84ce9f3d3bd4bebc12bd7cd2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::shader::grass2::bake_world_albedo_tint::impl$2::bind_entry_point -> ["rhapsody::rg::binding::RgProtocol", "rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<country_core::shader::grass2::bake_world_albedo_tint::main<rhapsody::rg::binding::RgProtocol> > @ 0x1797723 -> 0xbb3020 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_namespace_source_candidate; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: country_core::shader::grass2::build_draws::impl$2::bind_entry_point<rhapsody::rg::binding::RgProtocol,rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder>
// 信息 ID: a8f872e30e572f87a699ef81
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::shader::grass2::build_draws::impl$2::bind_entry_point -> ["rhapsody::rg::binding::RgProtocol", "rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<country_core::shader::grass2::build_draws::shadow<rhapsody::rg::binding::RgProtocol> > | rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<country_core::shader::grass2::build_draws::main<rhapsody::rg::binding::RgProtocol> > @ 0x1783cdf -> 0x17fa120 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_namespace_source_candidate; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: country_core::shader::grass2::card_splat_cull::impl$2::bind_entry_point<rhapsody::rg::binding::RgProtocol,rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder>
// 信息 ID: cc4ed80d5baf903c669f98c1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::shader::grass2::card_splat_cull::impl$2::bind_entry_point -> ["rhapsody::rg::binding::RgProtocol", "rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<country_core::shader::grass2::card_splat_cull::main<rhapsody::rg::binding::RgProtocol> > @ 0x17948e4 -> 0xb4d610 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_namespace_source_candidate; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: country_core::shader::grass2::card_splat_resolve::impl$2::bind_entry_point<rhapsody::rg::binding::RgProtocol,rhapsody::rg::binding::RgEntryBinder>
// 信息 ID: 7f6f652bdf50209ca3e34939
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::shader::grass2::card_splat_resolve::impl$2::bind_entry_point -> ["rhapsody::rg::binding::RgProtocol", "rhapsody::rg::binding::RgEntryBinder"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::node_builder::draw_stream::RasterStreamRecorder::shader<country_core::shader::grass2::card_splat_resolve::vs_main<rhapsody::rg::binding::RgProtocol> > @ 0x1982f56 -> 0xc29830 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_namespace_source_candidate; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: country_core::shader::grass2::card_splat_scan_args::impl$2::bind_entry_point<rhapsody::rg::binding::RgProtocol,rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder>
// 信息 ID: 88bf724034ce9c2df48e2ec7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::shader::grass2::card_splat_scan_args::impl$2::bind_entry_point -> ["rhapsody::rg::binding::RgProtocol", "rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<country_core::shader::grass2::card_splat_scan_args::main<rhapsody::rg::binding::RgProtocol> > @ 0x17938d0 -> 0xa64b20 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_namespace_source_candidate; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: country_core::shader::grass2::card_splat_scan_finalize::impl$2::bind_entry_point<rhapsody::rg::binding::RgProtocol,rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder>
// 信息 ID: 3c18b200afe053641c45dae8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::shader::grass2::card_splat_scan_finalize::impl$2::bind_entry_point -> ["rhapsody::rg::binding::RgProtocol", "rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<country_core::shader::grass2::card_splat_scan_finalize::main<rhapsody::rg::binding::RgProtocol> > @ 0x179951e -> 0x1ab6550 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_namespace_source_candidate; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: country_core::shader::grass2::card_splat_shadow_cull::impl$2::bind_entry_point<rhapsody::rg::binding::RgProtocol,rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder>
// 信息 ID: 09adba81f64847bb2a321aa9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::shader::grass2::card_splat_shadow_cull::impl$2::bind_entry_point -> ["rhapsody::rg::binding::RgProtocol", "rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<country_core::shader::grass2::card_splat_shadow_cull::main<rhapsody::rg::binding::RgProtocol> > @ 0x17811e4 -> 0xc18400 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_namespace_source_candidate; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: country_core::shader::grass2::card_splat_workgroup_header::impl$2::bind_entry_point<rhapsody::rg::binding::RgProtocol,rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder>
// 信息 ID: da7204469b7e8e61291587fa
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::shader::grass2::card_splat_workgroup_header::impl$2::bind_entry_point -> ["rhapsody::rg::binding::RgProtocol", "rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<country_core::shader::grass2::card_splat_workgroup_header::main<rhapsody::rg::binding::RgProtocol> > | rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<country_core::shader::grass2::card_splat_scan_gather::main<rhapsody::rg::binding::RgProtocol> > @ 0x178a7c5 -> 0x936140 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_namespace_source_candidate; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: country_core::shader::grass2::card_splats::impl$2::bind_entry_point<rhapsody::rg::binding::RgProtocol,rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder>
// 信息 ID: 4ea903a23f22df787baf21ad
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::shader::grass2::card_splats::impl$2::bind_entry_point -> ["rhapsody::rg::binding::RgProtocol", "rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<country_core::shader::grass2::card_splats::prepass<rhapsody::rg::binding::RgProtocol> > @ 0x1779d70 -> 0x1167df0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_namespace_source_candidate; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: country_core::shader::grass2::card_splats_clear::impl$2::bind_entry_point<rhapsody::rg::binding::RgProtocol,rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder>
// 信息 ID: 535dd8003f60a6cfa948a782
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::shader::grass2::card_splats_clear::impl$2::bind_entry_point -> ["rhapsody::rg::binding::RgProtocol", "rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<country_core::shader::grass2::card_splats_clear::clear<rhapsody::rg::binding::RgProtocol> > @ 0x1786b6b -> 0xb77d40 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_namespace_source_candidate; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: country_core::shader::grass2::card_splats_shadow::impl$2::bind_entry_point<rhapsody::rg::binding::RgProtocol,rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder>
// 信息 ID: 0432f4da3787d0b7d386404d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::shader::grass2::card_splats_shadow::impl$2::bind_entry_point -> ["rhapsody::rg::binding::RgProtocol", "rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<country_core::shader::grass2::card_splats_shadow::prepass<rhapsody::rg::binding::RgProtocol> > @ 0x177c941 -> 0x1bf30d0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_namespace_source_candidate; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: country_core::shader::grass2::combine_grass_removal::impl$2::bind_entry_point<rhapsody::rg::binding::RgProtocol,rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder>
// 信息 ID: 3aa8ffbcd7165b39a6f5efc8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::shader::grass2::combine_grass_removal::impl$2::bind_entry_point -> ["rhapsody::rg::binding::RgProtocol", "rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<country_core::shader::grass2::combine_grass_removal::main<rhapsody::rg::binding::RgProtocol> > @ 0x178cfd1 -> 0xaef7f0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_namespace_source_candidate; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: country_core::shader::grass2::inc::active_instances::impl$0::descriptors<shabriri_rt::api::EmptyProtocol>
// 信息 ID: 93b2b987d98fd7376b330abe
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::shader::grass2::inc::active_instances::impl$0::descriptors -> ["shabriri_rt::api::EmptyProtocol"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] shaderface::binding_set::BindingSetDesc::with<country_core::shader::grass2::inc::active_instances::Grass2ActiveInstances<shabriri_rt::api::EmptyProtocol> > @ 0x1a67600 -> 0xb7b270 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_namespace_source_candidate; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: country_core::shader::grass2::inc::card_splat_common::impl$12::descriptors<shabriri_rt::api::EmptyProtocol>
// 信息 ID: df38c5aba627b9377b45f94e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::shader::grass2::inc::card_splat_common::impl$12::descriptors -> ["shabriri_rt::api::EmptyProtocol"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] shaderface::binding_set::BindingSetDesc::with<country_core::shader::grass2::inc::card_splat_common::Grass2ExemplarParamsBlock<shabriri_rt::api::EmptyProtocol> > @ 0x1a69120 -> 0xaf3030 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_namespace_source_candidate; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: country_core::shader::ice_plane::impl$2::bind_entry_point<rhapsody::rg::binding::RgProtocol,rhapsody::rg::binding::RgEntryBinder>
// 信息 ID: 92589615bdc01df386002f72
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::shader::ice_plane::impl$2::bind_entry_point -> ["rhapsody::rg::binding::RgProtocol", "rhapsody::rg::binding::RgEntryBinder"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::node_builder::draw_stream::RasterStreamRecorder::prebuilt_pipeline_with_compatible_args<country_core::shader::ice_plane::vs_main<rhapsody::rg::binding::RgProtocol> > @ 0x1981a54 -> 0x1b26510 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_namespace_source_candidate; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: country_core::shader::inc::color_library::impl$3::descriptors<shabriri_rt::api::EmptyProtocol>
// 信息 ID: ad5f19a861d8a193006fce3a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::shader::inc::color_library::impl$3::descriptors -> ["shabriri_rt::api::EmptyProtocol"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] shaderface::binding_set::BindingSetDesc::with<country_core::shader::inc::color_library::ColorLibrary<shabriri_rt::api::EmptyProtocol> > @ 0x1a68390 -> 0xa04cd0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_namespace_source_candidate; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: country_core::shader::inc::flag::impl$0::descriptors<shabriri_rt::api::EmptyProtocol>
// 信息 ID: 787ef7a7f82cd5678b23ef0f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::shader::inc::flag::impl$0::descriptors -> ["shabriri_rt::api::EmptyProtocol"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] shaderface::binding_set::BindingSetDesc::with<country_core::shader::inc::flag::FlagTextures<shabriri_rt::api::EmptyProtocol> > @ 0x13f430 -> 0x5da6a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] shaderface::binding_set::BindingSetDesc::with<country_core::shader::inc::flag::FlagTextures<shabriri_rt::api::EmptyProtocol> > @ 0x1a69310 -> 0x5da6a0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_namespace_source_candidate; 作者定义文件是否精确确认仍按证据状态。
// [文件映射] pdb_symbolizer_namespace_source_candidate; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: country_core::shader::inc::frame::impl$2::bind<rhapsody::rg::binding::RgProtocol,rhapsody::rg::api::RgBindingSetBinder>
// 信息 ID: 3d8523571d1477e82d72f82b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::shader::inc::frame::impl$2::bind -> ["rhapsody::rg::binding::RgProtocol", "rhapsody::rg::api::RgBindingSetBinder"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::api::GraphApi::push_instant_binding_set<country_core::shader::inc::frame::FrameConstants,country_core::shader::inc::frame::FrameConstants> @ 0x457cac -> 0x1809b0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_namespace_source_candidate; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: country_core::shader::inc::lantern_lighting::impl$5::bind<rhapsody::rg::binding::RgProtocol,rhapsody::rg::node_builder::compute_pass_builder::RgComputeBinder>
// 信息 ID: 8a0cd290d506c6e9ca08f0e9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::shader::inc::lantern_lighting::impl$5::bind -> ["rhapsody::rg::binding::RgProtocol", "rhapsody::rg::node_builder::compute_pass_builder::RgComputeBinder"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::shader::compute_froxel_lighting::impl$2::bind_entry_point<rhapsody::rg::binding::RgProtocol,rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder> @ 0x9032ec -> 0x17f9e20 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_namespace_source_candidate; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: country_core::shader::inc::object_push::impl$0::descriptors<shabriri_rt::api::EmptyProtocol>
// 信息 ID: 9b6ac10ec5a9050847470639
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::shader::inc::object_push::impl$0::descriptors -> ["shabriri_rt::api::EmptyProtocol"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] shaderface::binding_set::BindingSetDesc::with<country_core::shader::inc::object_push::ObjectPush<shabriri_rt::api::EmptyProtocol> > @ 0x1a681a0 -> 0xad3c50 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_namespace_source_candidate; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: country_core::shader::inc::reflection_fog::impl$5::bind<rhapsody::rg::binding::RgProtocol,rhapsody::rg::node_builder::compute_pass_builder::RgComputeBinder>
// 信息 ID: 851b747202af8214f517a516
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::shader::inc::reflection_fog::impl$5::bind -> ["rhapsody::rg::binding::RgProtocol", "rhapsody::rg::node_builder::compute_pass_builder::RgComputeBinder"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::shader::water::deferred_rt::impl$2::bind_entry_point<rhapsody::rg::binding::RgProtocol,rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder> @ 0x9cc73d -> 0x19db9b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::shader::water::ice_refl_march::impl$2::bind_entry_point<rhapsody::rg::binding::RgProtocol,rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder> @ 0x19e7520 -> 0x19db9b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::shader::water::ice_deferred_rt::impl$2::bind_entry_point<rhapsody::rg::binding::RgProtocol,rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder> @ 0x1ab9232 -> 0x19db9b0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_namespace_source_candidate; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: country_core::shader::inc::roof_color_data::impl$0::descriptors<shabriri_rt::api::EmptyProtocol>
// 信息 ID: f5ba543f3124baf7623a2853
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::shader::inc::roof_color_data::impl$0::descriptors -> ["shabriri_rt::api::EmptyProtocol"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] shaderface::binding_set::BindingSetDesc::with<country_core::shader::inc::roof_color_data::RoofColors<shabriri_rt::api::EmptyProtocol> > @ 0x13f050 -> 0x3c36c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] shaderface::binding_set::BindingSetDesc::with<country_core::shader::inc::roof_color_data::RoofColors<shabriri_rt::api::EmptyProtocol> > @ 0x1a6a480 -> 0x3c36c0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_namespace_source_candidate; 作者定义文件是否精确确认仍按证据状态。
// [文件映射] pdb_symbolizer_namespace_source_candidate; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: country_core::shader::inc::rt_defs::impl$9::descriptors<shabriri_rt::api::EmptyProtocol>
// 信息 ID: 04badbedd2e675b4d7217097
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::shader::inc::rt_defs::impl$9::descriptors -> ["shabriri_rt::api::EmptyProtocol"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] shaderface::binding_set::BindingSetDesc::with<country_core::shader::inc::rt_defs::RtScene<shabriri_rt::api::EmptyProtocol> > @ 0x13fde0 -> 0x180da0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_namespace_source_candidate; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: country_core::shader::inc::shadow::impl$0::descriptors<shabriri_rt::api::EmptyProtocol>
// 信息 ID: 68dbb2223781916e9b7a90e7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::shader::inc::shadow::impl$0::descriptors -> ["shabriri_rt::api::EmptyProtocol"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] shaderface::binding_set::BindingSetDesc::with<country_core::shader::inc::shadow::SunViews<shabriri_rt::api::EmptyProtocol> > @ 0x13ec70 -> 0x5accd0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_namespace_source_candidate; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: country_core::shader::inc::stone_foundation_color_customization::impl$0::descriptors<shabriri_rt::api::EmptyProtocol>
// 信息 ID: ba0843c5828482b9afd4715b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::shader::inc::stone_foundation_color_customization::impl$0::descriptors -> ["shabriri_rt::api::EmptyProtocol"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] shaderface::binding_set::BindingSetDesc::with<country_core::shader::inc::stone_foundation_color_customization::StoneFoundationColors<shabriri_rt::api::EmptyProtocol> > @ 0x13f240 -> 0x1ef980 (direct_call); 同地址别名未裁定。
// [静态直接引用] shaderface::binding_set::BindingSetDesc::with<country_core::shader::inc::stone_foundation_color_customization::StoneFoundationColors<shabriri_rt::api::EmptyProtocol> > @ 0x1a696f0 -> 0x1ef980 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_namespace_source_candidate; 作者定义文件是否精确确认仍按证据状态。
// [文件映射] pdb_symbolizer_namespace_source_candidate; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: country_core::shader::inc::terrain_height::impl$0::descriptors<shabriri_rt::api::EmptyProtocol>
// 信息 ID: 6a5789fb71ba2ce38bf49930
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::shader::inc::terrain_height::impl$0::descriptors -> ["shabriri_rt::api::EmptyProtocol"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] shaderface::binding_set::BindingSetDesc::with<country_core::shader::inc::terrain_height::TerrainHeightmap<shabriri_rt::api::EmptyProtocol> > @ 0x1a68b50 -> 0xad3d50 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_namespace_source_candidate; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: country_core::shader::inc::terrain_height::impl$3::descriptors<shabriri_rt::api::EmptyProtocol>
// 信息 ID: eda281a6e29fb1f9aa97e18a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::shader::inc::terrain_height::impl$3::descriptors -> ["shabriri_rt::api::EmptyProtocol"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] shaderface::binding_set::BindingSetDesc::with<country_core::shader::inc::terrain_height::VisualTerrainHeightmap<shabriri_rt::api::EmptyProtocol> > @ 0x1a68580 -> 0xad48e0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_namespace_source_candidate; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: country_core::shader::inc::volumetric_shadow::impl$5::bind<rhapsody::rg::binding::RgProtocol,rhapsody::rg::node_builder::compute_pass_builder::RgComputeBinder>
// 信息 ID: f79534cd37d3c45324f14a51
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::shader::inc::volumetric_shadow::impl$5::bind -> ["rhapsody::rg::binding::RgProtocol", "rhapsody::rg::node_builder::compute_pass_builder::RgComputeBinder"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 4 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::shader::compute_froxel_lighting::impl$2::bind_entry_point<rhapsody::rg::binding::RgProtocol,rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder> @ 0x90349b -> 0xc492a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::shader::compute_froxel_lighting::impl$2::bind_entry_point<rhapsody::rg::binding::RgProtocol,rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder> @ 0x9034e7 -> 0xc492a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::shader::apply_volumetrics::impl$2::bind_entry_point<rhapsody::rg::binding::RgProtocol,rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder> @ 0x14fa1f8 -> 0xc492a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::shader::inc::reflection_fog::impl$5::bind<rhapsody::rg::binding::RgProtocol,rhapsody::rg::node_builder::compute_pass_builder::RgComputeBinder> @ 0x19db9d6 -> 0xc492a0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_namespace_source_candidate; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: country_core::shader::inc::wall_color_customization::impl$0::descriptors<shabriri_rt::api::EmptyProtocol>
// 信息 ID: de32cf8b455aa6143c46ff1b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::shader::inc::wall_color_customization::impl$0::descriptors -> ["shabriri_rt::api::EmptyProtocol"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] shaderface::binding_set::BindingSetDesc::with<country_core::shader::inc::wall_color_customization::WallColorData<shabriri_rt::api::EmptyProtocol> > @ 0x13ee60 -> 0x58ce80 (direct_call); 同地址别名未裁定。
// [静态直接引用] shaderface::binding_set::BindingSetDesc::with<country_core::shader::inc::wall_color_customization::WallColorData<shabriri_rt::api::EmptyProtocol> > @ 0x1a6a0a0 -> 0x58ce80 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_namespace_source_candidate; 作者定义文件是否精确确认仍按证据状态。
// [文件映射] pdb_symbolizer_namespace_source_candidate; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: country_core::shader::inc::wall_slot_identity::impl$0::descriptors<shabriri_rt::api::EmptyProtocol>
// 信息 ID: 46cca54b6735e510a81f2053
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::shader::inc::wall_slot_identity::impl$0::descriptors -> ["shabriri_rt::api::EmptyProtocol"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] shaderface::binding_set::BindingSetDesc::with<country_core::shader::inc::wall_slot_identity::WallSlotIdentity<shabriri_rt::api::EmptyProtocol> > @ 0x13fa00 -> 0x21f590 (direct_call); 同地址别名未裁定。
// [静态直接引用] shaderface::binding_set::BindingSetDesc::with<country_core::shader::inc::wall_slot_identity::WallSlotIdentity<shabriri_rt::api::EmptyProtocol> > @ 0x1a67bd0 -> 0x21f590 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_namespace_source_candidate; 作者定义文件是否精确确认仍按证据状态。
// [文件映射] pdb_symbolizer_namespace_source_candidate; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: country_core::shader::inc::window_color_customization::impl$0::descriptors<shabriri_rt::api::EmptyProtocol>
// 信息 ID: 1eb5db100a7a165c28e052c1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::shader::inc::window_color_customization::impl$0::descriptors -> ["shabriri_rt::api::EmptyProtocol"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] shaderface::binding_set::BindingSetDesc::with<country_core::shader::inc::window_color_customization::WindowColorCustomization<shabriri_rt::api::EmptyProtocol> > @ 0x13f810 -> 0x78c60 (direct_call); 同地址别名未裁定。
// [静态直接引用] shaderface::binding_set::BindingSetDesc::with<country_core::shader::inc::window_color_customization::WindowColorCustomization<shabriri_rt::api::EmptyProtocol> > @ 0x1a68f30 -> 0x78c60 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_namespace_source_candidate; 作者定义文件是否精确确认仍按证据状态。
// [文件映射] pdb_symbolizer_namespace_source_candidate; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: country_core::shader::inc::wooden_wall_color_customization::impl$0::descriptors<shabriri_rt::api::EmptyProtocol>
// 信息 ID: 1d4f862e779243d1213f9a80
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::shader::inc::wooden_wall_color_customization::impl$0::descriptors -> ["shabriri_rt::api::EmptyProtocol"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] shaderface::binding_set::BindingSetDesc::with<country_core::shader::inc::wooden_wall_color_customization::WoodenWallColors<shabriri_rt::api::EmptyProtocol> > | shaderface::binding_set::BindingSetDesc::with<country_core::shader::inc::halftimber_color_customization::HalftimberColors<shabriri_rt::api::EmptyProtocol> > @ 0x13ea80 -> 0x41a050 (direct_call); 同地址别名未裁定。
// [静态直接引用] shaderface::binding_set::BindingSetDesc::with<country_core::shader::inc::wooden_wall_color_customization::WoodenWallColors<shabriri_rt::api::EmptyProtocol> > | shaderface::binding_set::BindingSetDesc::with<country_core::shader::inc::halftimber_color_customization::HalftimberColors<shabriri_rt::api::EmptyProtocol> > @ 0x1a679e0 -> 0x41a050 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_namespace_source_candidate; 作者定义文件是否精确确认仍按证据状态。
// [文件映射] pdb_symbolizer_namespace_source_candidate; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: country_core::shader::inc::world::impl$9::descriptors<shabriri_rt::api::EmptyProtocol>
// 信息 ID: 11abd9d7c32fab61a924f6b0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::shader::inc::world::impl$9::descriptors -> ["shabriri_rt::api::EmptyProtocol"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] shaderface::binding_set::BindingSetDesc::with<country_core::shader::water::inc::water_mask_sampler::WaterMask<shabriri_rt::api::EmptyProtocol> > | shaderface::binding_set::BindingSetDesc::with<country_core::shader::inc::terrain_lighting::TerrainLighting<shabriri_rt::api::EmptyProtocol> > | shaderface::binding_set::BindingSetDesc::with<country_core::shader::inc::world::PlantSampleMask<shabriri_rt::api::EmptyProtocol> > | shaderface::binding_set::BindingSetDesc::with<country_core::shader::inc::fog_noise::FogNoiseTexture<shabriri_rt::api::EmptyProtocol> > | 其余 6 个同地址别名见共享索引 @ 0x1a67220 -> 0x902890 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_namespace_source_candidate; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: country_core::shader::leaf_particles::impl$2::bind_entry_point<rhapsody::rg::binding::RgProtocol,rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder>
// 信息 ID: 759d76d7b5ac711c85f8ea83
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::shader::leaf_particles::impl$2::bind_entry_point -> ["rhapsody::rg::binding::RgProtocol", "rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<country_core::shader::leaf_particles::main<rhapsody::rg::binding::RgProtocol> > @ 0x177defc -> 0x1847d60 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_namespace_source_candidate; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: country_core::shader::light_binning::bin_lights_to_cells_1::impl$2::bind_entry_point<rhapsody::rg::binding::RgProtocol,rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder>
// 信息 ID: 028e778f3e927e01dcbe4345
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::shader::light_binning::bin_lights_to_cells_1::impl$2::bind_entry_point -> ["rhapsody::rg::binding::RgProtocol", "rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<country_core::shader::light_binning::bin_lights_to_cells_1::main<rhapsody::rg::binding::RgProtocol> > @ 0x178b9bb -> 0xa7af50 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_namespace_source_candidate; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: country_core::shader::light_binning::cell_subtotal_prefix_sum::impl$2::bind_entry_point<rhapsody::rg::binding::RgProtocol,rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder>
// 信息 ID: f4b22a5abdb069e958a8ff6e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::shader::light_binning::cell_subtotal_prefix_sum::impl$2::bind_entry_point -> ["rhapsody::rg::binding::RgProtocol", "rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<country_core::shader::light_binning::cell_subtotal_prefix_sum::main<rhapsody::rg::binding::RgProtocol> > | rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<country_core::shader::nani::clear_subtotals::main<rhapsody::rg::binding::RgProtocol> > @ 0x1794ed5 -> 0x9ddf00 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_namespace_source_candidate; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: country_core::shader::lut_gallery::thumbnail::impl$2::bind_entry_point<rhapsody::rg::binding::RgProtocol,rhapsody::rg::binding::RgEntryBinder>
// 信息 ID: 7f4cacafc82da1fed3175647
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::shader::lut_gallery::thumbnail::impl$2::bind_entry_point -> ["rhapsody::rg::binding::RgProtocol", "rhapsody::rg::binding::RgEntryBinder"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::node_builder::draw_stream::RasterStreamRecorder::shader<country_core::shader::lut_gallery::thumbnail::vs_main<rhapsody::rg::binding::RgProtocol> > @ 0x1985e13 -> 0xae1770 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_namespace_source_candidate; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: country_core::shader::nani::allocate_draw_commands::impl$2::bind_entry_point<rhapsody::rg::binding::RgProtocol,rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder>
// 信息 ID: 8c75b07eaa1690f49f37da96
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::shader::nani::allocate_draw_commands::impl$2::bind_entry_point -> ["rhapsody::rg::binding::RgProtocol", "rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<country_core::shader::nani::allocate_draw_commands::main<rhapsody::rg::binding::RgProtocol> > @ 0x1793cc5 -> 0x92e3c0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_namespace_source_candidate; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: country_core::shader::nani::clear_draw_lists::impl$2::bind_entry_point<rhapsody::rg::binding::RgProtocol,rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder>
// 信息 ID: 8869d22e1924e1551845c2aa
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::shader::nani::clear_draw_lists::impl$2::bind_entry_point -> ["rhapsody::rg::binding::RgProtocol", "rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<country_core::shader::nani::clear_draw_lists::main<rhapsody::rg::binding::RgProtocol> > @ 0x178f1e4 -> 0x1ab2010 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_namespace_source_candidate; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: country_core::shader::nani::cull_and_bucket::impl$2::bind_entry_point<rhapsody::rg::binding::RgProtocol,rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder>
// 信息 ID: 8e5faa3712401922e33f6422
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::shader::nani::cull_and_bucket::impl$2::bind_entry_point -> ["rhapsody::rg::binding::RgProtocol", "rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<country_core::shader::nani::cull_and_bucket::main<rhapsody::rg::binding::RgProtocol> > @ 0x1782acd -> 0x1847f80 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_namespace_source_candidate; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: country_core::shader::nani::cull_and_bucket::impl$3::to_shader_permutation_key<rhapsody::rg::binding::RgProtocol>
// 信息 ID: 772811e587c3b89354067bda
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::shader::nani::cull_and_bucket::impl$3::to_shader_permutation_key -> ["rhapsody::rg::binding::RgProtocol"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 4 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::gpu_generate_draw_lists @ 0x9bc930 -> 0x9d7390 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::cull_and_bucket_external<country_core::resources::roofs::roof_buffers::WoodenPlankSsbo,country_core::render::nani::cull_shaders::NaniCullAndBucketWoodenPlank> @ 0xc309c3 -> 0x9d7390 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::cull_and_bucket_external<country_core::resources::roof_tile_ssbo::RoofTileSsbo,country_core::render::nani::cull_shaders::NaniCullAndBucketRoofTile> @ 0xc30cf3 -> 0x9d7390 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::cull_and_bucket_external<country_core::resources::roofs::roof_buffers::WoodenGablePlankSsbo,country_core::render::nani::cull_shaders::NaniCullAndBucketWoodenGablePlank> @ 0xc31023 -> 0x9d7390 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_namespace_source_candidate; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: country_core::shader::nani::cull_and_bucket_flowers::impl$2::bind_entry_point<rhapsody::rg::binding::RgProtocol,rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder>
// 信息 ID: 3e66818964015673cee801ca
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::shader::nani::cull_and_bucket_flowers::impl$2::bind_entry_point -> ["rhapsody::rg::binding::RgProtocol", "rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<country_core::shader::nani::cull_and_bucket_flowers::main<rhapsody::rg::binding::RgProtocol> > | rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<country_core::shader::nani::cull_and_bucket_bush::main<rhapsody::rg::binding::RgProtocol> > @ 0x1784357 -> 0xa64090 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_namespace_source_candidate; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: country_core::shader::nani::cull_and_bucket_stone_foundation_surface::impl$2::bind_entry_point<rhapsody::rg::binding::RgProtocol,rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder>
// 信息 ID: 68d0238347e10b908dc80db4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::shader::nani::cull_and_bucket_stone_foundation_surface::impl$2::bind_entry_point -> ["rhapsody::rg::binding::RgProtocol", "rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<country_core::shader::nani::cull_and_bucket_curved_halftimber::main<rhapsody::rg::binding::RgProtocol> > | rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<country_core::shader::nani::cull_and_bucket_stone_foundation_surface::main<rhapsody::rg::binding::RgProtocol> > | rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<country_core::shader::nani::cull_and_bucket_stair_rope::main<rhapsody::rg::binding::RgProtocol> > | rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<country_core::shader::nani::cull_and_bucket_smoke::main<rhapsody::rg::binding::RgProtocol> > | 其余 3 个同地址别名见共享索引 @ 0x177bd28 -> 0x93a1f0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_namespace_source_candidate; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: country_core::shader::nani::cull_and_bucket_stone_foundation_surface::impl$3::to_shader_permutation_key<rhapsody::rg::binding::RgProtocol>
// 信息 ID: c43919b13e25e8859655c6b1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::shader::nani::cull_and_bucket_stone_foundation_surface::impl$3::to_shader_permutation_key -> ["rhapsody::rg::binding::RgProtocol"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 7 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::render::nani::cull_shaders::impl$5::nani_cull_shader @ 0xa7290d -> 0x93ae90 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::nani::cull_shaders::impl$1::nani_cull_shader @ 0xa72a6d -> 0x93ae90 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::nani::cull_shaders::impl$4::nani_cull_shader @ 0xa72bcd -> 0x93ae90 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::nani::cull_shaders::impl$6::nani_cull_shader @ 0xa72d2d -> 0x93ae90 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::nani::cull_shaders::impl$3::nani_cull_shader @ 0xa72e8d -> 0x93ae90 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::nani::cull_shaders::impl$0::nani_cull_shader @ 0xa72fed -> 0x93ae90 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::nani::cull_shaders::impl$2::nani_cull_shader @ 0xa7314d -> 0x93ae90 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_namespace_source_candidate; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: country_core::shader::nani::cull_and_bucket_wooden_gable_plank::impl$2::bind_entry_point<rhapsody::rg::binding::RgProtocol,rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder>
// 信息 ID: 400266aecdbd8104eada25f0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::shader::nani::cull_and_bucket_wooden_gable_plank::impl$2::bind_entry_point -> ["rhapsody::rg::binding::RgProtocol", "rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<country_core::shader::nani::cull_and_bucket_grass2_lod0::main<rhapsody::rg::binding::RgProtocol> > | rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<country_core::shader::nani::cull_and_bucket_roof_tile::main<rhapsody::rg::binding::RgProtocol> > | rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<country_core::shader::nani::cull_and_bucket_wooden_gable_plank::main<rhapsody::rg::binding::RgProtocol> > | rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<country_core::shader::nani::cull_and_bucket_wooden_plank::main<rhapsody::rg::binding::RgProtocol> > @ 0x177ff16 -> 0x9d69d0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_namespace_source_candidate; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: country_core::shader::nani::cull_and_bucket_wooden_gable_plank::impl$3::to_shader_permutation_key<rhapsody::rg::binding::RgProtocol>
// 信息 ID: 5e7eebffe32955e1328ae4ac
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::shader::nani::cull_and_bucket_wooden_gable_plank::impl$3::to_shader_permutation_key -> ["rhapsody::rg::binding::RgProtocol"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 4 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::gpu_generate_draw_lists @ 0x9bc930 -> 0x9d7390 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::cull_and_bucket_external<country_core::resources::roofs::roof_buffers::WoodenPlankSsbo,country_core::render::nani::cull_shaders::NaniCullAndBucketWoodenPlank> @ 0xc309c3 -> 0x9d7390 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::cull_and_bucket_external<country_core::resources::roof_tile_ssbo::RoofTileSsbo,country_core::render::nani::cull_shaders::NaniCullAndBucketRoofTile> @ 0xc30cf3 -> 0x9d7390 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::cull_and_bucket_external<country_core::resources::roofs::roof_buffers::WoodenGablePlankSsbo,country_core::render::nani::cull_shaders::NaniCullAndBucketWoodenGablePlank> @ 0xc31023 -> 0x9d7390 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_namespace_source_candidate; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: country_core::shader::nani::draw_indexed_indirect_to_dispatch::impl$2::bind_entry_point<rhapsody::rg::binding::RgProtocol,rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder>
// 信息 ID: fcb110b3d95ccfc6f6c204e0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::shader::nani::draw_indexed_indirect_to_dispatch::impl$2::bind_entry_point -> ["rhapsody::rg::binding::RgProtocol", "rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<country_core::shader::nani::draw_indexed_indirect_to_dispatch::main<rhapsody::rg::binding::RgProtocol> > @ 0x178dfbd -> 0x19bf600 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_namespace_source_candidate; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: country_core::shader::nani::fill_sorted_instance_list::impl$2::bind_entry_point<rhapsody::rg::binding::RgProtocol,rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder>
// 信息 ID: bba4222b6ad25a7e2eef7bba
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::shader::nani::fill_sorted_instance_list::impl$2::bind_entry_point -> ["rhapsody::rg::binding::RgProtocol", "rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<country_core::shader::nani::fill_sorted_instance_list::main<rhapsody::rg::binding::RgProtocol> > @ 0x177e4f0 -> 0x935920 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_namespace_source_candidate; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: country_core::shader::nani::fill_unsorted_instance_indirection::impl$2::bind_entry_point<rhapsody::rg::binding::RgProtocol,rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder>
// 信息 ID: cb516bc3e38cabefc548625f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::shader::nani::fill_unsorted_instance_indirection::impl$2::bind_entry_point -> ["rhapsody::rg::binding::RgProtocol", "rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<country_core::shader::nani::fill_unsorted_instance_indirection::main<rhapsody::rg::binding::RgProtocol> > @ 0x1784964 -> 0xa7b260 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_namespace_source_candidate; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: country_core::shader::nani::fill_unsorted_instance_list::impl$2::bind_entry_point<rhapsody::rg::binding::RgProtocol,rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder>
// 信息 ID: f43891f0d5e29cab6f6e8ef4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::shader::nani::fill_unsorted_instance_list::impl$2::bind_entry_point -> ["rhapsody::rg::binding::RgProtocol", "rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<country_core::shader::nani::fill_unsorted_instance_list::main<rhapsody::rg::binding::RgProtocol> > @ 0x1790f99 -> 0xa7a8d0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_namespace_source_candidate; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: country_core::shader::nani::generate_dispatch_args::impl$2::bind_entry_point<rhapsody::rg::binding::RgProtocol,rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder>
// 信息 ID: 6b8f68f0fc4e36fe97c57c79
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::shader::nani::generate_dispatch_args::impl$2::bind_entry_point -> ["rhapsody::rg::binding::RgProtocol", "rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<country_core::shader::nani::generate_dispatch_args::main<rhapsody::rg::binding::RgProtocol> > @ 0x17787de -> 0xb4e3f0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_namespace_source_candidate; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: country_core::shader::nani::generate_draw_lists::impl$2::bind_entry_point<rhapsody::rg::binding::RgProtocol,rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder>
// 信息 ID: 1986d47f2d2d01d66ad7cd31
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::shader::nani::generate_draw_lists::impl$2::bind_entry_point -> ["rhapsody::rg::binding::RgProtocol", "rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<country_core::shader::nani::generate_draw_lists::main<rhapsody::rg::binding::RgProtocol> > @ 0x1782423 -> 0x1ab23f0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_namespace_source_candidate; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: country_core::shader::nani::group_draw_cmd_prefix_sum::impl$2::bind_entry_point<rhapsody::rg::binding::RgProtocol,rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder>
// 信息 ID: 7c62f899a91d353a06d2bd91
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::shader::nani::group_draw_cmd_prefix_sum::impl$2::bind_entry_point -> ["rhapsody::rg::binding::RgProtocol", "rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<country_core::shader::nani::group_draw_cmd_prefix_sum::main<rhapsody::rg::binding::RgProtocol> > @ 0x179158b -> 0x19bf2e0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_namespace_source_candidate; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: country_core::shader::nani::inc::indirection::impl$0::descriptors<shabriri_rt::api::EmptyProtocol>
// 信息 ID: fa66a8b4b3197ce9c1dba408
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::shader::nani::inc::indirection::impl$0::descriptors -> ["shabriri_rt::api::EmptyProtocol"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] shaderface::binding_set::BindingSetDesc::with<country_core::shader::nani::inc::indirection::NaniInstanceIndirection<shabriri_rt::api::EmptyProtocol> > @ 0x13fbf0 -> 0x58d120 (direct_call); 同地址别名未裁定。
// [静态直接引用] shaderface::binding_set::BindingSetDesc::with<country_core::shader::nani::inc::indirection::NaniInstanceIndirection<shabriri_rt::api::EmptyProtocol> > | shaderface::binding_set::BindingSetDesc::with<rhapsody_core::shader::debug::DebugInfoResources<shabriri_rt::api::EmptyProtocol> > @ 0x1a677f0 -> 0x58d120 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_namespace_source_candidate; 作者定义文件是否精确确认仍按证据状态。
// [文件映射] pdb_symbolizer_namespace_source_candidate; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: country_core::shader::nani::prefix_sum_unsorted_instances_per_atlas_entry::impl$2::bind_entry_point<rhapsody::rg::binding::RgProtocol,rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder>
// 信息 ID: 07d6cf6075421cfffecabc32
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::shader::nani::prefix_sum_unsorted_instances_per_atlas_entry::impl$2::bind_entry_point -> ["rhapsody::rg::binding::RgProtocol", "rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<country_core::shader::nani::prefix_sum_unsorted_instances_per_atlas_entry::main<rhapsody::rg::binding::RgProtocol> > @ 0x179098d -> 0x93ab80 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_namespace_source_candidate; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: country_core::shader::nani::prepare_unsorted_draw_lists::impl$2::bind_entry_point<rhapsody::rg::binding::RgProtocol,rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder>
// 信息 ID: e80b254eb27fa53302488172
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::shader::nani::prepare_unsorted_draw_lists::impl$2::bind_entry_point -> ["rhapsody::rg::binding::RgProtocol", "rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<country_core::shader::nani::prepare_unsorted_draw_lists::main<rhapsody::rg::binding::RgProtocol> > @ 0x178e59d -> 0x17fedb0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_namespace_source_candidate; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: country_core::shader::nani::subtotal_prefix_sum::impl$2::bind_entry_point<rhapsody::rg::binding::RgProtocol,rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder>
// 信息 ID: f6df6eaffa42de5acca8b775
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::shader::nani::subtotal_prefix_sum::impl$2::bind_entry_point -> ["rhapsody::rg::binding::RgProtocol", "rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<country_core::shader::nani::subtotal_prefix_sum::main<rhapsody::rg::binding::RgProtocol> > @ 0x178dbce -> 0x92e280 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_namespace_source_candidate; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: country_core::shader::nani_curved_halftimber::impl$6::bind<rhapsody::raw::binding_set::RawProtocol,rhapsody::raw::binding_set::RawBinder>
// 信息 ID: 03f04cf8ac351e0c62a78c98
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::shader::nani_curved_halftimber::impl$6::bind -> ["rhapsody::raw::binding_set::RawProtocol", "rhapsody::raw::binding_set::RawBinder"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::update_binding_set<country_core::shader::nani_curved_halftimber::MaterialData<rhapsody::raw::binding_set::RawProtocol> > @ 0x1558378 -> 0x1bf27c0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_namespace_source_candidate; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: country_core::shader::painterly::acquire_target::impl$2::bind_entry_point<rhapsody::rg::binding::RgProtocol,rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder>
// 信息 ID: 36e4b3c9f05f854d5c588e64
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::shader::painterly::acquire_target::impl$2::bind_entry_point -> ["rhapsody::rg::binding::RgProtocol", "rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<country_core::shader::painterly::acquire_target::main<rhapsody::rg::binding::RgProtocol> > @ 0x178c5db -> 0xc3cc80 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_namespace_source_candidate; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: country_core::shader::painterly::convolve_target::impl$2::bind_entry_point<rhapsody::rg::binding::RgProtocol,rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder>
// 信息 ID: 9f564bdc597153c4b46a8fcc
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::shader::painterly::convolve_target::impl$2::bind_entry_point -> ["rhapsody::rg::binding::RgProtocol", "rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<country_core::shader::painterly::convolve_target::main<rhapsody::rg::binding::RgProtocol> > @ 0x17817f3 -> 0xb4e1b0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_namespace_source_candidate; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: country_core::shader::painterly::generate_strokes::impl$2::bind_entry_point<rhapsody::rg::binding::RgProtocol,rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder>
// 信息 ID: beefc98c163fc943751ba9dc
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::shader::painterly::generate_strokes::impl$2::bind_entry_point -> ["rhapsody::rg::binding::RgProtocol", "rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<country_core::shader::painterly::generate_strokes::main<rhapsody::rg::binding::RgProtocol> > @ 0x1788a55 -> 0xb77e90 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_namespace_source_candidate; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: country_core::shader::path_pebbles::impl$2::bind_entry_point<rhapsody::rg::binding::RgProtocol,rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder>
// 信息 ID: bf0d9dd59e5a69343f65aa94
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::shader::path_pebbles::impl$2::bind_entry_point -> ["rhapsody::rg::binding::RgProtocol", "rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<country_core::shader::path_pebbles::main<rhapsody::rg::binding::RgProtocol> > @ 0x1793353 -> 0x9d8670 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_namespace_source_candidate; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: country_core::shader::paths::combine_path_exclusions::impl$2::bind_entry_point<rhapsody::rg::binding::RgProtocol,rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder>
// 信息 ID: b877eb4c3d4f99ded65aee3f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::shader::paths::combine_path_exclusions::impl$2::bind_entry_point -> ["rhapsody::rg::binding::RgProtocol", "rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<country_core::shader::paths::combine_path_exclusions::main<rhapsody::rg::binding::RgProtocol> > @ 0x178adbf -> 0xb85920 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_namespace_source_candidate; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: country_core::shader::paths::directional_contract::impl$2::bind_entry_point<rhapsody::rg::binding::RgProtocol,rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder>
// 信息 ID: 2713aff750d5c12bcb42cd9c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::shader::paths::directional_contract::impl$2::bind_entry_point -> ["rhapsody::rg::binding::RgProtocol", "rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<country_core::shader::paths::directional_contract::main<rhapsody::rg::binding::RgProtocol> > @ 0x1792773 -> 0xc18070 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_namespace_source_candidate; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: country_core::shader::paths::extend_path_mask_margin::impl$2::bind_entry_point<rhapsody::rg::binding::RgProtocol,rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder>
// 信息 ID: 9be3c3287a9c3533aef4daea
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::shader::paths::extend_path_mask_margin::impl$2::bind_entry_point -> ["rhapsody::rg::binding::RgProtocol", "rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<country_core::shader::paths::extend_path_mask_margin::main<rhapsody::rg::binding::RgProtocol> > @ 0x177ead1 -> 0xb4e890 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_namespace_source_candidate; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: country_core::shader::paths::inc::path_mask_sampler::impl$0::descriptors<shabriri_rt::api::EmptyProtocol>
// 信息 ID: 1d4d1591bbc19e0244161085
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::shader::paths::inc::path_mask_sampler::impl$0::descriptors -> ["shabriri_rt::api::EmptyProtocol"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] shaderface::binding_set::BindingSetDesc::with<country_core::shader::inc::world::GardenMask<shabriri_rt::api::EmptyProtocol> > | shaderface::binding_set::BindingSetDesc::with<country_core::shader::paths::inc::path_mask_sampler::PathExclusionMask<shabriri_rt::api::EmptyProtocol> > @ 0x1a68770 -> 0x902620 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_namespace_source_candidate; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: country_core::shader::paths::path_mask_visual::impl$2::bind_entry_point<rhapsody::rg::binding::RgProtocol,rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder>
// 信息 ID: dd6f58c3d9c54724c73be10e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::shader::paths::path_mask_visual::impl$2::bind_entry_point -> ["rhapsody::rg::binding::RgProtocol", "rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<country_core::shader::paths::path_mask_visual::main<rhapsody::rg::binding::RgProtocol> > @ 0x1785ee5 -> 0x1806e70 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_namespace_source_candidate; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: country_core::shader::prefix_scan::inclusive_prefix_scan_merge::impl$2::bind_entry_point<rhapsody::rg::binding::RgProtocol,rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder>
// 信息 ID: 0f97566d1cd7f7afa00ff101
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::shader::prefix_scan::inclusive_prefix_scan_merge::impl$2::bind_entry_point -> ["rhapsody::rg::binding::RgProtocol", "rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<country_core::shader::light_binning::cell_prefix_merge::main<rhapsody::rg::binding::RgProtocol> > | rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<country_core::shader::prefix_scan::inclusive_prefix_scan_merge::main<rhapsody::rg::binding::RgProtocol> > @ 0x1779360 -> 0x9d67d0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_namespace_source_candidate; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: country_core::shader::prefix_scan::inclusive_prefix_scan_subtotals::impl$2::bind_entry_point<rhapsody::rg::binding::RgProtocol,rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder>
// 信息 ID: 406d69ae8151e16c0f7910d1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::shader::prefix_scan::inclusive_prefix_scan_subtotals::impl$2::bind_entry_point -> ["rhapsody::rg::binding::RgProtocol", "rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<country_core::shader::prefix_scan::inclusive_prefix_scan_subtotals::main<rhapsody::rg::binding::RgProtocol> > @ 0x177a90e -> 0x19bf830 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_namespace_source_candidate; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: country_core::shader::primary_rt_debug::impl$2::bind_entry_point<rhapsody::rg::binding::RgProtocol,rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder>
// 信息 ID: a5246b4bdee6cf13b5a17ad7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::shader::primary_rt_debug::impl$2::bind_entry_point -> ["rhapsody::rg::binding::RgProtocol", "rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<country_core::shader::primary_rt_debug::main<rhapsody::rg::binding::RgProtocol> > @ 0x1798f82 -> 0xacd010 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_namespace_source_candidate; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: country_core::shader::rasterize_buildings::impl$2::bind_entry_point<rhapsody::rg::binding::RgProtocol,rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder>
// 信息 ID: fbb283617a10725c9de82785
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::shader::rasterize_buildings::impl$2::bind_entry_point -> ["rhapsody::rg::binding::RgProtocol", "rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<country_core::shader::rasterize_buildings::main<rhapsody::rg::binding::RgProtocol> > @ 0x177cf32 -> 0xa18110 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_namespace_source_candidate; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: country_core::shader::rasterize_buildings_animate::impl$2::bind_entry_point<rhapsody::rg::binding::RgProtocol,rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder>
// 信息 ID: 5b30a52dec6a2ca226786b86
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::shader::rasterize_buildings_animate::impl$2::bind_entry_point -> ["rhapsody::rg::binding::RgProtocol", "rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<country_core::shader::terrain_derivative::main<rhapsody::rg::binding::RgProtocol> > | rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<country_core::shader::lut::water_noise::main<rhapsody::rg::binding::RgProtocol> > | rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<country_core::shader::terrain_lighting_texture::main<rhapsody::rg::binding::RgProtocol> > | rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<country_core::shader::rasterize_buildings_animate::main<rhapsody::rg::binding::RgProtocol> > @ 0x177a34d -> 0x903ee0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_namespace_source_candidate; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: country_core::shader::rdi::temporal::impl$2::bind_entry_point<rhapsody::rg::binding::RgProtocol,rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder>
// 信息 ID: 07cf3fc20e5623c14d065ec9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::shader::rdi::temporal::impl$2::bind_entry_point -> ["rhapsody::rg::binding::RgProtocol", "rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<country_core::shader::rdi::temporal::main<rhapsody::rg::binding::RgProtocol> > @ 0x1781e08 -> 0xc470f0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_namespace_source_candidate; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: country_core::shader::read_mouse_depth_value::impl$2::bind_entry_point<rhapsody::rg::binding::RgProtocol,rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder>
// 信息 ID: 0798a92d886f7a4331c21cf8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::shader::read_mouse_depth_value::impl$2::bind_entry_point -> ["rhapsody::rg::binding::RgProtocol", "rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<country_core::shader::read_mouse_depth_value::main<rhapsody::rg::binding::RgProtocol> > @ 0x17903b7 -> 0xad4640 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_namespace_source_candidate; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: country_core::shader::read_terrain_stroke::impl$2::bind_entry_point<rhapsody::rg::binding::RgProtocol,rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder>
// 信息 ID: fd8d3c4f7a7c0092c6bfc165
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::shader::read_terrain_stroke::impl$2::bind_entry_point -> ["rhapsody::rg::binding::RgProtocol", "rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<country_core::shader::read_terrain_stroke::main<rhapsody::rg::binding::RgProtocol> > @ 0x1795316 -> 0xad43b0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_namespace_source_candidate; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: country_core::shader::rocky_terrain::displace_rocky_terrain::impl$2::bind_entry_point<rhapsody::rg::binding::RgProtocol,rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder>
// 信息 ID: 4829b03cdaf5b67a0cad38e8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::shader::rocky_terrain::displace_rocky_terrain::impl$2::bind_entry_point -> ["rhapsody::rg::binding::RgProtocol", "rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<country_core::shader::rocky_terrain::displace_rocky_terrain::main<rhapsody::rg::binding::RgProtocol> > @ 0x177b88e -> 0x1ab6c20 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_namespace_source_candidate; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: country_core::shader::rocky_terrain::inc::mesh_data::impl$0::descriptors<shabriri_rt::api::EmptyProtocol>
// 信息 ID: 236ce06cae72248af7f200e8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::shader::rocky_terrain::inc::mesh_data::impl$0::descriptors -> ["shabriri_rt::api::EmptyProtocol"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] shaderface::binding_set::BindingSetDesc::with<country_core::shader::rocky_terrain::inc::mesh_data::RockyTerrainMesh<shabriri_rt::api::EmptyProtocol> > @ 0x1a69eb0 -> 0xc496d0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_namespace_source_candidate; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: country_core::shader::rocky_terrain::stairs::stairs::impl$2::bind_entry_point<rhapsody::rg::binding::RgProtocol,rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder>
// 信息 ID: cc6e6eb3892c832e9e9be0cf
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::shader::rocky_terrain::stairs::stairs::impl$2::bind_entry_point -> ["rhapsody::rg::binding::RgProtocol", "rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<country_core::shader::rocky_terrain::stairs::stairs::main<rhapsody::rg::binding::RgProtocol> > @ 0x179712a -> 0x19bf080 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_namespace_source_candidate; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: country_core::shader::rocky_terrain::stairs::stairs_counter::impl$2::bind_entry_point<rhapsody::rg::binding::RgProtocol,rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder>
// 信息 ID: 55254b8d6a0c752a877d2355
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::shader::rocky_terrain::stairs::stairs_counter::impl$2::bind_entry_point -> ["rhapsody::rg::binding::RgProtocol", "rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<country_core::shader::rocky_terrain::stairs::stairs_counter::main<rhapsody::rg::binding::RgProtocol> > | rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<country_core::shader::light_binning::bin_lights_to_cells_0::main<rhapsody::rg::binding::RgProtocol> > @ 0x177b2dd -> 0x17fefd0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_namespace_source_candidate; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: country_core::shader::roof::inc::dormer_hole::impl$3::descriptors<shabriri_rt::api::EmptyProtocol>
// 信息 ID: 72329881875b0aa8185744a3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::shader::roof::inc::dormer_hole::impl$3::descriptors -> ["shabriri_rt::api::EmptyProtocol"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] shaderface::binding_set::BindingSetDesc::with<country_core::shader::roof::inc::dormer_hole::DormerHoles<shabriri_rt::api::EmptyProtocol> > @ 0x13f620 -> 0x39fb80 (direct_call); 同地址别名未裁定。
// [静态直接引用] shaderface::binding_set::BindingSetDesc::with<country_core::shader::roof::inc::dormer_hole::DormerHoles<shabriri_rt::api::EmptyProtocol> > @ 0x1a68d40 -> 0x39fb80 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_namespace_source_candidate; 作者定义文件是否精确确认仍按证据状态。
// [文件映射] pdb_symbolizer_namespace_source_candidate; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: country_core::shader::roof::inc::stone_floor::impl$0::descriptors<shabriri_rt::api::EmptyProtocol>
// 信息 ID: b86975767395341d00e4b3d9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::shader::roof::inc::stone_floor::impl$0::descriptors -> ["shabriri_rt::api::EmptyProtocol"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] shaderface::binding_set::BindingSetDesc::with<country_core::shader::roof::inc::stone_floor::StoneFloorTextures<shabriri_rt::api::EmptyProtocol> > @ 0x1a6a290 -> 0xc479e0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_namespace_source_candidate; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: country_core::shader::roof::inc::stone_floor::impl$2::bind<rhapsody::raw::binding_set::RawProtocol,rhapsody::raw::binding_set::RawBinder>
// 信息 ID: cda2bf5f195eaa1eff901606
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::shader::roof::inc::stone_floor::impl$2::bind -> ["rhapsody::raw::binding_set::RawProtocol", "rhapsody::raw::binding_set::RawBinder"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::update_binding_set<country_core::shader::roof::inc::stone_floor::StoneFloorTextures<rhapsody::raw::binding_set::RawProtocol> > @ 0x15c810f -> 0xc494f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::update_binding_set<country_core::shader::roof::inc::stone_floor::StoneFloorTextures<rhapsody::raw::binding_set::RawProtocol> > @ 0x15c8356 -> 0xc494f0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_namespace_source_candidate; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: country_core::shader::roof::inc::wooden_floor::impl$0::descriptors<shabriri_rt::api::EmptyProtocol>
// 信息 ID: eb3e043a0d67b82fc4955b9a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::shader::roof::inc::wooden_floor::impl$0::descriptors -> ["shabriri_rt::api::EmptyProtocol"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] shaderface::binding_set::BindingSetDesc::with<country_core::shader::roof::inc::wooden_floor::WoodenFloorTextures<shabriri_rt::api::EmptyProtocol> > @ 0x1a69ad0 -> 0xb762c0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_namespace_source_candidate; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: country_core::shader::roof::inc::wooden_floor::impl$2::bind<rhapsody::raw::binding_set::RawProtocol,rhapsody::raw::binding_set::RawBinder>
// 信息 ID: 8721299ea0d47096137973d6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::shader::roof::inc::wooden_floor::impl$2::bind -> ["rhapsody::raw::binding_set::RawProtocol", "rhapsody::raw::binding_set::RawBinder"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::update_binding_set<country_core::shader::roof::inc::wooden_floor::WoodenFloorTextures<rhapsody::raw::binding_set::RawProtocol> > @ 0x156256f -> 0xb78280 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::update_binding_set<country_core::shader::roof::inc::wooden_floor::WoodenFloorTextures<rhapsody::raw::binding_set::RawProtocol> > @ 0x15627b6 -> 0xb78280 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_namespace_source_candidate; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: country_core::shader::roof_tiles_gravity::impl$2::bind_entry_point<rhapsody::rg::binding::RgProtocol,rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder>
// 信息 ID: 0f9ba929910852c4303677be
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::shader::roof_tiles_gravity::impl$2::bind_entry_point -> ["rhapsody::rg::binding::RgProtocol", "rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<country_core::shader::roof_tiles_gravity::main<rhapsody::rg::binding::RgProtocol> > @ 0x178cba6 -> 0xbb2ec0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_namespace_source_candidate; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: country_core::shader::screenshot::capture_screenshot_buffer::impl$6::bind_entry_point<rhapsody::rg::binding::RgProtocol,rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder>
// 信息 ID: fb1521f9b1ea2dca4749b85e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::shader::screenshot::capture_screenshot_buffer::impl$6::bind_entry_point -> ["rhapsody::rg::binding::RgProtocol", "rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<country_core::shader::screenshot::capture_screenshot_buffer::sample4<rhapsody::rg::binding::RgProtocol> > @ 0x477d03 -> 0x1ef5b0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_namespace_source_candidate; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: country_core::shader::screenshot::capture_screenshot_texture::impl$2::bind_entry_point<rhapsody::rg::binding::RgProtocol,rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder>
// 信息 ID: e2f4beac3a0d3b3b0134e332
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::shader::screenshot::capture_screenshot_texture::impl$2::bind_entry_point -> ["rhapsody::rg::binding::RgProtocol", "rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<country_core::shader::screenshot::capture_screenshot_texture::sample4<rhapsody::rg::binding::RgProtocol> > | rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<country_core::shader::screenshot::capture_screenshot_texture::sample1<rhapsody::rg::binding::RgProtocol> > @ 0x4788cf -> 0x78d70 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_namespace_source_candidate; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: country_core::shader::screenshot::tgsr_capture_hires::impl$2::bind_entry_point<rhapsody::rg::binding::RgProtocol,rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder>
// 信息 ID: 27096932ea8bc19ba13b0d3e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::shader::screenshot::tgsr_capture_hires::impl$2::bind_entry_point -> ["rhapsody::rg::binding::RgProtocol", "rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<country_core::shader::screenshot::tgsr_capture_hires::main<rhapsody::rg::binding::RgProtocol> > @ 0x4782dc -> 0x1032c0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_namespace_source_candidate; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: country_core::shader::screenshot::tgsr_capture_native::impl$2::bind_entry_point<rhapsody::rg::binding::RgProtocol,rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder>
// 信息 ID: 2eddd472e04e4e861d76c012
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::shader::screenshot::tgsr_capture_native::impl$2::bind_entry_point -> ["rhapsody::rg::binding::RgProtocol", "rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<country_core::shader::screenshot::tgsr_capture_native::main<rhapsody::rg::binding::RgProtocol> > @ 0x479a7b -> 0x5dc490 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_namespace_source_candidate; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: country_core::shader::screenshot::tgsr_capture_ref::impl$2::bind_entry_point<rhapsody::rg::binding::RgProtocol,rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder>
// 信息 ID: 86baff5f3dc1799725ec5f2a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::shader::screenshot::tgsr_capture_ref::impl$2::bind_entry_point -> ["rhapsody::rg::binding::RgProtocol", "rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<country_core::shader::screenshot::tgsr_capture_ref::main<rhapsody::rg::binding::RgProtocol> > @ 0x47948b -> 0x50dc40 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_namespace_source_candidate; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: country_core::shader::sheep::impl$9::bind<rhapsody::raw::binding_set::RawProtocol,rhapsody::raw::binding_set::RawBinder>
// 信息 ID: f03d9bbd74daaf84bac310ca
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::shader::sheep::impl$9::bind -> ["rhapsody::raw::binding_set::RawProtocol", "rhapsody::raw::binding_set::RawBinder"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::update_binding_set<country_core::shader::sheep::MaterialData<rhapsody::raw::binding_set::RawProtocol> > @ 0x15a6428 -> 0xa048c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::update_binding_set<country_core::shader::sheep::MaterialData<rhapsody::raw::binding_set::RawProtocol> > @ 0x15a6685 -> 0xa048c0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_namespace_source_candidate; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: country_core::shader::terrain_editing::calc_highlight_hill_mask::impl$2::bind_entry_point<rhapsody::rg::binding::RgProtocol,rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder>
// 信息 ID: d0e6c9cc31ae5b211c8db576
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::shader::terrain_editing::calc_highlight_hill_mask::impl$2::bind_entry_point -> ["rhapsody::rg::binding::RgProtocol", "rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<country_core::shader::terrain_editing::calc_highlight_hill_mask::main<rhapsody::rg::binding::RgProtocol> > @ 0x1791b82 -> 0xc84d10 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_namespace_source_candidate; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: country_core::shader::terrain_editing::rasterize_terrain_stroke::impl$2::bind_entry_point<rhapsody::rg::binding::RgProtocol,rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder>
// 信息 ID: 490d8702a4190063919ca2d0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::shader::terrain_editing::rasterize_terrain_stroke::impl$2::bind_entry_point -> ["rhapsody::rg::binding::RgProtocol", "rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<country_core::shader::terrain_editing::rasterize_terrain_stroke::main<rhapsody::rg::binding::RgProtocol> > @ 0x178a231 -> 0xaf3130 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_namespace_source_candidate; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: country_core::shader::terrain_editing::read_terrain_height::impl$2::bind_entry_point<rhapsody::rg::binding::RgProtocol,rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder>
// 信息 ID: 139d012707c798199239081c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::shader::terrain_editing::read_terrain_height::impl$2::bind_entry_point -> ["rhapsody::rg::binding::RgProtocol", "rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<country_core::shader::shadow_mask_mean::main<rhapsody::rg::binding::RgProtocol> > | rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<country_core::shader::light_binning::cell_prefix_sum::main<rhapsody::rg::binding::RgProtocol> > | rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<country_core::shader::terrain_editing::read_terrain_height::main<rhapsody::rg::binding::RgProtocol> > | rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<country_core::shader::water::read_water_mask::main<rhapsody::rg::binding::RgProtocol> > @ 0x1784ee0 -> 0x9ddd00 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_namespace_source_candidate; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: country_core::shader::text::impl$2::bind_entry_point<rhapsody::rg::binding::RgProtocol,rhapsody::rg::binding::RgEntryBinder>
// 信息 ID: 68a3ad3820e2793b15fdcef6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::shader::text::impl$2::bind_entry_point -> ["rhapsody::rg::binding::RgProtocol", "rhapsody::rg::binding::RgEntryBinder"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::node_builder::draw_stream::RasterStreamRecorder::shader<country_core::shader::text::vs_main<rhapsody::rg::binding::RgProtocol> > @ 0x198fa8f -> 0xba2a10 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_namespace_source_candidate; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: country_core::shader::texture_export::export_2d::impl$2::bind_entry_point<rhapsody::rg::binding::RgProtocol,rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder>
// 信息 ID: 640b6b4c94716332db1a448f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::shader::texture_export::export_2d::impl$2::bind_entry_point -> ["rhapsody::rg::binding::RgProtocol", "rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<country_core::shader::texture_export::export_2d::main<rhapsody::rg::binding::RgProtocol> > | rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<country_core::shader::texture_export::export_2d_array::main<rhapsody::rg::binding::RgProtocol> > @ 0x1788413 -> 0x935ec0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_namespace_source_candidate; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: country_core::shader::texture_export::export_2d_array::impl$2::bind_entry_point<rhapsody::rg::binding::RgProtocol,rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder>
// 信息 ID: 14de96dbef4b0467e40cb1e1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::shader::texture_export::export_2d_array::impl$2::bind_entry_point -> ["rhapsody::rg::binding::RgProtocol", "rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<country_core::shader::texture_export::export_2d::main<rhapsody::rg::binding::RgProtocol> > | rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<country_core::shader::texture_export::export_2d_array::main<rhapsody::rg::binding::RgProtocol> > @ 0x1788413 -> 0x935ec0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_namespace_source_candidate; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: country_core::shader::timed_terrain_gizmo::impl$6::bind<rhapsody::raw::binding_set::RawProtocol,rhapsody::raw::binding_set::RawBinder>
// 信息 ID: 422117960a66e14af4fdea09
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::shader::timed_terrain_gizmo::impl$6::bind -> ["rhapsody::raw::binding_set::RawProtocol", "rhapsody::raw::binding_set::RawBinder"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::update_binding_set<country_core::shader::timed_terrain_gizmo::MaterialData> @ 0x15cc899 -> 0xa04a70 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::update_binding_set<country_core::shader::timed_terrain_gizmo::MaterialData> @ 0x15ccaca -> 0xa04a70 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_namespace_source_candidate; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: country_core::shader::upload_cpu_raster::impl$2::bind_entry_point<rhapsody::rg::binding::RgProtocol,rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder>
// 信息 ID: 7157e633678d5632f3103804
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::shader::upload_cpu_raster::impl$2::bind_entry_point -> ["rhapsody::rg::binding::RgProtocol", "rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<country_core::shader::upload_cpu_raster::main<rhapsody::rg::binding::RgProtocol> > @ 0x478ead -> 0x5da840 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_namespace_source_candidate; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: country_core::shader::upload_cpu_raster::impl$3::to_shader_permutation_key<rhapsody::rg::binding::RgProtocol>
// 信息 ID: 029e09a7c00c8821bedf4985
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::shader::upload_cpu_raster::impl$3::to_shader_permutation_key -> ["rhapsody::rg::binding::RgProtocol"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 7 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::render_loop::compute_passes::upload_world_raster::upload_raster_staging_to_texture<country_core::resources::world_raster_defs::GardenRaster> @ 0x69f764 -> 0x5daa70 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::upload_world_raster::upload_raster_staging_to_texture<country_core::resources::world_raster_defs::WaterRaster> @ 0x69fcf4 -> 0x5daa70 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::upload_world_raster::upload_raster_staging_to_texture<country_core::resources::world_raster_defs::PathRaster> @ 0x6a0284 -> 0x5daa70 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::upload_world_raster::upload_raster_staging_to_texture<country_core::resources::world_raster_defs::Cobblestone1Raster> @ 0x6a0814 -> 0x5daa70 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::upload_world_raster::upload_raster_staging_to_texture<country_core::resources::world_raster_defs::Cobblestone3Raster> @ 0x6a0da4 -> 0x5daa70 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::upload_world_raster::upload_raster_staging_to_texture<country_core::resources::world_raster_defs::PlantExemplarRaster> @ 0x6a1334 -> 0x5daa70 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::upload_world_raster::upload_raster_staging_to_texture<country_core::resources::world_raster_defs::Cobblestone2Raster> @ 0x6a18c4 -> 0x5daa70 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_namespace_source_candidate; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: country_core::shader::upload_plant_sample::impl$2::bind_entry_point<rhapsody::rg::binding::RgProtocol,rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder>
// 信息 ID: 90c8c0fe13680aa652bbdf5d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::shader::upload_plant_sample::impl$2::bind_entry_point -> ["rhapsody::rg::binding::RgProtocol", "rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<country_core::shader::upload_plant_sample::main<rhapsody::rg::binding::RgProtocol> > @ 0x17983ad -> 0x9ccba0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_namespace_source_candidate; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: country_core::shader::vermeer::acquire_target::impl$2::bind_entry_point<rhapsody::rg::binding::RgProtocol,rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder>
// 信息 ID: 16759b88dcd534a3dbb7b02d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::shader::vermeer::acquire_target::impl$2::bind_entry_point -> ["rhapsody::rg::binding::RgProtocol", "rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<country_core::shader::vermeer::acquire_target::main<rhapsody::rg::binding::RgProtocol> > @ 0x17958f1 -> 0xc49050 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_namespace_source_candidate; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: country_core::shader::vermeer::combine::impl$2::bind_entry_point<rhapsody::rg::binding::RgProtocol,rhapsody::rg::binding::RgEntryBinder>
// 信息 ID: ac84a42d76b46419b43c6919
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::shader::vermeer::combine::impl$2::bind_entry_point -> ["rhapsody::rg::binding::RgProtocol", "rhapsody::rg::binding::RgEntryBinder"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::node_builder::draw_stream::RasterStreamRecorder::shader<country_core::shader::vermeer::combine::vs_main<rhapsody::rg::binding::RgProtocol> > @ 0x1990f54 -> 0xb75940 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_namespace_source_candidate; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: country_core::shader::vermeer::generate_strokes::impl$2::bind_entry_point<rhapsody::rg::binding::RgProtocol,rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder>
// 信息 ID: e27bf15eb964b3ec5101f552
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::shader::vermeer::generate_strokes::impl$2::bind_entry_point -> ["rhapsody::rg::binding::RgProtocol", "rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<country_core::shader::vermeer::generate_strokes::main<rhapsody::rg::binding::RgProtocol> > @ 0x1789637 -> 0xb77980 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_namespace_source_candidate; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: country_core::shader::vermeer::generate_strokes::impl$3::to_shader_permutation_key<rhapsody::rg::binding::RgProtocol>
// 信息 ID: 60146bb3913567e5c0dd7253
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::shader::vermeer::generate_strokes::impl$3::to_shader_permutation_key -> ["rhapsody::rg::binding::RgProtocol"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::render_loop::vermeer::vermeer_render_from_target @ 0x941bc9 -> 0xb7b080 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_namespace_source_candidate; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: country_core::shader::wall::fill_instance_indirection::impl$2::bind_entry_point<rhapsody::rg::binding::RgProtocol,rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder>
// 信息 ID: 8637acc0568df21569b29078
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::shader::wall::fill_instance_indirection::impl$2::bind_entry_point -> ["rhapsody::rg::binding::RgProtocol", "rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<country_core::shader::wall::fill_instance_indirection::main<rhapsody::rg::binding::RgProtocol> > @ 0x177c334 -> 0x9dd2e0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_namespace_source_candidate; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: country_core::shader::wall::generate_draw_lists2::impl$2::bind_entry_point<rhapsody::rg::binding::RgProtocol,rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder>
// 信息 ID: 271d0f355339f3e214b84ae2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::shader::wall::generate_draw_lists2::impl$2::bind_entry_point -> ["rhapsody::rg::binding::RgProtocol", "rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<country_core::shader::wall::generate_draw_lists2::cs_pass2<rhapsody::rg::binding::RgProtocol> > @ 0x1787834 -> 0x9d9670 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_namespace_source_candidate; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: country_core::shader::wall::generate_draw_lists::impl$2::bind_entry_point<rhapsody::rg::binding::RgProtocol,rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder>
// 信息 ID: f768f5145aba8d60924a1c3b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::shader::wall::generate_draw_lists::impl$2::bind_entry_point -> ["rhapsody::rg::binding::RgProtocol", "rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<country_core::shader::wall::generate_draw_lists::cs_pass1<rhapsody::rg::binding::RgProtocol> > @ 0x1778242 -> 0x9d8f90 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_namespace_source_candidate; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: country_core::shader::wall::generate_draw_lists_shadow::impl$2::bind_entry_point<rhapsody::rg::binding::RgProtocol,rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder>
// 信息 ID: 11cd89e077f5304b6bc7f6b6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::shader::wall::generate_draw_lists_shadow::impl$2::bind_entry_point -> ["rhapsody::rg::binding::RgProtocol", "rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<country_core::shader::wall::generate_draw_lists_shadow::main<rhapsody::rg::binding::RgProtocol> > @ 0x1799b72 -> 0x184d2b0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_namespace_source_candidate; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: country_core::shader::wall::prefix_brick_chunk_subtotals::impl$2::bind_entry_point<rhapsody::rg::binding::RgProtocol,rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder>
// 信息 ID: d25b30c5611ffadf1d7ba837
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::shader::wall::prefix_brick_chunk_subtotals::impl$2::bind_entry_point -> ["rhapsody::rg::binding::RgProtocol", "rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<country_core::shader::wall::prefix_brick_chunk_subtotals::main<rhapsody::rg::binding::RgProtocol> > @ 0x178d60d -> 0x9dd790 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_namespace_source_candidate; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: country_core::shader::wall::prefix_brick_chunks::impl$2::bind_entry_point<rhapsody::rg::binding::RgProtocol,rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder>
// 信息 ID: 5a70a6d986ec2a55fa07e7d1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::shader::wall::prefix_brick_chunks::impl$2::bind_entry_point -> ["rhapsody::rg::binding::RgProtocol", "rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<country_core::shader::wall::prefix_brick_chunks::main<rhapsody::rg::binding::RgProtocol> > @ 0x1789c1c -> 0x1848810 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_namespace_source_candidate; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: country_core::shader::wall::reset_pass2_inst_counts::impl$2::bind_entry_point<rhapsody::rg::binding::RgProtocol,rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder>
// 信息 ID: ac4c29bdd04d4b9a20381c5b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::shader::wall::reset_pass2_inst_counts::impl$2::bind_entry_point -> ["rhapsody::rg::binding::RgProtocol", "rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<country_core::shader::wall::reset_pass2_inst_counts::main<rhapsody::rg::binding::RgProtocol> > @ 0x177ad03 -> 0xc7d420 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_namespace_source_candidate; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: country_core::shader::water::deferred_rt::impl$2::bind_entry_point<rhapsody::rg::binding::RgProtocol,rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder>
// 信息 ID: 6694dd966d451e68122cffe9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::shader::water::deferred_rt::impl$2::bind_entry_point -> ["rhapsody::rg::binding::RgProtocol", "rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<country_core::shader::water::deferred_rt::main<rhapsody::rg::binding::RgProtocol> > @ 0x178057a -> 0x9cc410 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_namespace_source_candidate; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: country_core::shader::water::ice_deferred_rt::impl$2::bind_entry_point<rhapsody::rg::binding::RgProtocol,rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder>
// 信息 ID: d5457b50c74fb671a069f249
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::shader::water::ice_deferred_rt::impl$2::bind_entry_point -> ["rhapsody::rg::binding::RgProtocol", "rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<country_core::shader::water::ice_deferred_rt::main<rhapsody::rg::binding::RgProtocol> > @ 0x178ebd8 -> 0x1ab8ef0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_namespace_source_candidate; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: country_core::shader::water::ice_refl_march::impl$2::bind_entry_point<rhapsody::rg::binding::RgProtocol,rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder>
// 信息 ID: dad77afcdee0d1656e51244a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::shader::water::ice_refl_march::impl$2::bind_entry_point -> ["rhapsody::rg::binding::RgProtocol", "rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<country_core::shader::water::ice_refl_march::main<rhapsody::rg::binding::RgProtocol> > @ 0x17864fb -> 0x19e71b0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_namespace_source_candidate; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: country_core::shader::water::ice_refl_resolve::impl$2::bind_entry_point<rhapsody::rg::binding::RgProtocol,rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder>
// 信息 ID: 744781284443237c858b5b60
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::shader::water::ice_refl_resolve::impl$2::bind_entry_point -> ["rhapsody::rg::binding::RgProtocol", "rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<country_core::shader::water::ice_refl_resolve::main<rhapsody::rg::binding::RgProtocol> > @ 0x17871a2 -> 0xad4b70 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_namespace_source_candidate; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: country_core::shader::water::ice_refr_resolve::impl$2::bind_entry_point<rhapsody::rg::binding::RgProtocol,rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder>
// 信息 ID: 46f3bb92130925abf4f7cd46
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::shader::water::ice_refr_resolve::impl$2::bind_entry_point -> ["rhapsody::rg::binding::RgProtocol", "rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<country_core::shader::water::ice_refr_resolve::main<rhapsody::rg::binding::RgProtocol> > @ 0x1795edf -> 0x1bf28b0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_namespace_source_candidate; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: country_core::shader::water::inc::water_mask_sampler::impl$2::bind<rhapsody::rg::binding::RgProtocol,rhapsody::rg::node_builder::compute_pass_builder::RgComputeBinder>
// 信息 ID: f65320582a5bac5f06387d1b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::shader::water::inc::water_mask_sampler::impl$2::bind -> ["rhapsody::rg::binding::RgProtocol", "rhapsody::rg::node_builder::compute_pass_builder::RgComputeBinder"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::shader::paths::path_mask_visual::impl$2::bind_entry_point<rhapsody::rg::binding::RgProtocol,rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder> @ 0x1806faa -> 0xaef6a0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_namespace_source_candidate; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: country_core::shader::water::inc::waves::impl$3::descriptors<shabriri_rt::api::EmptyProtocol>
// 信息 ID: 5440a2e360498ae76f10cbb9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::shader::water::inc::waves::impl$3::descriptors -> ["shabriri_rt::api::EmptyProtocol"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] shaderface::binding_set::BindingSetDesc::with<country_core::shader::water::inc::waves::WaterWaves<shabriri_rt::api::EmptyProtocol> > @ 0x1a67fb0 -> 0xb75790 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_namespace_source_candidate; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: country_core::shader::water::spawn_lilypads::impl$2::bind_entry_point<rhapsody::rg::binding::RgProtocol,rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder>
// 信息 ID: 17ad4f02c53fc071329e351b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::shader::water::spawn_lilypads::impl$2::bind_entry_point -> ["rhapsody::rg::binding::RgProtocol", "rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<country_core::shader::water::spawn_lilypads::main<rhapsody::rg::binding::RgProtocol> > @ 0x17964da -> 0xc7cda0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_namespace_source_candidate; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: country_core::shader::water_plane::impl$2::bind_entry_point<rhapsody::rg::binding::RgProtocol,rhapsody::rg::binding::RgEntryBinder>
// 信息 ID: 99dd2c25947e45281ed9d4f4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::shader::water_plane::impl$2::bind_entry_point -> ["rhapsody::rg::binding::RgProtocol", "rhapsody::rg::binding::RgEntryBinder"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::node_builder::draw_stream::RasterStreamRecorder::prebuilt_pipeline_with_compatible_args<country_core::shader::water_plane::vs_main<rhapsody::rg::binding::RgProtocol> > @ 0x197f02b -> 0x19b5e30 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_namespace_source_candidate; 作者定义文件是否精确确认仍按证据状态。

// === PDB ENRICHMENT V1 END ===
