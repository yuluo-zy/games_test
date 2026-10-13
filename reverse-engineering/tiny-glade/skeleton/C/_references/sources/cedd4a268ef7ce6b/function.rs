// Generated from PDB metadata by skeleton/_tools/enrich_skeleton.py.
// Version: C
// Skeleton path: _references/sources/cedd4a268ef7ce6b/function.rs
// Original source: /home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
// 函数体、未记录参数名和布局 = unknown。

// === PDB STRUCTURES V1 BEGIN ===
// 数据结构事实按版本保留；PDB 定义 / 仅名称关系 / 原指令观察分别标注。
// 完整结构及原始类型记录: function.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: function.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: function.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 3; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::walls::wall_stone_columns::StoneColumnCollisionStorage>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallDeleted>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::walls::wall_stone_columns::StoneColumnsChanged>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::resources::walls::wall_stone_columns::StoneColumnCollisionStorage>", "bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>", "bevy_ecs::message::message_reader::MessageReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallDeleted>", "bevy_ecs::message::message_writer::MessageWriter<country_core::resources::walls::wall_stone_columns::StoneColumnsChanged>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::wall_stone_columns::StoneColumnCollisionStorage>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallDeleted>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::resources::walls::wall_stone_columns::StoneColumnsChanged>
// [文件级接口候选，具体函数归属 unknown] bool (*)(country_core::systems::collision::world::SubworldColliderHandle,ref$<enum2$<country_core::systems::collision::world::RaycastGeoSource> >)
// 候选参数类型: ["country_core::systems::collision::world::SubworldColliderHandle", "ref$<enum2$<country_core::systems::collision::world::RaycastGeoSource> >"]

// ------------------------------------------------------------------
// 函数: country_core::systems::collision::world::impl$14::iter_shape_overlaps::closure$0<tuple$<utils::IsoTransform,ref$<country_core::systems::collision::world::RaycastMeshData> >,bool (*)(country_core::systems::collision::world::SubworldColliderHandle,ref$<enum2$<country_core::systems::collision::world::RaycastGeoSource> >),system_decorator::decorator_visual::playermade_stairs::playermade_stairs_assemble_visuals::playermade_stairs_assemble::closure_env$35<true> >
// 信息 ID: 55788adf550d40f86660f7c7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [符号内函数类型候选] bool (*)(country_core::systems::collision::world::SubworldColliderHandle,ref$<enum2$<country_core::systems::collision::world::RaycastGeoSource> >)
// 参数类型拼写: ["country_core::systems::collision::world::SubworldColliderHandle", "ref$<enum2$<country_core::systems::collision::world::RaycastGeoSource> >"]
// [泛型宿主/闭包/直接引用方参数候选] 137943db23b17086ce8b9d61: bool (*)(country_core::systems::collision::world::SubworldColliderHandle,ref$<enum2$<country_core::systems::collision::world::RaycastGeoSource> >)
// [类型关系] ref$ -> ["country_core::systems::collision::world::RaycastMeshData"]; 字段布局 = unknown。
// [类型关系] tuple$ -> ["utils::IsoTransform", "ref$<country_core::systems::collision::world::RaycastMeshData>"]; 字段布局 = unknown。
// [类型关系] enum2$ -> ["country_core::systems::collision::world::RaycastGeoSource"]; 字段布局 = unknown。
// [类型关系] ref$ -> ["enum2$<country_core::systems::collision::world::RaycastGeoSource>"]; 字段布局 = unknown。
// [类型关系] system_decorator::decorator_visual::playermade_stairs::playermade_stairs_assemble_visuals::playermade_stairs_assemble::closure_env$35 -> ["true"]; 字段布局 = unknown。
// [类型关系] country_core::systems::collision::world::impl$14::iter_shape_overlaps::closure$0 -> ["tuple$<utils::IsoTransform,ref$<country_core::systems::collision::world::RaycastMeshData> >", "bool (*)(country_core::systems::collision::world::SubworldColliderHandle,ref$<enum2$<country_core::systems::collision::world::RaycastGeoSource> >)", "system_decorator::decorator_visual::playermade_stairs::playermade_stairs_assemble_visuals::playermade_stairs_assemble::closure_env$35<true>"]; 字段布局 = unknown。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::collision::world::impl$14::iter_shape_overlaps", "captures": "unknown"}
// [闭包环境类型引用] country_core::systems::collision::world::impl$14::iter_shape_overlaps::closure_env$0 <- core::ops::function::FnOnce::call_once<country_core::systems::collision::world::impl$14::iter_shape_overlaps::closure_env$0<tuple$<utils::IsoTransform,ref$<country_core::systems::collision::world::RaycastMeshData> >,bool (*)(country_core::systems::collision::world::SubworldColliderHandle,ref$<enum2$<country_core::systems::collision::world::RaycastGeoSource> >),system_decorator::decorator_visual::playermade_stairs::playermade_stairs_assemble_visuals::playermade_stairs_assemble::closure_env$35<false> >,tuple$<rapier3d::geometry::collider_components::ColliderHandle,ref$<rapier3d::geometry::collider::Collider> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::collision::world::impl$14::iter_shape_overlaps::closure_env$0 <- core::ops::function::FnOnce::call_once<country_core::systems::collision::world::impl$14::iter_shape_overlaps::closure_env$0<country_core::systems::collision::world::SubworldColliderHandle,system_decorator::decorator_visual::playermade_stairs::calculate_distortion_from_structural_graph::calculate_distortion_from_structural_graph::closure_env$2,system_decorator::decorator_visual::playermade_stairs::calculate_distortion_from_structural_graph::calculate_distortion_from_structural_graph::closure_env$3>,tuple$<rapier3d::geometry::collider_components::ColliderHandle,ref$<rapier3d::geometry::collider::Collider> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::collision::world::impl$14::iter_shape_overlaps::closure_env$0 <- core::ops::function::FnOnce::call_once<country_core::systems::collision::world::impl$14::iter_shape_overlaps::closure_env$0<country_core::systems::collision::world::SubworldColliderHandle,system_decorator::decorator_visual::playermade_stairs::calculate_distortion_from_structural_graph::calculate_distortion_from_structural_graph::closure_env$0,system_decorator::decorator_visual::playermade_stairs::calculate_distortion_from_structural_graph::calculate_distortion_from_structural_graph::closure_env$1>,tuple$<rapier3d::geometry::collider_components::ColliderHandle,ref$<rapier3d::geometry::collider::Collider> > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 3 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::shapes::circle::ui_edit_circle::show_full_circle_signifier::closure$0::closure$0
// 信息 ID: dfa7e298849ee392aa8ffcaa
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::wall::shapes::circle::ui_edit_circle::show_full_circle_signifier", "captures": "unknown"}
// [闭包环境类型引用] country_core::systems::wall::shapes::circle::ui_edit_circle::show_full_circle_signifier::closure$0::closure_env$0 <- core::ops::function::FnOnce::call_once<country_core::systems::wall::shapes::circle::ui_edit_circle::show_full_circle_signifier::closure$0::closure_env$0,tuple$<> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 1 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// === PDB ENRICHMENT V1 END ===
