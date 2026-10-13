// Generated from PDB metadata by skeleton/_tools/enrich_skeleton.py.
// Version: C
// Skeleton path: _unmapped/module-00797.rs
// Original source: 
// 函数体、未记录参数名和布局 = unknown。

// === PDB STRUCTURES V1 BEGIN ===
// 数据结构事实按版本保留；PDB 定义 / 仅名称关系 / 原指令观察分别标注。
// 完整结构及原始类型记录: module-00797.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: module-00797.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: module-00797.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 3; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(array$<f64,4>,ref_mut$<slice2$<u8> >)
// 候选参数类型: ["array$<f64,4>", "ref_mut$<slice2$<u8> >"]
// [文件级接口候选，具体函数归属 unknown] enum2$<shaderface::binding_set::BindingSetLayoutItemIdent> (*)(enum2$<shaderface::binding_set::BindingSetLayoutItemIdent>)
// 候选参数类型: ["enum2$<shaderface::binding_set::BindingSetLayoutItemIdent>"]
// [文件级接口候选，具体函数归属 unknown] alloc::sync::Arc<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global> (*)(rhapsody::raw::physical_device::PhysicalDevice)
// 候选参数类型: ["rhapsody::raw::physical_device::PhysicalDevice"]

// ------------------------------------------------------------------
// 函数: rhapsody::vertex::impl$43::source
// 信息 ID: 2a7b082526c921201739c05b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 230 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::world::World::register_component<country_core::components::drawable::NaniActiveInstance> @ 0x2db39a -> 0x104100 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::world::World::register_component<country_core::components::WoodenGate> @ 0x2dbafa -> 0x104100 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::world::World::register_component<country_core::components::FollowMouse> @ 0x2dc94a -> 0x104100 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::world::World::register_component<country_core::systems::prefab::PrefabInstanceData> @ 0x2dd0aa -> 0x104100 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::world::World::register_component<country_core::resources::critters::sheep::SheepAccessory> @ 0x2dd80a -> 0x104100 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::world::World::register_component<country_core::components::Ui2DFollowMouse> @ 0x2ddf6a -> 0x104100 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::world::World::register_component<country_core::components::Ui2dExtent> @ 0x2de6ba -> 0x104100 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::world::World::register_component<country_core::components::AutoClutter> @ 0x2dee1a -> 0x104100 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::world::World::register_component<country_core::systems::look_at_camera::LookAtCamera> @ 0x2df57a -> 0x104100 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::world::World::register_component<country_core::components::SnappingToSegmentPreview> @ 0x2dfcca -> 0x104100 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::world::World::register_component<country_core::components::SemiTransparent> @ 0x2e041a -> 0x104100 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::world::World::register_component<country_core::resources::roofs::roof_animation::RoofMarkedForDeletion> @ 0x2e0b6a -> 0x104100 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::world::World::register_component<country_core::resources::clutter_meshes::ClutterMarkedForDeletion> @ 0x2e1a1a -> 0x104100 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::world::World::register_component<country_core::systems::wall::snapping::telegraph_walls_are_merged::LineAcrossWallSignifier> @ 0x2e216a -> 0x104100 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::world::World::register_component<country_core::components::TestCube> @ 0x2e28ca -> 0x104100 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::world::World::register_component<country_core::components::DashedLineLength> @ 0x2e301a -> 0x104100 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] unknown_source_preserved_in_module_bucket; 作者定义文件是否精确确认仍按证据状态。

// === PDB ENRICHMENT V1 END ===
