// Generated from PDB metadata by skeleton/_tools/enrich_skeleton.py.
// Version: C
// Skeleton path: _unmapped/module-00136.rs
// Original source: 
// 函数体、未记录参数名和布局 = unknown。

// === PDB STRUCTURES V1 BEGIN ===
// 数据结构事实按版本保留；PDB 定义 / 仅名称关系 / 原指令观察分别标注。
// 完整结构及原始类型记录: module-00136.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: module-00136.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: module-00136.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::mode_manager::RequestAppModeChange>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::ui_clutter::ui_focus_decoclutter::RequestDecoClutterPicker>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::systems::ui_clutter::ui_focus_decoclutter::LastClonedClutter>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::ui_clutter::ui_focus_decoclutter::RequestMoveClutter>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::ui_clutter::ui_focus_decoclutter::RequestRotateDecorator>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::decorator_audio::PlayDecoratorSoundCmd> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::systems::ui_clutter::ui_focus_decoclutter::DecoClutterPicker>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >", "bevy_ecs::message::message_writer::MessageWriter<country_core::systems::mode_manager::RequestAppModeChange>", "bevy_ecs::message::message_writer::MessageWriter<country_core::systems::ui_clutter::ui_focus_decoclutter::RequestDecoClutterPicker>", "bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>", "bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>", "bevy_ecs::change_detection::ResMut<country_core::systems::ui_clutter::ui_focus_decoclutter::LastClonedClutter>", "bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>", "tuple$<bevy_ecs::change_detection::Res<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::ui_clutter::ui_focus_decoclutter::RequestMoveClutter>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::ui_clutter::ui_focus_decoclutter::RequestRotateDecorator>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::decorator_audio::PlayDecoratorSoundCmd> >", "bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>", "bevy_ecs::change_detection::ResMut<country_core::systems::ui_clutter::ui_focus_decoclutter::DecoClutterPicker>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::systems::mode_manager::RequestAppModeChange>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::systems::ui_clutter::ui_focus_decoclutter::RequestDecoClutterPicker>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::ui_clutter::ui_focus_decoclutter::LastClonedClutter>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::clutter_registry::ClutterRegistry>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::systems::ui_clutter::ui_focus_decoclutter::RequestMoveClutter>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::systems::ui_clutter::ui_focus_decoclutter::RequestRotateDecorator>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::decorator_audio::PlayDecoratorSoundCmd>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::ui_clutter::ui_focus_decoclutter::DecoClutterPicker>

// ------------------------------------------------------------------
// 函数: country_core::shader::inc::halftimber_color_customization::impl$0::descriptors<shabriri_rt::api::EmptyProtocol>
// 信息 ID: bc73cacdcef0e8ccc40d8718
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::shader::inc::halftimber_color_customization::impl$0::descriptors -> ["shabriri_rt::api::EmptyProtocol"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] shaderface::binding_set::BindingSetDesc::with<country_core::shader::inc::wooden_wall_color_customization::WoodenWallColors<shabriri_rt::api::EmptyProtocol> > | shaderface::binding_set::BindingSetDesc::with<country_core::shader::inc::halftimber_color_customization::HalftimberColors<shabriri_rt::api::EmptyProtocol> > @ 0x13ea80 -> 0x41a050 (direct_call); 同地址别名未裁定。
// [静态直接引用] shaderface::binding_set::BindingSetDesc::with<country_core::shader::inc::wooden_wall_color_customization::WoodenWallColors<shabriri_rt::api::EmptyProtocol> > | shaderface::binding_set::BindingSetDesc::with<country_core::shader::inc::halftimber_color_customization::HalftimberColors<shabriri_rt::api::EmptyProtocol> > @ 0x1a679e0 -> 0x41a050 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] unknown_source_preserved_in_module_bucket; 作者定义文件是否精确确认仍按证据状态。

// === PDB ENRICHMENT V1 END ===
